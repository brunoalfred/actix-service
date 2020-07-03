use actix_web::{ HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}


pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world Again")
}

pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("This is my Room")
}

