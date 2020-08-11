use actix_web::{ HttpResponse, Responder};


pub async fn index4() -> impl Responder {
    HttpResponse::Ok().body("Register Page")
}