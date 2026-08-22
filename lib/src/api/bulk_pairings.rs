use futures::stream::StreamExt;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::bulk_pairings::*;
use crate::model::games::GameJson;

impl LichessApi<reqwest::Client> {
    pub async fn get_bulk_pairings(&self) -> Result<Vec<BulkPairing>> {
        self.get_single_model(list::GetRequest::new()).await
    }

    pub async fn create_bulk_pairing(
        &self,
        form: create::CreateBulkPairingForm,
    ) -> Result<BulkPairing> {
        self.get_single_model(create::PostRequest::new(form)).await
    }

    pub async fn get_bulk_pairing(
        &self,
        request: impl Into<show::GetRequest>,
    ) -> Result<BulkPairing> {
        self.get_single_model(request.into()).await
    }

    pub async fn cancel_bulk_pairing(
        &self,
        request: impl Into<remove::DeleteRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }

    pub async fn export_bulk_pairing_games(
        &self,
        id: &str,
        query: games::GetQuery,
    ) -> Result<impl StreamExt<Item = Result<GameJson>>> {
        self.get_streamed_models(games::GetRequest::new(id, query))
            .await
    }

    pub async fn start_bulk_pairing_clocks(
        &self,
        request: impl Into<start_clocks::PostRequest>,
    ) -> Result<bool> {
        self.get_ok(request.into()).await
    }
}
