use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::bot::*;

impl LichessApi<reqwest::Client> {
    pub async fn bot_abort_game(&self, request: abort::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn bot_stream_game_chat(
        &self,
        request: chat::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<chat::ChatLine>>> {
        self.get_streamed_models(request).await
    }

    pub async fn bot_write_in_chat(&self, request: chat::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn bot_make_move(&self, request: r#move::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn bot_resign_game(&self, request: resign::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }

    pub async fn bot_stream_incoming_events(
        &self,
        request: stream::events::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<stream::events::Event>>> {
        self.get_streamed_models(request).await
    }

    pub async fn bot_stream_board_state(
        &self,
        request: stream::game::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<stream::game::Event>>> {
        self.get_streamed_models(request).await
    }

    pub async fn bot_upgrade_account(&self, request: upgrade::PostRequest) -> Result<bool> {
        self.get_ok(request).await
    }
}
