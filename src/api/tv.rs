use crate::client::LichessApi;
use crate::error::Result;
use crate::model::tv::*;

impl LichessApi<reqwest::Client> {
    pub async fn tv_channels(&self, request: channels::GetRequest) -> Result<Channels> {
        self.get_single_model(request).await
    }
}
