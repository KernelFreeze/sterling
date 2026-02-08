pub mod error;
mod messages;
pub mod types;

use typed_builder::TypedBuilder;

const DEFAULT_BASE_URL: &str = "https://api.anthropic.com";
const DEFAULT_API_VERSION: &str = "2023-06-01";

#[derive(Debug, Clone, TypedBuilder)]
pub struct AnthropicClient {
    pub api_key: String,
    #[builder(default = DEFAULT_BASE_URL.to_owned())]
    pub base_url: String,
    #[builder(default = DEFAULT_API_VERSION.to_owned())]
    pub api_version: String,
    #[builder(default = reqwest::Client::new())]
    pub http: reqwest::Client,
}
