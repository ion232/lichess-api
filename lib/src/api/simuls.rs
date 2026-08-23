//! Simultaneous exhibitions: one host playing many opponents at once.
//!
//! Lists recently created, started, and finished simuls, matching what's shown
//! on <https://lichess.org/simul>. The created/finished lists are not
//! exhaustive — only simuls with a strong enough host are included. This
//! endpoint is public, but when called with a bearer token the pending list is
//! populated with the caller's own created-but-unstarted simuls.

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::simuls::*;

impl LichessApi<reqwest::Client> {
    pub async fn get_current_simuls(&self) -> Result<current::Simuls> {
        self.get_single_model(current::GetRequest::new()).await
    }
}
