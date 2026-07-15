use actix_web::{App, HttpServer, Responder, http::Method, web};

use crate::template::HtmlTemplate;

mod error;
mod auth;
mod upload;
mod serve;
mod metadata;
mod template;

const PORT: u16 = 42967;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting on port {PORT}");
    HttpServer::new(|| {
        App::new()
            .route("/", web::method(Method::GET).to(index))
            .route("/upload-irc", web::method(Method::OPTIONS).to(upload::upload_options))
            .route("/upload-irc", web::method(Method::POST).to(upload::upload_irc))
            .route("/{id}", web::method(Method::GET).to(serve::serve))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}

async fn index() -> impl Responder {
    let silly = "flooferlands-file-host-that-is-used-for-irc-and-maybe-other-things-in-the-future";
    HtmlTemplate::new()
        .body_line("<p>Welcome to</p>")
        .body_line(format!("<a href=\"https://github.com/flooferland/irc-filehost\">{silly}</a>"))
        .body_line("<p>I'd add a directory listing here but that would be a security vulnerability!</p>")
        .body_line("<img src=\"https://avatars.githubusercontent.com/u/76737186\" alt=\"silly\" />")
        .build_response()
}
