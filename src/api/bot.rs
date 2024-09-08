use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::bot::online::OnlineBot;
use crate::model::bot::*;

impl LichessApi<reqwest::Client> {
    pub async fn bot_abort_game(&self, request: impl Into<abort::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn bot_stream_game_chat(
        &self,
        request: impl Into<chat::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<chat::ChatLine>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn bot_write_in_chat(&self, request: impl Into<chat::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn bot_draw_game(&self, request: impl Into<draw::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn bot_make_move(&self, request: impl Into<r#move::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn bot_get_online(
        &self,
        request: impl Into<online::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<OnlineBot>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn bot_resign_game(&self, request: impl Into<resign::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn bot_stream_incoming_events(
        &self,
        request: impl Into<stream::events::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<stream::events::Event>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn bot_stream_board_state(
        &self,
        request: impl Into<stream::game::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<stream::game::Event>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn bot_upgrade_account(
        &self,
        request: impl Into<upgrade::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }
}
