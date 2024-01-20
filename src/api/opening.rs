use crate::client::LichessApi;
use crate::error::Result;
use crate::model::opening::*;

use self::masters::OpeningExplorerJson;

impl LichessApi<reqwest::Client> {
    pub async fn masters_openings(&self, request: masters::GetRequest) -> Result<OpeningExplorerJson> {
        self.get_single_model(request).await
    }
}
