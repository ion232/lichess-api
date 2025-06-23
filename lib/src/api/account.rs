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
}
