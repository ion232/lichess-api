use crate::model::{Body, Request, VariantKey};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct UpdateArenaTournamentForm {
    pub name: Option<String>,
    #[serde(rename = "clockTime")]
    pub clock_time: f64,
    #[serde(rename = "clockIncrement")]
    pub clock_increment: u32,
    pub minutes: u32,
    #[serde(rename = "waitMinutes")]
    pub wait_minutes: Option<u32>,
    #[serde(rename = "startDate")]
    pub start_date: Option<i64>,
    pub variant: Option<VariantKey>,
    pub rated: Option<bool>,
    pub position: Option<String>,
    pub berserkable: Option<bool>,
    pub streakable: Option<bool>,
    #[serde(rename = "hasChat")]
    pub has_chat: Option<bool>,
    pub description: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "conditions.minRating.rating")]
    pub conditions_min_rating: Option<u32>,
    #[serde(rename = "conditions.maxRating.rating")]
    pub conditions_max_rating: Option<u32>,
    #[serde(rename = "conditions.nbRatedGame.nb")]
    pub conditions_nb_rated_game: Option<u32>,
    #[serde(rename = "conditions.allowList")]
    pub conditions_allow_list: Option<String>,
    #[serde(rename = "conditions.bots")]
    pub conditions_bots: Option<bool>,
    #[serde(rename = "conditions.accountAge")]
    pub conditions_account_age: Option<u32>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, UpdateArenaTournamentForm>;

impl PostRequest {
    pub fn new(id: &str, form: UpdateArenaTournamentForm) -> Self {
        Self::post(
            format!("/api/tournament/{id}"),
            None,
            Body::Form(form),
            None,
        )
    }
}
