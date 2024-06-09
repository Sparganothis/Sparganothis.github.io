use http::status::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum WebAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl WebAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            WebAppError::NotFound => StatusCode::NOT_FOUND,
            WebAppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
