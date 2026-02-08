mod chat;
pub mod error;
mod responses;
pub mod types;

use typed_builder::TypedBuilder;

const DEFAULT_BASE_URL: &str = "https://api.openai.com";

#[derive(Debug, Clone, TypedBuilder)]
pub struct OpenAiClient {
    pub api_key: String,
    #[builder(default = DEFAULT_BASE_URL.to_owned())]
    pub base_url: String,
    #[builder(default = reqwest::Client::new())]
    pub http: reqwest::Client,
}

#[cfg(test)]
mod tests {}
