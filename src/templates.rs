use actix_web::{HttpResponse, HttpResponseBuilder, http::{StatusCode, header::{self, ContentType}}};

use crate::templates::error_template::ErrorTemplate;

pub mod error_template;
pub mod index_template;
pub mod embed_template;

pub trait TemplateExtraTrait {
    /// Renders either the template, or an error page
    fn render_safe(self) -> String;

    /// Renders the template and builds an Actix response
    fn render_response(self) -> HttpResponse;
}
impl<T: askama::Template> TemplateExtraTrait for T {
    fn render_safe(self) -> String {
        match self.render() {
            Ok(text) => text,
            Err(err) => {
                ErrorTemplate {
                    name: "Failed to render askama template".to_string(),
                    text: err.to_string()
                }.render()
            }
        }
    }
    fn render_response(self) -> HttpResponse {
        let (status, text) = match self.render() {
            Ok(text) => (StatusCode::OK, text),
            Err(err) => {
                let template = ErrorTemplate {
                    name: "Failed to render askama template".to_string(),
                    text: err.to_string()
                };
                (StatusCode::OK, template.render())
            }
        };
        HttpResponseBuilder::new(status)
            .insert_header((header::CONTENT_TYPE, ContentType::html()))
            .body(text)
    }
}
