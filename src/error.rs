use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranslateError {
    #[error("HTTP error: {status} - {message}")]
    HttpError {
        status: u16,
        message: String,
        ip_address: String,
        time: String,
        url: String,
    },
    
    #[error("Google Translate API error: {0}")]
    ApiError(String),

    #[error("Reqwest error: {0}")]
    ReqwestError(reqwest::Error),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Failed to build URL: {0}")]
    UrlBuildError(#[from] url::ParseError),

    #[error("Text decode error: {0}")]
    TextDecodeError(String),

    #[error("Invalid input: {0}")]
    InvalidInputError(String),
}
