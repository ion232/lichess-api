//! Broadcasts: Lichess's live relays of over-the-board tournaments, made up
//! of one tournament containing one or more rounds, each fed by an external
//! PGN source.
//!
//! Reading broadcasts (getting a tournament, round, player, team standings,
//! listing/searching/browsing top and official broadcasts, and exporting PGN)
//! is public and needs no token. Creating or managing your own broadcasts —
//! [`create_broadcast_tournament`](LichessApi::create_broadcast_tournament),
//! [`update_broadcast_tournament`](LichessApi::update_broadcast_tournament),
//! [`create_broadcast_round`](LichessApi::create_broadcast_round),
//! [`update_broadcast_round`](LichessApi::update_broadcast_round),
//! [`push_broadcast_round_pgn`](LichessApi::push_broadcast_round_pgn), and
//! [`reset_broadcast_round`](LichessApi::reset_broadcast_round) — requires a
//! bearer token with the `study:write` scope.
//!
//! The `stream_*_pgn` methods keep the connection open and yield PGN as
//! games progress, so they return a `Stream` rather than a single value;
//! `export_*_pgn` methods stream the same way but close once the current
//! games have been sent.

use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::broadcasts::*;

impl LichessApi<reqwest::Client> {
    pub async fn export_broadcast_pgn(
        &self,
        broadcast_tournament_id: &str,
        query: export_pgn::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(export_pgn::GetRequest::new(broadcast_tournament_id, query))
            .await
    }

    pub async fn export_broadcast_round_pgn(
        &self,
        broadcast_round_id: &str,
        query: export_round_pgn::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(export_round_pgn::GetRequest::new(broadcast_round_id, query))
            .await
    }

    pub async fn stream_broadcast_group_pgn(
        &self,
        broadcast_group_id: &str,
        query: stream_group_pgn::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(stream_group_pgn::GetRequest::new(broadcast_group_id, query))
            .await
    }

    pub async fn stream_broadcast_round_pgn(
        &self,
        broadcast_round_id: &str,
        query: stream_round_pgn::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(stream_round_pgn::GetRequest::new(broadcast_round_id, query))
            .await
    }

    pub async fn stream_broadcast_tournament_pgn(
        &self,
        broadcast_tour_id: &str,
        query: stream_tournament_pgn::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<String>>> {
        self.get_pgn(stream_tournament_pgn::GetRequest::new(
            broadcast_tour_id,
            query,
        ))
        .await
    }

    pub async fn get_broadcast_round(
        &self,
        broadcast_tournament_slug: &str,
        broadcast_round_slug: &str,
        broadcast_round_id: &str,
    ) -> Result<BroadcastRound> {
        self.get_single_model(get_round::GetRequest::new(
            broadcast_tournament_slug,
            broadcast_round_slug,
            broadcast_round_id,
        ))
        .await
    }

    pub async fn get_my_broadcast_rounds(
        &self,
        query: list_my_rounds::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<BroadcastMyRound>>> {
        self.get_streamed_models(list_my_rounds::GetRequest::new(query))
            .await
    }

    pub async fn get_broadcasts_by_user(
        &self,
        username: &str,
        query: list_by_user::GetQuery,
    ) -> Result<BroadcastByUserPaginator> {
        self.get_single_model(list_by_user::GetRequest::new(username, query))
            .await
    }

    pub async fn get_official_broadcasts(
        &self,
        query: list_official::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<BroadcastWithRounds>>> {
        self.get_streamed_models(list_official::GetRequest::from(query))
            .await
    }

    pub async fn search_broadcasts(
        &self,
        query: search::GetQuery,
    ) -> Result<BroadcastSearchPaginator> {
        self.get_single_model(search::GetRequest::from(query)).await
    }

    pub async fn get_top_broadcasts(&self, query: top::GetQuery) -> Result<BroadcastTop> {
        self.get_single_model(top::GetRequest::from(query)).await
    }

    pub async fn update_broadcast_tournament(
        &self,
        broadcast_tournament_id: &str,
        form: create_tournament::CreateBroadcastTournamentForm,
    ) -> Result<bool> {
        self.get_ok(update_tournament::PostRequest::new(
            broadcast_tournament_id,
            form,
        ))
        .await
    }

    pub async fn create_broadcast_round(
        &self,
        broadcast_tournament_id: &str,
        form: create_round::BroadcastRoundForm,
    ) -> Result<BroadcastRoundNew> {
        self.get_single_model(create_round::PostRequest::new(
            broadcast_tournament_id,
            form,
        ))
        .await
    }

    pub async fn get_broadcast_player(
        &self,
        broadcast_tournament_id: &str,
        player_id: &str,
    ) -> Result<BroadcastPlayerEntryWithFideAndGames> {
        self.get_single_model(get_player::GetRequest::new(
            broadcast_tournament_id,
            player_id,
        ))
        .await
    }

    pub async fn get_broadcast_players(
        &self,
        request: impl Into<get_players::GetRequest>,
    ) -> Result<Vec<BroadcastPlayerEntry>> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_broadcast_team_standings(
        &self,
        request: impl Into<get_team_standings::GetRequest>,
    ) -> Result<Vec<BroadcastTeamLeaderboardEntry>> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_broadcast_tournament(
        &self,
        request: impl Into<get_tournament::GetRequest>,
    ) -> Result<BroadcastWithRoundsAndFullGroup> {
        self.get_single_model(request.into()).await
    }

    pub async fn create_broadcast_tournament(
        &self,
        form: create_tournament::CreateBroadcastTournamentForm,
    ) -> Result<BroadcastWithRounds> {
        self.get_single_model(create_tournament::PostRequest::new(form))
            .await
    }

    pub async fn update_broadcast_round(
        &self,
        broadcast_round_id: &str,
        query: update_round::PostQuery,
        form: create_round::BroadcastRoundForm,
    ) -> Result<BroadcastRound> {
        self.get_single_model(update_round::PostRequest::new(
            broadcast_round_id,
            query,
            form,
        ))
        .await
    }

    pub async fn push_broadcast_round_pgn(
        &self,
        broadcast_round_id: &str,
        pgn: String,
    ) -> Result<BroadcastPgnPush> {
        self.get_single_model(push_pgn::PostRequest::new(broadcast_round_id, pgn))
            .await
    }

    pub async fn reset_broadcast_round(
        &self,
        request: impl Into<reset_round::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }
}
