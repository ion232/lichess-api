use crate::client::LichessApi;
use crate::error::Result;
use crate::model::challenges::*;
use crate::model::games::GameJson;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn list_challenges(&self, request: list::GetRequest) -> Result<list::Challenges> {
        self.get_single_model(request).await
    }

    pub async fn create_challenge(&self, request: create::PostRequest) -> Result<ChallengeCreated> {
        self.get_single_model(request).await
    }

    pub async fn accept_challenge(&self, request: accept::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn decline_challenge(&self, request: decline::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn cancel_challenge(&self, request: cancel::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn challenge_ai(&self, request: ai::PostRequest) -> Result<GameJson> {
        self.get_single_model(request).await
    }

    pub async fn create_open_challenge(
        &self,
        request: open::PostRequest,
    ) -> Result<ChallengeOpenJson> {
        self.get_single_model(request).await
    }

    pub async fn start_clocks(&self, request: start_clocks::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn add_time_to_opponent_clock(&self, request: add_time::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }
}
