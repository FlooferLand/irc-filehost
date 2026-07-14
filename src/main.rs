use actix_web::{App, HttpServer, http::Method, web};

mod error;
mod auth;
mod upload;
mod serve;
mod metadata;

const PORT: u16 = 42967;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting on port {PORT}");
    HttpServer::new(|| {
        App::new()
            .route("/upload-irc", web::method(Method::OPTIONS).to(upload::upload_options))
            .route("/upload-irc", web::method(Method::POST).to(upload::upload_irc))
            .route("/{id}", web::method(Method::GET).to(serve::serve))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}