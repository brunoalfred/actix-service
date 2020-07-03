use actix_web::{get, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}


pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world Again")
}

pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("This is my Room")
}

#[get("/hello")]
async fn index4() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}