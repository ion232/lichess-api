//! Position analysis.
//!
//! Lichess maintains a database of cloud engine evaluations for positions it
//! has already analyzed (mostly openings, around 320 million positions).
//! [`LichessApi::get_cloud_evaluation`] looks one up by FEN and returns its
//! principal variations if present, or an error if the position hasn't been
//! evaluated. This endpoint is public and needs no bearer token; it's meant
//! for occasional lookups, not bulk fetching — for that, use the exported
//! evaluation database from lichess.org directly.

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::analysis::*;

impl LichessApi<reqwest::Client> {
    pub async fn get_cloud_evaluation(
        &self,
        request: impl Into<cloud::GetRequest>,
    ) -> Result<cloud::Evaluation> {
        self.get_single_model(request.into()).await
    }
}
