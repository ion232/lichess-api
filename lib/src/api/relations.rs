//! Follow, unfollow, and block other Lichess players.
//!
//! [`LichessApi::get_following`] streams the users the authenticated account
//! follows; the request/response types live in [`crate::model::relations`].
//! All operations here act on behalf of the authenticated user and require
//! the `follow:read` or `follow:write` OAuth scope, as appropriate.

use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::relations::*;
use crate::model::users::UserExtended;

impl LichessApi<reqwest::Client> {
    pub async fn get_following(
        &self,
        request: impl Into<following::GetRequest>,
    ) -> Result<impl StreamExt<Item = Result<UserExtended>>> {
        self.get_streamed_models(request.into()).await
    }

    pub async fn follow_user(&self, request: impl Into<follow::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn unfollow_user(&self, request: impl Into<unfollow::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn block_user(&self, request: impl Into<block::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn unblock_user(&self, request: impl Into<unblock::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }
}
