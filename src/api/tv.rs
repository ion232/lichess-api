use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::games::GameJson;
use crate::model::tv::*;

impl LichessApi<reqwest::Client> {
    pub async fn tv_channels(&self) -> Result<Channels> {
        self.get_single_model(channels::GetRequest::new()).await
    }

    pub async fn tv_stream_current(&self) -> Result<impl StreamExt<Item = Result<stream::Event>>> {
        self.get_streamed_models(stream::current::GetRequest::new())
            .await
    }

    pub async fn tv_stream_channel_current(
        &self,
        request: impl Into<stream::channel::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<stream::Event>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn tv_channel_games(
        &self,
        request: impl Into<games::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<GameJson>>> {
        self.get_streamed_models(request.into()).await
    }
}
