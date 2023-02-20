use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::board::*;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn board_abort_game(&self, request: abort::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn board_berserk_game(&self, request: berserk::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn board_stream_game_chat(
        &self,
        request: chat::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<chat::ChatLine>>> {
        self.get_streamed_models(request).await
    }

    pub async fn board_write_in_chat(&self, request: chat::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn board_claim_victory(&self, request: claim_victory::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn board_handle_draw(&self, request: draw::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn board_make_move(&self, request: r#move::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn board_resign_game(&self, request: resign::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn board_create_a_seek(
        &self,
        request: seek::PostRequest,
    ) -> Result<impl StreamExt<Item = Result<serde_json::Value>>> {
        self.get_streamed_models(request).await
    }

    pub async fn board_stream_incoming_events(
        &self,
        request: stream_events::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<stream_events::Event>>> {
        self.get_streamed_models(request).await
    }

    pub async fn board_stream_board_state(
        &self,
        request: stream_game::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<stream_game::Event>>> {
        self.get_streamed_models(request).await
    }

    pub async fn board_handle_takeback(&self, request: takeback::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }
}
