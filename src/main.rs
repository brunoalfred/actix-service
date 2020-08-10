use actix_web::{middleware, web, App, HttpServer};
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(routes::req_routes::index))
            .route("/about", web::get().to(routes::req_routes::index2))
            .route("/login", web::get().to(routes::req_routes::index3))
            .route("/register", web::get().to(routes::req_routes::index4))
            .service(routes::req_routes::index5)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
