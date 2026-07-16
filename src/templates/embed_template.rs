use askama::Template;
use mime_guess::Mime;

use crate::{extensions::StringExtraTrait, metadata::Metadata, opengraph::{Audio, Content, Image, Video}};

#[derive(Template)]
#[template(path = "embed.html")]
pub struct EmbedTemplate {
    pub hash: String,
    pub meta: Metadata,
    pub title: String,
    pub description: String,
    pub opengraph_type: String,
    pub raw_url: String,
    pub icon_url: String,
    pub cover_image: Image,
    pub content: Content
}
impl EmbedTemplate {
    pub fn from_file(url: String, hash: String, meta: Metadata, mime_type: Mime) -> Self {
        let filename = meta.name.with_ext(&meta.ext);

        let mut template = Self {
            hash: hash.to_owned(),
            meta: meta.to_owned(),
            raw_url: url.to_owned(),
            title: filename.to_owned(),
            description: filename.to_owned(),
            opengraph_type: "website".to_owned(),
            icon_url: "/static/icons/question_icon.png".to_owned(),
            cover_image: Image::new("/static/empty_card.png", &meta.name),
            content: Content::Unknown
        };
        match mime_type.type_().as_str() {
            "audio" => {
                template.opengraph_type = "music.song".to_owned();
                template.icon_url = "/static/icons/audio_icon.png".to_owned();
                template.content = Content::Audio(Audio {
                    url: url,
                    album: None,
                    duration_secs: 100,
                    musician: "".to_owned(),
                    mime_type: mime_type.to_string()
                })
            }
            "video" => {
                template.opengraph_type = "video.other".to_owned();
                template.icon_url = "/static/icons/video_icon.png".to_owned();
                template.content = Content::Video(Video {
                    url: url,
                    mime_type: mime_type.to_string()
                })
            }
            "image" => {
                let image = Image::new(&url, &meta.name);
                template.cover_image = image.clone();
                template.content = Content::Image(image);
            }
            _ => {}
        }
        template
    }

    pub fn download_name(&self) -> String {
        format!("{} [{}]", self.meta.name, self.hash)
            .with_ext(&self.meta.ext)
    }
}
