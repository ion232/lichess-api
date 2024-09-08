use crate::client::LichessApi;
use crate::error::Result;
use crate::model::users::*;

impl LichessApi<reqwest::Client> {
    pub async fn get_public_user_data(
        &self,
        request: impl Into<public::GetRequest>,
    ) -> Result<UserExtended> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_status_of_users(
        &self,
        request: impl Into<status::GetRequest>,
    ) -> Result<Vec<status::User>> {
        self.get_single_model(request.into()).await
    }
}
