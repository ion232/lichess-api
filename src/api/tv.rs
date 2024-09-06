use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::games::GameJson;
use crate::model::tv::*;

impl LichessApi<reqwest::Client> {
    pub async fn tv_channels(&self, request: channels::GetRequest) -> Result<Channels> {
        self.get_single_model(request).await
    }

    pub async fn tv_stream_current(
        &self,
        request: stream::current::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<stream::Event>>> {
        self.get_streamed_models(request).await
    }

    pub async fn tv_stream_channel_current(
        &self,
        request: stream::channel::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<stream::Event>>> {
        self.get_streamed_models(request).await
    }

    pub async fn tv_channel_games(
        &self,
        request: games::GetRequest,
    ) -> Result<impl StreamExt<Item = Result<GameJson>>> {
        self.get_streamed_models(request).await
    }
}
