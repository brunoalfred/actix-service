use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
async fn index5() -> impl Responder {
    HttpResponse::Ok().body("Hello ")
}
