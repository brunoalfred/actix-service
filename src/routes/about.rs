
use actix_web::{get, HttpResponse, Responder};

pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("About Page")
}