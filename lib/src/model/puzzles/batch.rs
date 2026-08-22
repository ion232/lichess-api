use crate::model::puzzles::{PuzzleAndGame, next::Difficulty};
use crate::model::{Body, Request};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub difficulty: Option<Difficulty>,
    pub nb: Option<u32>,
    pub color: Option<Color>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    White,
    Black,
}

pub type GetRequest = Request<GetQuery>;

impl GetRequest {
    pub fn new(angle: &str, query: GetQuery) -> Self {
        let path = format!("/api/puzzle/batch/{angle}");
        Self::get(path, query, None)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Select {
    pub puzzles: Vec<PuzzleAndGame>,
    pub glicko: Glicko,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Glicko {
    pub rating: f64,
    pub deviation: f64,
    pub provisional: Option<bool>,
}

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery {
    pub nb: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SolveRequest {
    pub solutions: Vec<Solution>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Solution {
    pub id: String,
    pub win: bool,
    pub rated: bool,
}

pub type PostRequest = Request<PostQuery, SolveRequest>;

impl PostRequest {
    pub fn new(angle: &str, query: PostQuery, solutions: SolveRequest) -> Self {
        let path = format!("/api/puzzle/batch/{angle}");
        Self::post(path, query, Body::Json(solutions), None)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SolveResponse {
    pub puzzles: Vec<PuzzleAndGame>,
    pub glicko: Glicko,
    pub rounds: Vec<Round>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    pub id: String,
    pub win: bool,
    pub rating_diff: i32,
}
