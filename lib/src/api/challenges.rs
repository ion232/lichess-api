//! Challenge other players, bots, or the Lichess AI to a game.
//!
//! Covers listing your incoming and outgoing challenges, creating, accepting,
//! declining, and canceling a challenge, creating an open challenge anyone can
//! join, and starting a game against the Lichess AI. Most operations require a
//! bearer token with a `challenge:*` scope; open challenges and public reads
//! are the exceptions. [`LichessApi::admin_challenge_tokens`] is restricted to
//! Lichess administrators.
//!
//! Request/response types live in [`crate::model::challenges`].

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::challenges::*;
use crate::model::games::stream::moves::Move;

impl LichessApi<reqwest::Client> {
    pub async fn list_challenges(&self) -> Result<list::Challenges> {
        self.get_single_model(list::GetRequest::new()).await
    }

    pub async fn create_challenge(
        &self,
        request: impl Into<create::PostRequest>,
    ) -> Result<ChallengeJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn accept_challenge(&self, request: impl Into<accept::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn decline_challenge(
        &self,
        request: impl Into<decline::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn cancel_challenge(&self, request: impl Into<cancel::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn challenge_ai(&self, request: impl Into<ai::PostRequest>) -> Result<Move> {
        self.get_single_model(request.into()).await
    }

    pub async fn create_open_challenge(
        &self,
        request: impl Into<open::PostRequest>,
    ) -> Result<ChallengeOpenJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn start_clocks(
        &self,
        request: impl Into<start_clocks::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn add_time_to_opponent_clock(
        &self,
        request: impl Into<add_time::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn show_challenge(
        &self,
        request: impl Into<show::GetRequest>,
    ) -> Result<ChallengeJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn admin_challenge_tokens(
        &self,
        request: impl Into<admin_challenge::PostRequest>,
    ) -> Result<AdminChallengeTokenResults> {
        self.get_single_model(request.into()).await
    }
}
