use crate::client::LichessApi;
use crate::error::Result;
use crate::model::messaging::*;

impl LichessApi<reqwest::Client> {
    pub async fn send_message(&self, request: impl Into<inbox::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }
}
