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
