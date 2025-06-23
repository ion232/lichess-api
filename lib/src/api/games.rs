use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::games::*;

impl LichessApi<reqwest::Client> {
    pub async fn export_one_game(
        &self,
        request: impl Into<export::one::GetRequest>,
    ) -> Result<GameJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn export_ongoing_game(
        &self,
        request: impl Into<export::ongoing::GetRequest>,
    ) -> Result<GameJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn export_games_of_user(
        &self,
        request: impl Into<export::by_user::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<GameJson>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn export_games_by_ids(
        &self,
        request: impl Into<export::by_ids::PostRequest>,
    ) -> Result<impl StreamExt<Item = Result<GameJson>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn stream_games_of_users(
        &self,
        request: impl Into<stream::by_users::PostRequest>,
    ) -> Result<impl StreamExt<Item = Result<GameStream>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn stream_games_by_ids(
        &self,
        request: impl Into<stream::by_ids::PostRequest>,
    ) -> Result<impl StreamExt<Item = Result<GameStream>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn add_game_ids_to_stream(
        &self,
        request: impl Into<stream::add_ids::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn get_my_ongoing_games(
        &self,
        request: impl Into<ongoing::GetRequest>,
    ) -> Result<ongoing::Games> {
        self.get_single_model(request.into()).await
    }

    pub async fn stream_game_moves(
        &self,
        request: impl Into<stream::moves::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<stream::moves::Move>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn import_game(
        &self,
        request: impl Into<import::PostRequest>,
    ) -> Result<import::ImportData> {
        self.get_single_model(request.into()).await
    }
}
