mod api;
mod models;
mod repository;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

use crate::{api::api::create_api_config, repository::config::ServerConfig};

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("Response not found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = ServerConfig::from_env().unwrap();

    let db = repository::db::Database::new(config.database_url);
    let app_data = web::Data::new(db);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(create_api_config)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((config.server_host.clone(), config.server_port))?
    .run();

    println!(
        "Server running at {}:{}",
        config.server_host.clone(),
        config.server_port
    );

    server.await
}
