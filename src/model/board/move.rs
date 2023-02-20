use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery {
    pub offering_draw: bool,
}

pub type PostRequest = crate::model::Request<PostQuery>;

impl PostRequest {
    pub fn new(game_id: &str, r#move: &str, offering_draw: bool) -> Self {
        let path = format!("/api/board/game/{}/move/{}", game_id, r#move);
        Self {
            method: http::Method::POST,
            path,
            query: Some(PostQuery { offering_draw }),
            body: Default::default(),
        }
    }
}
