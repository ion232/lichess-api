//! Look up Syzygy endgame tablebase results for positions with few pieces
//! remaining, one method per variant: standard chess
//! ([`lookup_standard`](LichessApi::lookup_standard)), antichess
//! ([`lookup_antichess`](LichessApi::lookup_antichess)), and atomic chess
//! ([`lookup_atomic`](LichessApi::lookup_atomic)). Each returns the win/loss/draw
//! category and, where known, distance-to-zero and distance-to-mate for the
//! position and for every legal move. These endpoints are public and need no
//! token, and are served from a separate host,
//! [`Domain::Tablebase`](crate::model::Domain::Tablebase).

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::tablebase::*;

impl LichessApi<reqwest::Client> {
    pub async fn lookup_antichess(
        &self,
        request: impl Into<antichess::GetRequest>,
    ) -> Result<TablebaseJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn lookup_atomic(
        &self,
        request: impl Into<atomic::GetRequest>,
    ) -> Result<TablebaseJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn lookup_standard(
        &self,
        request: impl Into<standard::GetRequest>,
    ) -> Result<TablebaseJson> {
        self.get_single_model(request.into()).await
    }
}
