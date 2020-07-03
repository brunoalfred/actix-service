use actix_web::{web, App, HttpServer, };
mod handlers;





#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .route("/", web::get().to(handlers::req_handlers::index))
        .route("/again", web::get().to(handlers::req_handlers::index2))
        .route("/inside", web::get().to(handlers::req_handlers::index3))
        
        
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
} 