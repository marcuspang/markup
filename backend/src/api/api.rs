use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Server running")
}

#[derive(Deserialize, Debug)]
struct GradeRequest {
    question: String,
    answer: String,
    marking_scheme: String,
}

#[post("/grade")]
async fn grade(req_body: web::Json<GradeRequest>) -> impl Responder {
    println!("{:#?}", req_body);
    HttpResponse::Ok().body("Success")
}

#[derive(Deserialize, Debug)]
struct GenerateRequest {
    questions: Vec<String>,
    answers: Vec<String>,
}

#[post("/generate")]
async fn generate(req_body: web::Json<GenerateRequest>) -> impl Responder {
    println!("{:#?}", req_body);
    HttpResponse::Ok().body("Success")
}

pub fn create_api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(hello)
            .service(grade)
            .service(generate),
    );
}
