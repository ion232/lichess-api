//! Sending private messages to other Lichess players.
//!
//! [`LichessApi::send_message`] posts a message to a user's inbox, on behalf
//! of the authenticated account. It requires the `msg:write` OAuth scope.

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::messaging::*;

impl LichessApi<reqwest::Client> {
    pub async fn send_message(&self, request: impl Into<inbox::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }
}
