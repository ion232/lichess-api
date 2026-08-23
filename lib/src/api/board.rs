//! The Board API: play games as if from a physical board or other external
//! device, rather than the normal Lichess UI.
//!
//! Covers making moves, offering or claiming draws, resigning, berserking,
//! seeking a game, and chatting, plus the event and game-state streams needed
//! to drive a game in real time. All methods here require a bearer token with
//! the `board:play` scope.
//!
//! [`LichessApi::board_stream_incoming_events`] and
//! [`LichessApi::board_stream_board_state`] return streams of events rather
//! than a single response: keep the connection open and read from the stream
//! as moves and other updates happen, rather than polling.

use futures::stream::StreamExt;

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
    ) -> Result<impl StreamExt<Item = Result<Vec<chat::ChatLine>>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn board_write_in_chat(&self, request: impl Into<chat::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn board_claim_draw(
        &self,
        request: impl Into<claim_draw::PostRequest>,
    ) -> Result<bool> {
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
