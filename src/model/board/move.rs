use serde::Serialize;

#[derive(Default, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery {
    pub offering_draw: bool,
}

pub type PostRequest = crate::model::Request<PostQuery>;

impl PostRequest {
    pub fn new(game_id: &str, r#move: &str, offering_draw: bool) -> Self {
        let path = format!("/api/board/game/{game_id}/move/{move}");
        Self::post(path, PostQuery { offering_draw }, None, None)
    }
}
