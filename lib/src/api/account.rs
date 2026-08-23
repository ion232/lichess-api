//! Read and manage the logged in user's own account: public profile, email
//! address, kid mode, preferences, and activity timeline.
//!
//! Every method here requires a bearer token for the account being queried —
//! there is no anonymous access. [`LichessApi::get_email_address`] and
//! [`LichessApi::get_kid_mode_status`]/[`LichessApi::set_kid_mode_status`]
//! additionally require the `email:read` and `preference:read` (or
//! `preference:write`) scopes respectively, not just any token.

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::account::*;

impl LichessApi<reqwest::Client> {
    pub async fn get_profile(&self) -> Result<profile::Profile> {
        self.get_single_model(profile::GetRequest::new()).await
    }

    pub async fn get_email_address(&self) -> Result<email::Email> {
        self.get_single_model(email::GetRequest::new()).await
    }

    pub async fn get_preferences(&self) -> Result<preferences::UserPreferences> {
        self.get_single_model(preferences::GetRequest::new()).await
    }

    pub async fn get_kid_mode_status(&self) -> Result<kid::KidMode> {
        self.get_single_model(kid::GetRequest::new()).await
    }

    pub async fn set_kid_mode_status(&self, request: impl Into<kid::PostRequest>) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn get_timeline(
        &self,
        request: impl Into<timeline::GetRequest>,
    ) -> Result<timeline::Timeline> {
        self.get_single_model(request.into()).await
    }
}
