use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::board::*;

impl LichessApi<reqwest::Client> {
    pub async fn board_abort_game(&self, request: impl Into<abort::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn board_berserk_game(
        &self,
        request: impl Into<berserk::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn board_stream_game_chat(
        &self,
        request: impl Into<chat::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<chat::ChatLine>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn board_write_in_chat(&self, request: impl Into<chat::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn board_claim_victory(
        &self,
        request: impl Into<claim_victory::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn board_handle_draw(&self, request: impl Into<draw::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn board_make_move(&self, request: impl Into<r#move::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn board_resign_game(&self, request: impl Into<resign::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn board_create_a_seek(
        &self,
        request: impl Into<seek::PostRequest>,
    ) -> Result<impl StreamExt<Item = Result<serde_json::Value>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn board_stream_incoming_events(
        &self,
        request: impl Into<stream::events::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<stream::events::Event>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn board_stream_board_state(
        &self,
        request: impl Into<stream::game::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<stream::game::Event>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn board_handle_takeback(
        &self,
        request: impl Into<takeback::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }
}
