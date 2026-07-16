use std::ops::Deref;
use actix_web::http::header::{AsHeaderName, HeaderMap};

pub trait HeaderMapExtraTrait {
    fn get_string(&self, key: impl AsHeaderName) -> Option<String>;
}
impl HeaderMapExtraTrait for HeaderMap {
    fn get_string(&self, key: impl AsHeaderName) -> Option<String> {
        let Some(value) = self.get(key) else { return None };
        let str = value.to_str()
            .map(|str| str.to_string())
            .unwrap_or_else(|_| String::from_utf8_lossy(value.as_bytes()).deref().to_string());
        Some(str.to_string())
    }
}

pub trait StringExtraTrait {
    fn with_ext(&self, ext: &str) -> Self;
}
impl StringExtraTrait for String {
    fn with_ext(&self, ext: &str) -> String {
        if !ext.is_empty() { format!("{self}.{ext}") } else { self.clone() }
    }
}

