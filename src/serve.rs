use std::path::Path;

use actix_web::{HttpRequest, HttpResponse, http::header::{self, ContentType}, web};
use regex::regex;
use tokio::fs;

use crate::{error::{MapServerErrorTrait, ServerError}, extensions::{HeaderMapExtraTrait, StringExtraTrait}, metadata::Metadata, templates::{TemplateExtraTrait, embed_template::EmbedTemplate}};

/// Supports both providing the file extension and not
async fn serve(req: HttpRequest, path: web::Path<String>, serve_type: ServeType) -> Result<HttpResponse, ServerError> {
    let url = req.full_url().to_string().replace("/preview", "");
    let raw_url = format!("{url}/raw");
    let id = path.into_inner();
    let id_split = id.split('.').collect::<Vec<&str>>();
    let Some(hash) = id_split.get(0).map(|str| str.to_string()) else {
        return Err(ServerError::InternalError { info: "Missing hash".to_owned() })
    };

    // Protecting against path traversal attacks
    let base_dir = Path::new("./media").canonicalize().server_err()?;
    let meta_path = base_dir.join(format!("{hash}.meta.toml"));
    if !meta_path.starts_with(&base_dir) {
        return Err(ServerError::InternalError { info: format!("Broken path at '{}'", meta_path.display()) })
    }
    if !meta_path.try_exists().unwrap_or(false) {
        return Err(ServerError::InternalError { info: format!("No file exists here! Shoo! (hash={hash})") })
    }

    // Reading the metadata file
    let meta_text = fs::read_to_string(meta_path).await.server_err()?;
    let meta = toml::from_str::<Metadata>(&meta_text).server_err()?;

    let file_name = hash.with_ext(&meta.ext);
    let file_path = base_dir.join(file_name);
    let file = fs::File::open(&file_path).await.server_err()?;
    let stream = tokio_util::io::ReaderStream::new(file);
    let mime = mime_guess::from_ext(&meta.ext).first_or_else(|| mime_guess::from_path(&file_path).first_or_octet_stream());

    // Chosing a response
    let is_raw: bool = match serve_type {
        ServeType::Default => {
            let user_agent = req.headers().get_string(header::USER_AGENT).unwrap_or("default".to_owned());
            let accepts = req.headers().get_string(header::ACCEPT).unwrap_or("*/*".to_owned())
                .split(',')
                .map(|s| s.split_once(';').map(|(main,_)| main).unwrap_or(s))
                .map(|s| s.split_once('+').map(|(main,_)| main).unwrap_or(s))
                .map(|s| s.trim().to_lowercase())
                .filter(|s| s.len() > 2)
                .collect::<Vec<String>>();

            // Doesn't accept */* on purpose
            let mime_supported =
                accepts.iter().any(|accepted| accepted.eq_ignore_ascii_case(&mime.to_string()));
            
            // Show the raw file if
            let mime_type = mime.type_().to_string().to_ascii_lowercase();
            //println!("'{user_agent}' accepts: {accepts:#?} (content type is '{mime_type}')" );
            match user_agent.to_ascii_lowercase().as_str() {
                s if regex!("discordbot").is_match(s) =>
                    ["video", "image"].contains(&mime_type.as_str()),
                s if regex!("whatsapp|halloy").is_match(s) =>
                    ["image"].contains(&mime_type.as_str()),
                s => mime_supported && !regex!("mozilla|firefox|chrome|chromium").is_match(s)
            }
        },
        ServeType::Preview => false,
        ServeType::Raw => true
    };
    let response = match is_raw {
        true =>
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .streaming(stream),
        false => {
            let embed = EmbedTemplate::from_file(raw_url.clone(), hash, meta, mime);
            let render = embed.render_safe();
            // println!("Made embed for '{raw_url}': {render}");
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(render)
        }
    };
    Ok(response)
}

enum ServeType {
    Default,
    Preview,
    Raw
}
pub async fn serve_default(req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse, ServerError> {
    serve(req, path, ServeType::Default).await
}
pub async fn serve_preview(req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse, ServerError> {
    serve(req, path, ServeType::Preview).await
}
pub async fn serve_raw(req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse, ServerError> {
    serve(req, path, ServeType::Raw).await
}
