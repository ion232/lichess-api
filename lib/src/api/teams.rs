//! Team info, membership, and team-only tournament listings.
//!
//! Lookups such as [`LichessApi::get_team`], [`LichessApi::search_teams`],
//! [`LichessApi::get_team_members`], and the team's arena/swiss tournament
//! listings are public. Joining and quitting a team need a token with
//! `team:write`; reading your team updates needs `team:read`. Everything
//! else here — viewing join requests, accepting or declining them, kicking a
//! member, and sending a team update — acts on a team you lead and needs a
//! token with `team:lead`.

use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::teams::*;
use crate::model::{ArenaTournament, SwissTournament};

impl LichessApi<reqwest::Client> {
    pub async fn get_popular_teams(
        &self,
        request: impl Into<all::GetRequest>,
    ) -> Result<TeamPaginatorJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn search_teams(
        &self,
        request: impl Into<search::GetRequest>,
    ) -> Result<TeamPaginatorJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_teams_of_player(
        &self,
        request: impl Into<of_username::GetRequest>,
    ) -> Result<Vec<Team>> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_team(&self, request: impl Into<show::GetRequest>) -> Result<Team> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_team_members(
        &self,
        request: impl Into<users::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<users::TeamMember>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn get_team_arena_tournaments(
        &self,
        request: impl Into<arena::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<ArenaTournament>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn get_team_swiss_tournaments(
        &self,
        request: impl Into<swiss::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<SwissTournament>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn get_team_join_requests(
        &self,
        request: impl Into<requests::GetRequest>,
    ) -> Result<Vec<TeamRequestWithUser>> {
        self.get_single_model(request.into()).await
    }

    pub async fn accept_team_join_request(
        &self,
        request: impl Into<request_accept::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn decline_team_join_request(
        &self,
        request: impl Into<request_decline::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn kick_team_member(&self, request: impl Into<kick::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn join_team(&self, request: impl Into<join::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn quit_team(&self, request: impl Into<quit::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn send_team_update(&self, request: impl Into<pm_all::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn get_team_updates(
        &self,
        request: impl Into<updates::GetRequest>,
    ) -> Result<TeamUpdates> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_team_updates_of_team(
        &self,
        request: impl Into<updates_of_team::GetRequest>,
    ) -> Result<TeamUpdatesOfTeam> {
        self.get_single_model(request.into()).await
    }
}
