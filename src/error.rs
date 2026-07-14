use actix_web::{HttpResponse, error, http::{StatusCode, header::ContentType}};
use derive_more::derive::{Display, Error};

pub trait MapServerErrorTrait<T> {
    fn server_err(self) -> Result<T, ServerError>;
}

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display("Unauthorized")]
    Unauthorized,

    #[display("Internal Error")]
    InternalError { info: String }
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let body = match self {
            Self::InternalError { info } => format!("{} {}", self.to_string(), info),
            _ => self.to_string()  
        };
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(body)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// IO error
impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self::InternalError { info: value.to_string() }
    }
}
impl From<actix_web::error::PayloadError> for ServerError {
    fn from(value: actix_web::error::PayloadError) -> Self {
        Self::InternalError { info: value.to_string() }
    }
}
impl<V, E> MapServerErrorTrait<V> for Result<V, E> where E: Into<ServerError> {
    fn server_err(self) -> Result<V, ServerError> {
        self.map_err(|e| e.into())
    }
}
