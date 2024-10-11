use crate::client::LichessApi;
use crate::error::Result;
use crate::model::oauth::*;

impl LichessApi<reqwest::Client> {
    pub async fn test_tokens(&self, request: impl Into<test::PostRequest>) -> Result<TestResults> {
        self.get_single_model(request.into()).await
    }

    pub async fn revoke_token(&self) -> Result<bool> {
        // XXX get_ok expects a bool, but this method just returns 200
        self.get_ok(revoke::DeleteRequest::new()).await
    }
}
