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
use crate::model::{BodyBounds, ErrorResponse, ModelBounds, QueryBounds, Request};

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn get_ok_or_error_response<Q, B>(&self, request: Request<Q, B>) -> Result<bool>
    where
        Q: QueryBounds,
        B: BodyBounds,
    {
        let result = self
            .get_model_or_error_response::<Q, B, crate::model::Ok>(request)
            .await;
        return Ok(result?.ok);
    }

    pub async fn get_model_or_error_response<Q, B, M>(&self, request: Request<Q, B>) -> Result<M>
    where
        Q: QueryBounds,
        B: BodyBounds,
        M: ModelBounds,
    {
        let response: ErrorResponse = self.get_single_model(request).await?;
        match response {
            ErrorResponse::Error { error } => Result::Err(Error::LichessStatus(error)),
            ErrorResponse::NotFound { error } => Result::Err(Error::LichessStatus(error)),
            ErrorResponse::Other {} => {
                Result::Err(Error::LichessStatus("Unknown lichess status".to_string()))
            }
        }
    }

    pub async fn get_single_model<Q, B, M>(&self, request: Request<Q, B>) -> Result<M>
    where
        Q: QueryBounds,
        B: BodyBounds,
        M: ModelBounds,
    {
        let request = request.as_http_request()?;
        let mut stream = self.send(request).await?;
        self.expect_one_model(&mut stream).await
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
        let request = request.as_http_request()?;
        self.send(request).await
    }
}
