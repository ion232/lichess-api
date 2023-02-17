use crate::client::LichessApi;
use crate::error::Result;
use crate::model::account::*;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn get_profile(&self, request: profile::GetRequest) -> Result<profile::Profile> {
        self.get_single_model(request).await
    }

    pub async fn get_email_address(&self, request: email::GetRequest) -> Result<email::Email> {
        self.get_single_model(request).await
    }

    pub async fn get_preferences(
        &self,
        request: preferences::GetRequest,
    ) -> Result<preferences::UserPreferences> {
        self.get_single_model(request).await
    }

    pub async fn get_kid_mode_status(&self, request: kid::GetRequest) -> Result<kid::KidMode> {
        self.get_single_model(request).await
    }

    pub async fn set_kid_mode_status(&self, request: kid::PostRequest) -> Result<bool> {
        self.get_ok_or_error_response(request).await
    }
}
