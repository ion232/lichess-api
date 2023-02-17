use crate::client::LichessApi;
use crate::error::Result;
use crate::model::messaging::*;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn send_message(&self, request: inbox::PostRequest) -> Result<bool> {
        self.get_ok_or_error_response(request).await
    }
}
