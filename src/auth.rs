use actix_web::{HttpRequest, http::header};
use tokio::fs;

use crate::extensions::HeaderMapExtraTrait;

const USER_FILE: &str = "./data/users";

// NOTE: Auth is just username:password but base64 encoded
pub async fn check_authorize(req: &HttpRequest) -> bool {
    let Some(auth) = req.headers().get_string(header::AUTHORIZATION).map(|str| str.replace("Basic ", "")) else { return false };
    let Ok(file) = fs::read_to_string(USER_FILE).await else {
        println!("You need to add users to '{USER_FILE}' for uploads to work. (Uploader token is '{auth}')");
        return false
    };
    let whitelisted = file.lines().any(|f| auth == f.trim());
    if !whitelisted {
        println!("User with token '{auth}' tried uploading a file, but their token isn't whitelisted in '{USER_FILE}'");
    }
    whitelisted
}

pub fn get_user_id(req: &HttpRequest) -> Option<String> {
    let info = req.connection_info();
    let peer = info.realip_remote_addr();
    peer.map(|v| v.to_string())
}
