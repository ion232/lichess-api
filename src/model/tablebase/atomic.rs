use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    fen: String,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(fen: &str) -> Self {
        Self {
            domain: crate::model::Domain::Tablebase,
            path: "/atomic".to_string(),
            query: Some(GetQuery {
                fen: fen.to_string(),
            }),
            ..Default::default()
        }
    }
}
