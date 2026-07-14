use std::fmt::Display;

use actix_web::{HttpResponse, Responder, http::header::ContentType};


const BASE_HTML: &str = include_str!("../data/base.html");

pub struct HtmlTemplate {
    title: String,
    body: Vec<String>
}
impl Default for HtmlTemplate {
    fn default() -> Self {
        Self { title: "".to_string(), body: Default::default() }
    }
}
impl HtmlTemplate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn body_line(mut self, s: impl Display) -> Self {
        self.body.push(s.to_string());
        self
    }

    pub fn build(self) -> String {
        let body = self.body.join("\n");
        BASE_HTML
            .replace("{{title}}", &self.title)
            .replace("{{body}}", &body)
    }
    pub fn build_response(self) -> impl Responder {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(self.build())
    }
}
