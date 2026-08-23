//! Arena tournaments: Lichess's continuous-pairing tournament format, where
//! players score points per game and climb a live leaderboard for the
//! duration of the event.
//!
//! Covers listing current tournaments, creating and updating them, joining,
//! withdrawing/pausing, terminating, managing team battles, and reading back
//! standings, results, team standings, and games. Creating or modifying a
//! tournament ([`create_arena_tournament`](LichessApi::create_arena_tournament),
//! [`update_arena_tournament`](LichessApi::update_arena_tournament),
//! [`join_arena_tournament`](LichessApi::join_arena_tournament),
//! [`withdraw_from_arena_tournament`](LichessApi::withdraw_from_arena_tournament),
//! [`terminate_arena_tournament`](LichessApi::terminate_arena_tournament), and
//! [`update_arena_team_battle`](LichessApi::update_arena_team_battle)) requires
//! a bearer token with the `tournament:write` scope; reading tournament info,
//! results, team standings, and games is public and needs no token.

use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::arena_tournaments::*;
use crate::model::games::GameJson;

impl LichessApi<reqwest::Client> {
    pub async fn get_current_arena_tournaments(&self) -> Result<ArenaTournaments> {
        self.get_single_model(current::GetRequest::new()).await
    }

    pub async fn create_arena_tournament(
        &self,
        request: impl Into<create::PostRequest>,
    ) -> Result<ArenaTournamentFull> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_arena_tournament(
        &self,
        id: &str,
        query: show::GetQuery,
    ) -> Result<ArenaTournamentFull> {
        self.get_single_model(show::GetRequest::new(id, query))
            .await
    }

    pub async fn update_arena_tournament(
        &self,
        id: &str,
        form: update::UpdateArenaTournamentForm,
    ) -> Result<ArenaTournamentFull> {
        self.get_single_model(update::PostRequest::new(id, form))
            .await
    }

    pub async fn export_arena_tournament_games(
        &self,
        id: &str,
        query: games::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<GameJson>>> {
        self.get_streamed_models(games::GetRequest::new(id, query))
            .await
    }

    pub async fn join_arena_tournament(
        &self,
        id: &str,
        form: join::JoinArenaTournamentForm,
    ) -> Result<bool> {
        self.get_ok(join::PostRequest::new(id, form)).await
    }

    pub async fn get_arena_tournament_results(
        &self,
        id: &str,
        query: results::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<ArenaResult>>> {
        self.get_streamed_models(results::GetRequest::new(id, query))
            .await
    }

    pub async fn get_arena_tournament_team_standing(
        &self,
        request: impl Into<teams::GetRequest>,
    ) -> Result<ArenaTeamStanding> {
        self.get_single_model(request.into()).await
    }

    pub async fn terminate_arena_tournament(
        &self,
        request: impl Into<terminate::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn withdraw_from_arena_tournament(
        &self,
        request: impl Into<withdraw::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn update_arena_team_battle(
        &self,
        id: &str,
        form: team_battle::TeamBattleForm,
    ) -> Result<ArenaTournamentFull> {
        self.get_single_model(team_battle::PostRequest::new(id, form))
            .await
    }

    pub async fn get_arena_tournaments_created_by_user(
        &self,
        username: &str,
        query: created_by_user::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<crate::model::ArenaTournament>>> {
        self.get_streamed_models(created_by_user::GetRequest::new(username, query))
            .await
    }

    pub async fn get_arena_tournaments_played_by_user(
        &self,
        username: &str,
        query: played_by_user::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<ArenaTournamentPlayed>>> {
        self.get_streamed_models(played_by_user::GetRequest::new(username, query))
            .await
    }
}
