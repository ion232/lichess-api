//! Query move statistics for a position from the Opening Explorer: aggregated
//! master games ([`openings_masters`](LichessApi::openings_masters)), rated
//! Lichess games ([`openings_lichess`](LichessApi::openings_lichess)), or a
//! specific player's games ([`openings_player`](LichessApi::openings_player)),
//! plus fetching a masters game's PGN by ID
//! ([`openings_otb`](LichessApi::openings_otb)). All of these are public and
//! need no token, and are served from a separate host,
//! [`Domain::Explorer`](crate::model::Domain::Explorer).

use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::openings::*;

impl LichessApi<reqwest::Client> {
    pub async fn openings_masters(
        &self,
        request: impl Into<masters::GetRequest>,
    ) -> Result<OpeningExplorerJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn openings_lichess(
        &self,
        request: impl Into<lichess::GetRequest>,
    ) -> Result<OpeningExplorerJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn openings_player(
        &self,
        request: impl Into<player::GetRequest>,
    ) -> Result<OpeningExplorerJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn openings_otb(
        &self,
        request: impl Into<otb::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(request.into()).await
    }
}
