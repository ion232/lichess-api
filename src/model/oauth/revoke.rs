use crate::model::Request;
use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct DeleteQuery;

pub type DeleteRequest = Request<DeleteQuery>;

impl DeleteRequest {
    pub fn new() -> Self {
        let path = format!("/api/token");
        Self::delete(path, None, None, None)
    }
}
