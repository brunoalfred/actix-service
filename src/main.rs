
use actix_web::{web, App, HttpServer,middleware, };
mod handlers;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .wrap(middleware::Compress::default())
        .route("/", web::get().to(handlers::req_handlers::index))
        .route("/about", web::get().to(handlers::req_handlers::index2))
        .route("/login", web::get().to(handlers::req_handlers::index3))
        .route("/register", web::get().to(handlers::req_handlers::index4))

        .service(handlers::req_handlers::index4)
        
        
    })  
    .bind("127.0.0.1:8088")?
    .run()
    .await
} 