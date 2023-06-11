mod config;
mod db;
mod errors;
mod models;

use actix_web::{get, post, web::Data, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::config::ServerConfig;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = ServerConfig::from_env().unwrap();

    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(hello)
            .service(grade)
            .service(mark)
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
