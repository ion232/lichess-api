//! Fetching puzzles, tracking puzzle progress, and puzzle racing.
//!
//! Covers the daily puzzle, fetching a puzzle by ID or a random next puzzle,
//! batches of puzzles for offline play, puzzle activity and the puzzle
//! dashboard, puzzles to replay for a theme, and the Puzzle Storm dashboard.
//! Also covers creating and joining a puzzle race and fetching its results.
//!
//! The daily puzzle, puzzle-by-ID, and Storm dashboard lookups are public.
//! Everything else needs a bearer token: reading activity, the dashboard,
//! replays, and fetching puzzles requires a `puzzle:read` scope, solving a
//! batch requires `puzzle:write`, and creating a race requires `racer:write`.
//!
//! Request/response types live in [`crate::model::puzzles`].

use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::puzzles::*;

impl LichessApi<reqwest::Client> {
    pub async fn get_daily_puzzle(&self) -> Result<daily::Puzzle> {
        self.get_single_model(daily::GetRequest::new()).await
    }

    pub async fn get_puzzle(&self, request: impl Into<id::GetRequest>) -> Result<id::Puzzle> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_new_puzzle(
        &self,
        request: impl Into<next::GetRequest>,
    ) -> Result<next::Puzzle> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_puzzle_activity(
        &self,
        request: impl Into<activity::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<activity::Activity>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn get_puzzles_to_replay(
        &self,
        request: impl Into<replay::GetRequest>,
    ) -> Result<replay::Replay> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_puzzle_dashboard(
        &self,
        request: impl Into<dashboard::GetRequest>,
    ) -> Result<dashboard::Dashboard> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_puzzle_storm_dashboard(
        &self,
        request: impl Into<storm_dashboard::GetRequest>,
    ) -> Result<storm_dashboard::Dashboard> {
        self.get_single_model(request.into()).await
    }

    pub async fn make_puzzle_race(
        &self,
        request: impl Into<race::PostRequest>,
    ) -> Result<race::Race> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_puzzle_batch(
        &self,
        request: impl Into<batch::GetRequest>,
    ) -> Result<batch::Select> {
        self.get_single_model(request.into()).await
    }

    pub async fn solve_puzzle_batch(
        &self,
        request: impl Into<batch::PostRequest>,
    ) -> Result<batch::SolveResponse> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_puzzle_race_results(
        &self,
        request: impl Into<racer::GetRequest>,
    ) -> Result<racer::RaceResults> {
        self.get_single_model(request.into()).await
    }
}
