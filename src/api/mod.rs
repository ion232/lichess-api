pub mod account;
pub mod board;
pub mod bot;
pub mod challenges;
pub mod games;
pub mod messaging;
pub mod puzzles;
pub mod users;

use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::{Error, Result};
use crate::model::{BodyBounds, Response, ModelBounds, QueryBounds, Request};

impl<'a> LichessApi<'a, reqwest::Client> {
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

    pub async fn get_single_model<Q, B, M>(&self, request: Request<Q, B>) -> Result<M>
    where
        Q: QueryBounds,
        B: BodyBounds,
        M: ModelBounds,
    {
        let request = request.as_http_request("application/json")?;
        let mut stream = self.send(request).await?;
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
        self.send(request).await
    }
}
