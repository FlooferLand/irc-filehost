use mime_guess::Mime;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Content {
    Audio(Audio),
    Video(Video),
    Image(Image),
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Audio {
    pub url: String,
    pub mime_type: String,
    pub duration_secs: u16,
    pub album: Option<String>,
    pub musician: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Video {
    pub url: String,
    pub mime_type: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub mime_type: String,
    pub alt: String
}
impl Image {
    pub fn new(url: &str, alt: &str, mime_type: &Mime) -> Self {
        Self {
            url: url.to_owned(),
            alt: alt.to_owned(),
            mime_type: mime_type.to_string()
        }
    }
}
