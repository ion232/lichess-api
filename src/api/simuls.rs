use crate::client::LichessApi;
use crate::error::Result;
use crate::model::simuls::*;

impl LichessApi<reqwest::Client> {
    pub async fn get_current_simuls(&self) -> Result<current::Simuls> {
        self.get_single_model(current::GetRequest::new()).await
    }
}
