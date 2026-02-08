use sterling_core::provider::{ProviderResponse, StatefulProvider};

use crate::OpenAiClient;
use crate::error::OpenAiError;
use crate::types::{ApiErrorBody, ResponseInput, ResponseRequest, ResponseResponse};

impl ProviderResponse for ResponseResponse {
    type ResponseId = String;

    fn response_id(&self) -> Self::ResponseId {
        self.id.clone()
    }
}

impl StatefulProvider for OpenAiClient {
    type Error = OpenAiError;
    type Input = ResponseInput;
    type Request = ResponseRequest;
    type Response = ResponseResponse;

    async fn respond(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
        let url = format!("{}/v1/responses", self.base_url);

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
