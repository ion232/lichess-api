use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::puzzles::*;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn get_daily_puzzle(&self, request: daily::GetRequest) -> Result<daily::Puzzle> {
        self.get_single_model(request).await
    }

    pub async fn get_puzzle(&self, request: id::GetRequest) -> Result<id::Puzzle> {
        self.get_single_model(request).await
    }

    pub async fn get_puzzle_activity(
        &self,
        request: activity::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<activity::Round>>> {
        self.get_streamed_models(request).await
    }

    pub async fn get_puzzle_dashboard(
        &self,
        request: dashboard::GetRequest,
    ) -> Result<dashboard::Dashboard> {
        self.get_model_or_error_response(request).await
    }

    pub async fn get_puzzle_storm_dashboard(
        &self,
        request: storm_dashboard::GetRequest,
    ) -> Result<storm_dashboard::Dashboard> {
        self.get_single_model(request).await
    }

    pub async fn make_puzzle_race(&self, request: race::PostRequest) -> Result<race::Race> {
        self.get_single_model(request).await
    }
}
