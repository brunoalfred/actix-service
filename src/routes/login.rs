use actix_web::{get, HttpResponse, Responder};


pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Login Page")
}