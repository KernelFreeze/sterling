use std::error::Error;

use crate::error::ProviderError;

pub trait StatelessProvider {
    type Message: Send + Sync;
    type Request: Send + Sync;
    type Response: Send + Sync;
    type Error: Into<ProviderError> + Error + Send + Sync + 'static;

    async fn complete(&self, request: Self::Request) -> Result<Self::Response, Self::Error>;
}

pub trait ProviderResponse {
    type ResponseId: Send + Sync + Clone;

    fn response_id(&self) -> Self::ResponseId;
}

pub trait StatefulProvider {
    type Input: Send + Sync;
    type Request: Send + Sync;
    type Response: ProviderResponse + Send + Sync;
    type Error: Into<ProviderError> + Error + Send + Sync + 'static;

    async fn respond(&self, request: Self::Request) -> Result<Self::Response, Self::Error>;
}
