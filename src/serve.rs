use std::path::Path;

use actix_web::{HttpRequest, HttpResponse, web};
use tokio::fs;

use crate::{error::{MapServerErrorTrait, ServerError}, metadata::Metadata};

/// Should support both providing the file extension and not
pub async fn serve(req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse, ServerError> {
    let id = path.into_inner();
    let hash = id.split('.').collect::<Vec<&str>>().first().unwrap().to_string();

    // Protecting against path traversal attacks
    let base_dir = Path::new("./media").canonicalize().server_err()?;
    let meta_path = base_dir.join(format!("{hash}.meta.toml"));
    if !meta_path.starts_with(&base_dir) || !meta_path.exists() {
        return Err(ServerError::InternalError { info: format!("Broken path at '{}'", meta_path.display()) })
    }

    // Reading the metadata
    let meta_text = fs::read_to_string(meta_path).await.server_err()?;
    let meta = toml::from_str::<Metadata>(&meta_text).server_err()?;

    let file_name = if !meta.ext.is_empty() { format!("{hash}.{}", meta.ext) } else { hash.clone() };
    let file_path = base_dir.join(file_name);
    let file = fs::File::open(&file_path).await.server_err()?;
    let stream = tokio_util::io::ReaderStream::new(file);
    let mime = mime_guess::from_ext(&meta.ext).first_or_else(|| mime_guess::from_path(&file_path).first_or_octet_stream());

    Ok(
        HttpResponse::Ok()
            .content_type(mime.as_ref())
            .streaming(stream)
    )
}
