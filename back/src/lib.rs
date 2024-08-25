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
use utils::{ensure_directory_structure, read_env, read_status, remove_status_file, update_status_file};
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



async fn download_thumbnail(thumb_url: &str, temp_dir_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let thumb_path = temp_dir_path.join("thumb.png");
    let mut thumb_file = File::create(&thumb_path)?;
    let client = reqwest::Client::new();
    let resp = client.get(thumb_url).send().await?.bytes().await?;
    thumb_file.write_all(&resp)?;
    Ok(())
}

async fn process_montage(output: std::path::PathBuf, temp_dir_path: &std::path::Path, montage_or_no_montage: &str, title: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    if montage_or_no_montage == "montage" {
        let vars = read_env();
        let vars = vars.ffmpeg;
        let (top_text, bottom_text, font_size) = gen_text(&title.trim().to_string());
        let output = scale(output, temp_dir_path, top_text, bottom_text, font_size, vars).await?;
        Ok(output)
    } else {
        Ok(output)
    }
}

async fn process_upload(key: &String, url: &String, title: &String, tags: &String, montage_or_no_montage: &String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = fb_get_me(key.clone()).await.unwrap();
        let id = resp.id;

        let _ = ensure_directory_structure(&id).await;

        let temp_dir_path = current_dir().unwrap().join(&id).join("temp");
        let temp_dir_path_str = temp_dir_path.to_string_lossy().to_string();

        update_status_file(&id, Status::Downloading);
        
        let result = download_video(&url, temp_dir_path_str.clone()).await.unwrap();

        if result == "None" {
            println!("PROCESS: Video is unavailable... Skipping...");
            update_status_file(&id, Status::Waiting);
            return Ok(());
        }

        let _ = download_audio(&url, temp_dir_path_str.clone()).await;
        let thumb_url = get_thumb_url(&url, temp_dir_path_str).await.unwrap();

        let thumb_path = temp_dir_path.join("thumb.png");
        let thumb_path_str = thumb_path.to_string_lossy().to_string();
        let _ = download_thumbnail(&thumb_url, &temp_dir_path).await;

        let video_path = temp_dir_path.join("video_stream.mp4");
        let audio_path = temp_dir_path.join("audio_stream.mp4");
        // Status montage
        update_status_file(&id, Status::Montage);

        let output = combine(
            video_path,
            audio_path,
            &temp_dir_path,
        )
        .await.unwrap();

        let title = if title.is_empty() {
            format!("{} {}", get_title(&url).await.unwrap(), tags)
        } else {
            format!("{} {}", title, tags)
        };

        let output = process_montage(output, &temp_dir_path, montage_or_no_montage.as_str(), title.as_str()).await;
        let path = output.unwrap().to_string_lossy().to_string();
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

        // Status Success
        update_status_file(&id, Status::Success);

        sleep(Duration::from_secs(10));

        remove_status_file(&id);

        Ok(())
}

async fn manual_upload(data: web::Json<ManualData>) -> impl Responder {
    let key = data.key.clone().unwrap_or_default();
    let url = data.url.clone().unwrap();
    let title = data.title.clone().unwrap_or_default();
    let tags = data.tags.clone().unwrap_or_default();
    let montage_or_no_montage = data.montage_or_no_montage.clone().unwrap_or_default();

    tokio::spawn(async move {
        if let Err(e) = process_upload(&key, &url, &title, &tags, &montage_or_no_montage).await {
            eprintln!("Error processing upload: {}", e);
        }
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
