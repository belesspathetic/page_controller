use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use shared::fb_health_check;

async fn own_health_check_handler() -> impl Responder {
    HttpResponse::Ok()
}

async fn fb_health_handler() -> impl Responder {
    let status = fb_health_check().await;

    let response = match status {
        Ok(_) => HttpResponse::Ok().finish(),           // Status 200 OK
        Err(_) => HttpResponse::BadRequest().finish(),  // Status 400 Bad Request
    };

    response
}

pub fn run(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin() // Allow requests from any source
            .allow_any_method() // Allow any HTTP method
            .allow_any_header();    // Allow any headers
        App::new()
            .wrap(cors)
            .route("/", web::get().to(own_health_check_handler))
            .route("/fb_health_check", web::get().to(fb_health_handler))
    })
    .bind(address)?
    .run();

    println!("INFO: Server start here: {}", address);
    Ok(server)
}
