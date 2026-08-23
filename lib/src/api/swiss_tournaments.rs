//! Swiss-system tournaments.
//!
//! Unlike arena tournaments, Swiss tournaments are always organized by a team
//! ([`LichessApi::create_swiss_tournament`] takes a `team_id`), run over a
//! fixed number of rounds, and pair players based on score each round rather
//! than continuously. Most write operations (creating, updating, joining,
//! scheduling the next round, terminating, withdrawing) require the
//! `tournament:write` scope; reading tournament info, results, games, and the
//! TRF export are public.

use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::SwissTournament;
use crate::model::games::GameJson;
use crate::model::swiss_tournaments::*;

impl LichessApi<reqwest::Client> {
    pub async fn create_swiss_tournament(
        &self,
        team_id: &str,
        form: create::CreateSwissTournamentForm,
    ) -> Result<SwissTournament> {
        self.get_single_model(create::PostRequest::new(team_id, form))
            .await
    }

    pub async fn get_swiss_tournament(
        &self,
        request: impl Into<show::GetRequest>,
    ) -> Result<SwissTournament> {
        self.get_single_model(request.into()).await
    }

    pub async fn update_swiss_tournament(
        &self,
        id: &str,
        form: update::UpdateSwissTournamentForm,
    ) -> Result<SwissTournament> {
        self.get_single_model(update::PostRequest::new(id, form))
            .await
    }

    pub async fn export_swiss_tournament_games(
        &self,
        id: &str,
        query: games::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<GameJson>>> {
        self.get_streamed_models(games::GetRequest::new(id, query))
            .await
    }

    pub async fn join_swiss_tournament(
        &self,
        id: &str,
        form: join::JoinSwissTournamentForm,
    ) -> Result<bool> {
        self.get_ok(join::PostRequest::new(id, form)).await
    }

    pub async fn get_swiss_tournament_results(
        &self,
        id: &str,
        query: results::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<SwissResult>>> {
        self.get_streamed_models(results::GetRequest::new(id, query))
            .await
    }

    pub async fn schedule_next_swiss_round(
        &self,
        id: &str,
        form: schedule_next_round::ScheduleNextRoundForm,
    ) -> Result<()> {
        self.get_empty(schedule_next_round::PostRequest::new(id, form))
            .await
    }

    pub async fn terminate_swiss_tournament(
        &self,
        request: impl Into<terminate::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn withdraw_from_swiss_tournament(
        &self,
        request: impl Into<withdraw::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn get_swiss_tournament_trf(
        &self,
        request: impl Into<trf::GetRequest>,
    ) -> Result<String> {
        self.get_text(request.into()).await
    }
}
