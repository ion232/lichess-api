pub mod messaging;
pub mod puzzles;

use async_std::stream::StreamExt;

use crate::client::LichessApi;
use crate::model::{Request, BodyBounds, ModelBounds, QueryBounds};
use crate::error::Result;

impl<'a> LichessApi<'a, reqwest::Client> {
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
