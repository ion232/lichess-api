use crate::model::{Color, Days, VariantKey};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery {
    #[serde(flatten)]
    pub seek_type: SeekType,
    pub rated: bool,
    pub variant: VariantKey,
    pub color: Color,
    pub rating_range: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum SeekType {
    Correspondence { days: Days },
    RealTime { time: u32, increment: u32 },
}

impl Default for PostQuery {
    fn default() -> Self {
        Self {
            rated: false,
            seek_type: SeekType::RealTime {
                time: 5,
                increment: 3,
            },
            variant: VariantKey::Standard,
            color: Color::Random,
            rating_range: "".to_string(),
        }
    }
}

pub type PostRequest = crate::model::Request<PostQuery>;

impl PostRequest {
    pub fn new() -> Self {
        PostQuery::default().into()
    }
}

impl From<PostQuery> for PostRequest {
    fn from(query: PostQuery) -> Self {
        Self::post("/api/board/seek", query, None, None)
    }
}

impl Default for PostRequest {
    fn default() -> Self {
        PostQuery::default().into()
    }
}
