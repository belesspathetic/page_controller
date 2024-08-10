use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use markdown::to_html;
use shared::{fb_get_me, fb_health_check};

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

pub fn run(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
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
    })
    .bind(address)?
    .run();

    println!("INFO: Server start here: {}", address);
    Ok(server)
}
