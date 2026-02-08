use sterling_core::provider::StatelessProvider;

use crate::AnthropicClient;
use crate::error::AnthropicError;
use crate::types::{ApiErrorBody, Message, MessagesRequest, MessagesResponse};

impl StatelessProvider for AnthropicClient {
    type Error = AnthropicError;
    type Message = Message;
    type Request = MessagesRequest;
    type Response = MessagesResponse;

    async fn complete(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
        let url = format!("{}/v1/messages", self.base_url);

        let response = self
            .http
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", &self.api_version)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body: ApiErrorBody = response.json().await?;
            return Err(AnthropicError::Api {
                status: status.as_u16(),
                message: body.error.message,
            });
        }

        let body = response.json().await?;
        Ok(body)
    }
}
