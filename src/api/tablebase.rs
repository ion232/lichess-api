use crate::client::LichessApi;
use crate::error::Result;
use crate::model::tablebase::*;

impl LichessApi<reqwest::Client> {
    pub async fn lookup_antichess(
        &self,
        request: impl Into<antichess::GetRequest>,
    ) -> Result<TablebaseJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn lookup_atomic(
        &self,
        request: impl Into<atomic::GetRequest>,
    ) -> Result<TablebaseJson> {
        self.get_single_model(request.into()).await
    }

    pub async fn lookup_standard(
        &self,
        request: impl Into<standard::GetRequest>,
    ) -> Result<TablebaseJson> {
        self.get_single_model(request.into()).await
    }
}
