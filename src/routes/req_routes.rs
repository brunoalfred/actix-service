use actix_web::{get, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to home Page")
}

pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("About Page")
}

pub async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Login Page")
}

pub async fn index4() -> impl Responder {
    HttpResponse::Ok().body("Register Page")
}

#[get("/hello")]
async fn index5() -> impl Responder {
    HttpResponse::Ok().body("      Hello ")
}
