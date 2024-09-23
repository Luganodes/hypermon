use actix_web::{http::StatusCode, ResponseError};

#[derive(Debug, thiserror::Error)]
pub enum HypermonError {
    #[error("Error while getting response: {0}")]
    ResponseError(#[from] anyhow::Error),

    #[error("Deserialization Error: {0}")]
    DeserializationError(#[source] anyhow::Error),

    #[error("Something went wrong internall")]
    InternalServerError,

    #[error("Register error")]
    RegisterError(#[source] anyhow::Error),
    
    #[error("Error while encoding metric families")]
    EncodeError(#[source] anyhow::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

impl ResponseError for HypermonError {
    fn status_code(&self) -> StatusCode {
        match self {
            HypermonError::ResponseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HypermonError::DeserializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HypermonError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            HypermonError::RegisterError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HypermonError::EncodeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HypermonError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
