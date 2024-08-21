use std::{collections::HashMap, env::current_dir, fs::File, io::Write, thread::sleep, time::Duration};
use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use fb_poster::{utils::Secrets, video::Video};
use markdown::to_html;
use montage::{combine, scale};
use shared::models::Status;
use pytube::{download_audio, download_video, get_thumb_url, get_title};
use shared::{fb_get_me, fb_health_check, models::ManualData};
use tx::gen_text;
use utils::{create_new_dir, create_status_file, is_directory_present, is_status_file_exist, read_env, read_status, remove_status_file, update_status_file};
pub mod process_table;
mod utils;
mod pytube;
mod montage;
mod tx;

async fn own_health_check_handler() -> impl Responder {
    HttpResponse::Ok()
}

async fn fb_health_handler() -> impl Responder {
    let status = fb_health_check().await;

    let response = match status {
        Ok(_) => HttpResponse::Ok().finish(),          // Status 200 OK
        Err(_) => HttpResponse::BadRequest().finish(), // Status 400 Bad Request
    };

    response
}

async fn patchnote() -> impl Responder {
    let markdown_content = std::fs::read_to_string("patchnote.md")
        .expect("Unable to read file");
    let html_content = to_html(&markdown_content);

    HttpResponse::Ok().body(html_content)
}

async fn get_me(key: web::Query<HashMap<String, String>>) -> impl Responder {
    let key = key.get("key").unwrap();
    let resp = fb_get_me(key.clone()).await.unwrap();

    

    HttpResponse::Ok().json(resp)
}

// Обработчик для запуска процесса

async fn manual_upload(data: web::Json<ManualData>) -> impl Responder {
    let key = data.key.clone().unwrap_or_default();


    tokio::spawn(async move {
        let resp = fb_get_me(key.clone()).await.unwrap();
        let id = resp.id;

        if !is_directory_present(&id) {
            let _ = create_new_dir(&id);
        }

        if !is_status_file_exist(&id) {
            create_status_file(&id);
        }

        let temp_dir_path = current_dir().unwrap().join(&id).join("temp");
        let temp_dir_path_str = temp_dir_path.to_string_lossy().to_string();

        // Обновляем статус на "Downloading"
        update_status_file(&id, Status::Downloading);
        

        let result = download_video(&data.url.clone().unwrap(), temp_dir_path_str.to_string()).await.unwrap();

        if result == "None" {
            println!("PROCESS: Video is unavailable... Skipping...");
            // Обновляем статус на "Waiting"
            update_status_file(&id, Status::Waiting);
            return;
        }

        let _ = download_audio(&data.url.clone().unwrap(), temp_dir_path_str.to_string()).await;
        let thumb_url = get_thumb_url(&data.url.clone().unwrap(), temp_dir_path_str.to_string()).await.unwrap();

        let vars = read_env();
        let thumb_path = temp_dir_path.join("thumb.png");
        let thumb_path_str = thumb_path.to_string_lossy().to_string();
        let mut thumb_file = File::create(&thumb_path).unwrap();
        let cl = reqwest::Client::new();

        println!("PROCESS: Downloading thumb...");
        let resp = cl.get(thumb_url).send().await.unwrap().bytes().await.unwrap();
        thumb_file.write_all(&resp).unwrap();
        println!("PROCESS: Downloading done...");

        let video_path = temp_dir_path.join("video_stream.mp4");
        let audio_path = temp_dir_path.join("audio_stream.mp4");

        // Status montage
        update_status_file(&id, Status::Montage);

        let output = combine(
            video_path,
            audio_path,
            &temp_dir_path,
            vars.ffmpeg.clone()
        )
        .await.unwrap();

        let title = if data.title.clone().unwrap() == "" {
            format!("{} {}", get_title(&data.url.clone().unwrap()).await.unwrap(), &data.tags.clone().unwrap())
        } else {
            format!("{} {}", data.title.clone().unwrap(), &data.tags.clone().unwrap())
        };

        if data.montage_or_no_montage.clone().unwrap() == "montage" {
            let (top_text, bottom_text, font_size) = gen_text(&title.trim().to_string());

            let output = scale(
                output,
                &temp_dir_path,
                top_text,
                bottom_text,
                font_size,
                vars.ffmpeg.clone()
            )
            .await.unwrap();
            let path = output.to_string_lossy().to_string();
            println!("Uploading...");

            // Status Uploading
            update_status_file(&id, Status::Uploading);
            

            let secrets = Secrets::new(&key, &id);
            if let Err(e) = Video::new(secrets.clone())
                    .local_video(path)
                    .with_title(title.clone())
                    .with_description(title)
                    .with_thumbnail(thumb_path_str)
                    .send()
                    .await {
                eprintln!("Failed to send video: {}", e);
            }
        } else {
            // Status Uploading
            update_status_file(&id, Status::Uploading);
            let path = output.to_string_lossy().to_string();
            println!("Uploading...");

            let secrets = Secrets::new(&key, &id);
            if let Err(e) = Video::new(secrets.clone())
                    .local_video(path)
                    .with_title(title.clone())
                    .with_description(title)
                    .with_thumbnail(thumb_path_str)
                    .send()
                    .await {
                eprintln!("Failed to send video: {}", e);
            }
        }

        // Status Success
        update_status_file(&id, Status::Success);

        sleep(Duration::from_secs(10));

        remove_status_file(&id);
    });

    HttpResponse::Ok().json("Process started")
}



async fn get_status(
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let key = match query.get("key") {
        Some(k) => k.clone(),
        None => return HttpResponse::BadRequest().body("Missing 'key' parameter"),
    };

    let resp = fb_get_me(key.clone()).await.unwrap();
    let id = resp.id;

    let path = current_dir().unwrap().join(id).join("status.txt");
    let path_str = path.to_string_lossy().to_string();


    match read_status(path_str) {
        Ok(st) => {
            println!("Returning status: {:?}", st);
            HttpResponse::Ok().json(st)
        },
        Err(_) => {
            HttpResponse::Ok().json("Waiting".to_string())
        }
    }

}

pub fn run(address: &str) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Allow requests from any source
            .allow_any_method() // Allow any HTTP method
            .allow_any_header(); // Allow any headers
        App::new()
            .wrap(cors)
            .route("/", web::get().to(own_health_check_handler))
            .route("/fb_health_check", web::get().to(fb_health_handler))
            .route("/patchnote", web::get().to(patchnote))
            .route("/get_me", web::get().to(get_me))
            .route("/manual_upload", web::post().to(manual_upload))
            .route("/status", web::get().to(get_status))
    })
    .bind(address)?
    .run();

    println!("INFO: Server start here: {}", address);
    Ok(server)
}
