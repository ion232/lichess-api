pub mod account;
pub mod analysis;
pub mod arena_tournaments;
pub mod board;
pub mod bot;
pub mod broadcasts;
pub mod bulk_pairings;
pub mod challenges;
pub mod external_engine;
pub mod games;
pub mod messaging;
pub mod oauth;
pub mod opening_explorer;
pub mod puzzles;
pub mod relations;
pub mod simuls;
pub mod studies;
pub mod swiss_tournaments;
pub mod tablebase;
pub mod teams;
pub mod tv;
pub mod users;

use async_std::stream::StreamExt;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::client::LichessApi;
use crate::model::Request;
use crate::error::Result;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn get_single_model<Q, B, M>(&self, request: Request<Q, B>) -> Result<M> where
        Q: Serialize + Default,
        B: Serialize + ToString,
        M: DeserializeOwned
    {
        let request = request.as_http_request()?;
        let mut stream = self.send(request).await?;
        self.expect_one_model(&mut stream).await
    }

    pub async fn get_streamed_models<Q, B, M>(&self, request: Request<Q, B>)
        -> Result<impl StreamExt<Item = Result<M>>> where
        Q: Serialize + Default,
        B: Serialize + ToString,
        M: DeserializeOwned
    {
        let request = request.as_http_request()?;
        self.send(request).await
    }
}
