use crate::client::LichessApi;
use crate::error::Result;
use crate::model::tablebase::*;

impl LichessApi<reqwest::Client> {
    pub async fn lookup_antichess(&self, request: antichess::GetRequest) -> Result<TablebaseJson> {
        self.get_single_model(request).await
    }

    pub async fn lookup_atomic(&self, request: atomic::GetRequest) -> Result<TablebaseJson> {
        self.get_single_model(request).await
    }

    pub async fn lookup_standard(&self, request: standard::GetRequest) -> Result<TablebaseJson> {
        self.get_single_model(request).await
    }
}
