use crate::client::LichessApi;
use crate::error::Result;
use crate::model::users::*;

impl<'a> LichessApi<'a, reqwest::Client> {
    pub async fn get_public_user_data(&self, request: public::GetRequest) -> Result<UserExtended> {
        self.get_single_model(request).await
    }

    pub async fn get_status_of_users(
        &self,
        request: status::GetRequest,
    ) -> Result<Vec<status::User>> {
        self.get_single_model(request).await
    }
}
