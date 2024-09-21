use actix_web::{http::StatusCode, ResponseError};

#[derive(Debug, thiserror::Error)]
pub enum HypermonError {
    #[error("Error while getting response: {0}")]
    ResponseError(#[from] anyhow::Error),

    #[error("Deserialization Error: {0}")]
    DeserializationError(#[source] anyhow::Error),
}

impl ResponseError for HypermonError {
    fn status_code(&self) -> StatusCode {
        match self {
            HypermonError::ResponseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HypermonError::DeserializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
