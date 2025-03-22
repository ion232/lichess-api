use crate::client::LichessApi;
use crate::error::Result;
use crate::model::oauth::*;

impl LichessApi<reqwest::Client> {
    pub async fn test_tokens(&self, request: impl Into<test::PostRequest>) -> Result<TestResults> {
        self.get_single_model(request.into()).await
    }

    pub async fn revoke_token(&self) -> Result<()> {
        self.get_empty(revoke::DeleteRequest::new()).await
    }
}
