use crate::model::{Body, Request, VariantKey};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct CreateSwissTournamentForm {
    pub name: Option<String>,
    #[serde(rename = "clock.limit")]
    pub clock_limit: u32,
    #[serde(rename = "clock.increment")]
    pub clock_increment: u32,
    #[serde(rename = "nbRounds")]
    pub nb_rounds: u32,
    #[serde(rename = "startsAt")]
    pub starts_at: Option<i64>,
    #[serde(rename = "roundInterval")]
    pub round_interval: Option<i32>,
    pub variant: Option<VariantKey>,
    pub position: Option<String>,
    pub description: Option<String>,
    pub rated: Option<bool>,
    pub password: Option<String>,
    #[serde(rename = "forbiddenPairings")]
    pub forbidden_pairings: Option<String>,
    #[serde(rename = "manualPairings")]
    pub manual_pairings: Option<String>,
    #[serde(rename = "chatFor")]
    pub chat_for: Option<u8>,
    #[serde(rename = "conditions.minRating.rating")]
    pub conditions_min_rating: Option<u32>,
    #[serde(rename = "conditions.maxRating.rating")]
    pub conditions_max_rating: Option<u32>,
    #[serde(rename = "conditions.nbRatedGame.nb")]
    pub conditions_nb_rated_game: Option<u32>,
    #[serde(rename = "conditions.playYourGames")]
    pub conditions_play_your_games: Option<bool>,
    #[serde(rename = "conditions.allowList")]
    pub conditions_allow_list: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, CreateSwissTournamentForm>;

impl PostRequest {
    pub fn new(team_id: &str, form: CreateSwissTournamentForm) -> Self {
        Self::post(
            format!("/api/swiss/new/{team_id}"),
            None,
            Body::Form(form),
            None,
        )
    }
}
