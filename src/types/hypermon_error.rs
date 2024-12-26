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

    #[error("RPC Client Error: {0}")]
    RpcClientError(#[source] anyhow::Error),

    #[error("Validator is jailed or not found: {0}")]
    ValidatorJailedOrNotFound(String),

    #[error("Couldn't unwrap SyncInfo")]
    UnableToUnwrapSyncInfo,
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
            HypermonError::RpcClientError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            HypermonError::UnableToUnwrapSyncInfo => StatusCode::INTERNAL_SERVER_ERROR,
            HypermonError::ValidatorJailedOrNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<web3::Error> for HypermonError {
    fn from(value: web3::Error) -> Self {
        match value {
            web3::Error::Unreachable => {
                HypermonError::RpcClientError(anyhow::anyhow!("RPC is unreachable!"))
            }
            web3::Error::Decoder(s) => {
                HypermonError::RpcClientError(anyhow::anyhow!("Decoding error: {s:?}"))
            }
            web3::Error::InvalidResponse(s) => {
                HypermonError::RpcClientError(anyhow::anyhow!("Invalid response from RPC: {s:?}"))
            }
            web3::Error::Transport(s) => {
                HypermonError::RpcClientError(anyhow::anyhow!("RPC Transport Error: {s:?}"))
            }
            web3::Error::Rpc(s) => {
                HypermonError::RpcClientError(anyhow::anyhow!("RPC Error: {s:?}"))
            }
            web3::Error::Io(s) => {
                HypermonError::RpcClientError(anyhow::anyhow!("RPC IO Error: {s:?}"))
            }
            web3::Error::Recovery(s) => {
                HypermonError::RpcClientError(anyhow::anyhow!("RPC Recovery Error: {s:?}"))
            }
            web3::Error::Internal => {
                HypermonError::RpcClientError(anyhow::anyhow!("Internal RPC Error"))
            }
        }
    }
}
