use sterling_core::provider::StatelessProvider;

use crate::OpenAiClient;
use crate::error::OpenAiError;
use crate::types::{ApiErrorBody, ChatCompletionRequest, ChatCompletionResponse, ChatMessage};

impl StatelessProvider for OpenAiClient {
    type Error = OpenAiError;
    type Message = ChatMessage;
    type Request = ChatCompletionRequest;
    type Response = ChatCompletionResponse;

    async fn complete(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
        let url = format!("{}/v1/chat/completions", self.base_url);

        let response = self
            .http
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body: ApiErrorBody = response.json().await?;
            return Err(OpenAiError::Api {
                status: status.as_u16(),
                message: body.error.message,
            });
        }

        let body = response.json().await?;
        Ok(body)
    }
}
