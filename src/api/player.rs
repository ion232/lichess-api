use crate::{client::LichessApi, error::Result, model::player::*};

impl LichessApi<reqwest::Client> {
    pub async fn get_all_top_10(&self) -> Result<Leaderboards> {
        self.get_single_model(top_10::GetRequest::default()).await
    }

    pub async fn get_one_leaderboard(
        &self,
        request: impl Into<leaderboard::GetRequest>,
    ) -> Result<Leaderboard> {
        self.get_single_model(request.into()).await
    }
}
