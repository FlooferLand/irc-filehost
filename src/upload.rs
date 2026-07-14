use actix_web::{HttpRequest, HttpResponse, Responder, http::header, web};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use tokio::{fs, io::AsyncWriteExt};

use crate::{auth::{check_authorize, get_user_id}, error::{MapServerErrorTrait, ServerError}, metadata::Metadata};
use futures::StreamExt;

/// IRC fileserver information
pub async fn upload_options() -> impl Responder {
    HttpResponse::NoContent()
        .insert_header(("Allow", "OPTIONS, POST"))
        .insert_header(("Accept-Post", "image/*, video/*"))
        .finish()
}

// IRC upload
pub async fn upload_irc(req: HttpRequest, mut payload: web::Payload) -> Result<HttpResponse, ServerError> {
    if !check_authorize(&req).await {
        return Err(ServerError::Unauthorized);
    }
    let user = get_user_id(&req).unwrap_or("Unknown".to_string());

    // Getting file info
    let mut name = "".to_string();
    let mut ext = "".to_string();
    if let Some(Ok(str)) = req.headers().get(header::CONTENT_DISPOSITION).map(|h| h.to_str()) {
        let content = content_disposition::parse_content_disposition(str);
        if let Some(filename) = content.filename() {
            name = filename.0;
            if let Some(path) = filename.1 { ext = path.to_string().to_ascii_lowercase() }
        }
    }
    println!("User '{user}' is uploading '{Filename}'", Filename = if !ext.is_empty() { format!("{name}.{ext}") } else { name.clone() });
    
    // Writing the file
    let temp_path = format!("./media/{}.tmp", hex::encode(rand::random::<[u8; 8]>()));
    let mut file = fs::File::create(&temp_path).await.server_err()?;
    let mut hasher = Sha256::new();
    while let Some(chunk) = payload.next().await {
        let bytes = chunk.server_err()?;
        hasher.update(&bytes);
        file.write_all(&bytes).await.server_err()?
    }
    file.flush().await.server_err()?;
    drop(file);

    // Hashing
    let hash = URL_SAFE_NO_PAD.encode(hasher.finalize())[..12].to_string();

    // Updating the extension if there isn't one
    if ext.is_empty() {
        if let Ok(bytes) = fs::read(&temp_path).await.server_err() {
            ext = infer::get(&bytes).map(|t| t.extension().to_string()).unwrap_or_else(|| "".to_string());
        }
    }

    // Naming the file
    let final_name = if !ext.is_empty() { format!("{hash}.{ext}") } else { hash.clone() };
    let final_path = format!("./media/{}", final_name);
    fs::rename(&temp_path, &final_path).await.server_err()?;

    // Creating a metadata file
    let size_bytes = fs::metadata(&final_path).await.map(|v| v.len()).unwrap_or(0);
    let meta_name = format!("{hash}.meta.toml");
    let meta_path = format!("./media/{}", meta_name);
    let meta = Metadata {
        creation: Utc::now(),
        size_bytes, ext, name
    };
    match toml::to_string(&meta) {
        Ok(str) => {
            if let Err(err) = fs::write(meta_path, str).await {
                return Err(ServerError::InternalError { info: format!("Failed to write metadata file: {err}") })
            }
        },
        Err(err) => {
            return Err(ServerError::InternalError { info: format!("Failed to serialize metadata file: {err}") })
        },
    };

    // Grabbing and finalizing stuff
    let size_text = ByteSize::b(size_bytes).display().si().to_string();
    let connection = req.connection_info();
    let location = format!("{}://{}/{}", connection.scheme(), connection.host(), hash);
    println!("User '{user}' has uploaded '{final_name}' ({size_text})\n");
    Ok(HttpResponse::Created()
        .insert_header((header::LOCATION, location))
        .finish()
    )
}
