use actix_web::{get, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to home Page")
}
