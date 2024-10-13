pub mod account;
pub mod analysis;
pub mod board;
pub mod bot;
pub mod challenges;
pub mod external_engine;
pub mod games;
pub mod messaging;
pub mod oauth;
pub mod openings;
pub mod puzzles;
pub mod simuls;
pub mod studies;
pub mod tablebase;
pub mod tv;
pub mod users;

use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::{Error, Result};
use crate::model::{BodyBounds, ModelBounds, QueryBounds, Request, Response};

impl LichessApi<reqwest::Client> {
    pub async fn get_ok<Q, B>(&self, request: Request<Q, B>) -> Result<bool>
    where
        Q: QueryBounds,
        B: BodyBounds,
    {
        let result = self
            .get_single_model::<Q, B, crate::model::Ok>(request)
            .await;
        return Ok(result?.ok);
    }

    pub async fn get_pgn<Q, B>(
        &self,
        request: Request<Q, B>,
    ) -> Result<impl StreamExt<Item = Result<String>>>
    where
        Q: QueryBounds,
        B: BodyBounds,
    {
        let request = request.as_http_request("application/x-chess-pgn")?;
        let stream = self.make_request_as_raw_lines(request).await?;
        Ok(stream)
    }

    pub async fn get_empty<Q, B>(&self, request: Request<Q, B>) -> Result<()>
    where
        Q: QueryBounds,
        B: BodyBounds,
    {
        let request = request.as_http_request("application/json")?;
        let mut stream = self.make_request(request).await?;
        self.expect_empty(&mut stream).await?;
        Ok(())
    }

    pub async fn get_single_model<Q, B, M>(&self, request: Request<Q, B>) -> Result<M>
    where
        Q: QueryBounds,
        B: BodyBounds,
        M: ModelBounds,
    {
        let request = request.as_http_request("application/json")?;
        let mut stream = self.make_request(request).await?;
        let res: Response<M> = self.expect_one_model(&mut stream).await?;
        match res {
            Response::Model(m) => Ok(m),
            Response::Error { error } => Err(Error::LichessStatus(error)),
        }
    }

    pub async fn get_streamed_models<Q, B, M>(
        &self,
        request: Request<Q, B>,
    ) -> Result<impl StreamExt<Item = Result<M>>>
    where
        Q: QueryBounds,
        B: BodyBounds,
        M: ModelBounds,
    {
        let request = request.as_http_request("application/x-ndjson")?;
        self.make_request(request).await
    }
}
