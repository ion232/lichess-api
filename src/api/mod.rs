pub mod messaging;
pub mod puzzles;

use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::model::{Request, StatusResponse, BodyBounds, ModelBounds, QueryBounds};
use crate::error::{Error, Result};

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn get_status_response<Q, B>(&self, request: Request<Q, B>) -> Result<bool> where
        Q: QueryBounds,
        B: BodyBounds
    {
        let response: StatusResponse = self.get_single_model(request).await?;
        match response {
            StatusResponse::Ok { ok } => Result::Ok(ok),
            StatusResponse::Error { error } => Result::Err(Error::LichessStatus(error)),
            StatusResponse::NotFound { error } => Result::Err(Error::LichessStatus(error))
        }
    }

    pub async fn get_single_model<Q, B, M>(&self, request: Request<Q, B>) -> Result<M> where
        Q: QueryBounds,
        B: BodyBounds,
        M: ModelBounds
    {
        let request = request.as_http_request()?;
        let mut stream = self.send(request).await?;
        self.expect_one_model(&mut stream).await
    }

    pub async fn get_streamed_models<Q, B, M>(&self, request: Request<Q, B>)
    -> Result<impl StreamExt<Item = Result<M>>> where
        Q: QueryBounds,
        B: BodyBounds,
        M: ModelBounds
    {
        let request = request.as_http_request()?;
        self.send(request).await
    }
}
