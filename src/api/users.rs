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

    pub async fn get_rating_history(
        &self,
        request: impl Into<rating_history::GetRequest>,
    ) -> Result<rating_history::RatingHistory> {
        self.get_single_model(request.into()).await
    }

    /// Get performance statistics of a user.
    pub async fn get_user_performance_statistics(
        &self,
        request: impl Into<performance::GetRequest>,
    ) -> Result<performance::Performance> {
        self.get_single_model(request.into()).await
    }

    pub async fn get_users_by_id(
        &self,
        request: impl Into<by_id::PostRequest>,
    ) -> Result<Vec<User>> {
        self.get_single_model(request.into()).await
    }
}
