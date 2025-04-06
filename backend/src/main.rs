use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, App, HttpServer, Responder, HttpResponse};

mod api;
mod handlers;
mod db;

#[get("/api")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("🚀 Cryptocurrency News Aggregator is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at: http://127.0.0.1:8080");

    HttpServer::new(|| {
        let cors = Cors::permissive(); // Разрешить всё (для dev)

        App::new()
            .wrap(cors)
            .service(index)
            .service(handlers::get_crypto_data)
            .service(handlers::get_crypto_news)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
