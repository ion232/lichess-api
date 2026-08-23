//! Look up FIDE-rated players and their rating history.
//!
//! Search by name with [`LichessApi::search_fide_player`], fetch a single
//! [`Player`] by FIDE ID with [`LichessApi::get_fide_player`], and get their
//! historical standard, rapid, and blitz ratings with
//! [`LichessApi::get_fide_player_ratings`]. This data is public and does not
//! require a token.

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::fide::*;

impl LichessApi<reqwest::Client> {
    pub async fn search_fide_player(
        &self,
        request: impl Into<search::GetRequest>,
    ) -> Result<Vec<Player>> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_fide_player(&self, request: impl Into<player::GetRequest>) -> Result<Player> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_fide_player_ratings(
        &self,
        request: impl Into<ratings::GetRequest>,
    ) -> Result<ratings::PlayerRatings> {
        self.get_single_model(request.into()).await
    }
}
