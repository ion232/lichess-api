use crate::client::LichessApi;
use crate::model::Ok;
use crate::model::messaging::*;
use crate::error::Result;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn send_message(&self, request: inbox::PostRequest) -> Result<bool> {
        let ok: Ok = self.get_single_model(request).await?;
        ok.into()
    }
}
