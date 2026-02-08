use sterling_core::error::ProviderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnthropicError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<AnthropicError> for ProviderError {
    fn from(err: AnthropicError) -> Self {
        match err {
            AnthropicError::Network(e) => ProviderError::Network(e),
            AnthropicError::Api { status, message } => ProviderError::Api { status, message },
            AnthropicError::Serialization(e) => ProviderError::Serialization(e),
        }
    }
}
