//! Play games as a Lichess [Bot account](https://lichess.org/api#tag/Bot).
//!
//! A regular player account must first be upgraded to a Bot account with
//! [`bot_upgrade_account`](crate::client::LichessApi::bot_upgrade_account) —
//! irreversible, and only possible before the account has played any game.
//! Once upgraded, a bot streams incoming challenges and game state and reacts
//! to them (moves, resignations, draw/takeback offers, chat) rather than
//! using the normal web UI; see the [board](crate::api::board) module for the
//! equivalent flow for human-driven clients.
//!
//! All methods here require a bearer token with the `bot:play` scope, except
//! [`bot_get_online`](crate::client::LichessApi::bot_get_online), which is
//! public.

use futures::stream::StreamExt;

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

    pub async fn bot_claim_draw(
        &self,
        request: impl Into<claim_draw::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn bot_claim_victory(
        &self,
        request: impl Into<claim_victory::PostRequest>,
    ) -> Result<bool> {
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

    pub async fn bot_handle_takeback(
        &self,
        request: impl Into<takeback::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn bot_upgrade_account(
        &self,
        request: impl Into<upgrade::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }
}
