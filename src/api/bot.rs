use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::bot::*;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn bot_abort_game(&self, request: abort::PostRequest) -> Result<bool> {
        self.get_ok_or_error_response(request).await
    }

    pub async fn bot_stream_game_chat(
        &self,
        request: chat::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<chat::ChatLine>>> {
        self.get_streamed_models(request).await
    }

    pub async fn bot_write_in_chat(&self, request: chat::PostRequest) -> Result<bool> {
        self.get_ok_or_error_response(request).await
    }

    pub async fn bot_make_move(&self, request: r#move::PostRequest) -> Result<bool> {
        self.get_ok_or_error_response(request).await
    }

    pub async fn bot_resign_game(&self, request: resign::PostRequest) -> Result<bool> {
        self.get_ok_or_error_response(request).await
    }

    pub async fn bot_stream_incoming_events(
        &self,
        request: stream_events::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<stream_events::Event>>> {
        self.get_streamed_models(request).await
    }

    pub async fn bot_stream_board_state(
        &self,
        request: stream_game::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<stream_game::Event>>> {
        self.get_streamed_models(request).await
    }

    pub async fn bot_upgrade_account(&self, request: upgrade::PostRequest) -> Result<bool> {
        self.get_ok_or_error_response(request).await
    }
}
