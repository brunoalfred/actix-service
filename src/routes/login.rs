use actix_web::{HttpResponse, Responder};


pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Login Page")
}