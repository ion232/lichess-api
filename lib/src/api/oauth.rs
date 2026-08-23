//! Inspect and revoke OAuth2 tokens, and exchange an authorization code for
//! an access token.
//!
//! This module covers the token-facing half of the OAuth2 PKCE flow. For
//! generating the authorization URL and completing the flow end to end, see
//! [`crate::model::oauth::authorize::AuthorizationUrl`] and
//! [`crate::model::oauth::PendingAuthorization`], which both carry a full
//! walkthrough.

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::oauth::*;

impl LichessApi<reqwest::Client> {
    pub async fn test_tokens(&self, request: impl Into<test::PostRequest>) -> Result<TestResults> {
        self.get_single_model(request.into()).await
    }

    pub async fn revoke_token(&self) -> Result<()> {
        self.get_empty(revoke::DeleteRequest::new()).await
    }

    /// Exchange an authorization code for an access token.
    ///
    /// This completes the flow started with
    /// [`crate::model::oauth::authorize::AuthorizationUrl`]. It is the one
    /// endpoint that takes no bearer token, since it is what produces one, so
    /// the client may be built with `LichessApi::new(client, None)`.
    pub async fn obtain_access_token(
        &self,
        request: impl Into<token::PostRequest>,
    ) -> Result<AccessToken> {
        self.get_single_model(request.into()).await
    }
}
