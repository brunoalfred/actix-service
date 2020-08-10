use actix_web::{get, HttpResponse, Responder};


pub async fn index4() -> impl Responder {
    HttpResponse::Ok().body("Register Page")
}