use sterling_core::error::ProviderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenAiError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<OpenAiError> for ProviderError {
    fn from(err: OpenAiError) -> Self {
        match err {
            OpenAiError::Network(e) => ProviderError::Network(e),
            OpenAiError::Api { status, message } => ProviderError::Api { status, message },
            OpenAiError::Serialization(e) => ProviderError::Serialization(e),
        }
    }
}
