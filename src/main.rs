use actix_files::Files;
use actix_web::{App, HttpServer, http::Method, web};

mod error;
mod auth;
mod upload;

const PORT: u16 = 42967;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting on port {PORT}");
    HttpServer::new(|| {
        App::new()
            .route("/upload-irc", web::method(Method::OPTIONS).to(upload::upload_options))
            .route("/upload-irc", web::method(Method::POST).to(upload::upload_irc))
            .service(Files::new("/", "./media/"))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}