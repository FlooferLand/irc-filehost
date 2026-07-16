#![allow(dead_code)]
use actix_files::Files;
use actix_web::{App, HttpServer, Responder, http::Method, web};
use crate::templates::{TemplateExtraTrait, index_template::IndexTemplate};

mod error;
mod auth;
mod upload;
mod serve;
mod metadata;
mod templates;
mod opengraph;
mod extensions;

const PORT: u16 = 42967;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting on port {PORT}");
    HttpServer::new(|| {
        App::new()
            .route("/", web::method(Method::GET).to(index))
            .route("/upload-irc", web::method(Method::OPTIONS).to(upload::upload_options))
            .route("/upload-irc", web::method(Method::POST).to(upload::upload_irc))
            .service(Files::new("/static", "./static").prefer_utf8(true).show_files_listing())
            .route("/{id}", web::method(Method::GET).to(serve::serve_default))
            .route("/{id}/preview", web::method(Method::GET).to(serve::serve_preview))
            .route("/{id}/raw", web::method(Method::GET).to(serve::serve_raw))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}

async fn index() -> impl Responder {
    IndexTemplate {}.render_response()
}
