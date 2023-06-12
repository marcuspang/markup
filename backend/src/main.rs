mod models;
mod repository;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

use crate::repository::config::ServerConfig;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Server running")
}

#[post("/grade")]
async fn grade(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/mark")]
async fn mark(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Response not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = ServerConfig::from_env().unwrap();

    let db = repository::db::Database::new(config.DATABASE_URL);
    let app_data = web::Data::new(db);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(hello)
            .service(grade)
            .service(mark)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((config.SERVER_HOST.clone(), config.SERVER_PORT))?
    .run();

    println!(
        "Server running at {}:{}",
        config.SERVER_HOST.clone(),
        config.SERVER_PORT
    );

    server.await
}
