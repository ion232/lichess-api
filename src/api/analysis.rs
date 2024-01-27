use crate::client::LichessApi;
use crate::error::Result;
use crate::model::analysis::*;

impl LichessApi<reqwest::Client> {
    pub async fn get_cloud_evaluation(
        &self,
        request: cloud::GetRequest,
    ) -> Result<cloud::Evaluation> {
        self.get_single_model(request).await
    }
}
