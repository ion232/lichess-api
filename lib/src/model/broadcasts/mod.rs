pub mod create_round;
pub mod create_tournament;
pub mod export_pgn;
pub mod export_round_pgn;
pub mod get_player;
pub mod get_players;
pub mod get_round;
pub mod get_team_standings;
pub mod get_tournament;
pub mod list_by_user;
pub mod list_my_rounds;
pub mod list_official;
pub mod push_pgn;
pub mod reset_round;
pub mod search;
pub mod stream_group_pgn;
pub mod stream_round_pgn;
pub mod stream_tournament_pgn;
pub mod top;
pub mod update_round;
pub mod update_tournament;

use crate::model::{LightUser, PlayerColor, Title};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct PgnStreamQuery {
    pub clocks: Option<bool>,
    pub comments: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTour {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub created_at: i64,
    pub dates: Option<Vec<i64>>,
    pub info: Option<BroadcastTourInfo>,
    pub tier: Option<i32>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub team_table: Option<bool>,
    pub show_team_scores: Option<bool>,
    pub url: String,
    pub community_owner: Option<LightUser>,
}

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTourInfo {
    pub format: Option<String>,
    pub tc: Option<String>,
    #[serde(rename = "fideTC")]
    pub fide_tc: Option<FideTimeControl>,
    pub time_zone: Option<String>,
    pub location: Option<String>,
    pub players: Option<String>,
    pub website: Option<String>,
    pub standings: Option<String>,
    pub regulations: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FideTimeControl {
    Standard,
    Rapid,
    Blitz,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastRoundInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub created_at: i64,
    pub rated: Option<bool>,
    pub ongoing: Option<bool>,
    pub starts_at: Option<i64>,
    pub starts_after_previous: Option<bool>,
    pub finished_at: Option<i64>,
    #[deprecated]
    pub finished: Option<bool>,
    pub url: String,
    pub delay: Option<i64>,
    pub custom_scoring: Option<BroadcastCustomScoring>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastCustomScoring {
    pub white: BroadcastCustomPointsPerColor,
    pub black: BroadcastCustomPointsPerColor,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastCustomPointsPerColor {
    pub win: f64,
    pub draw: f64,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastRoundStudyInfo {
    pub writeable: Option<bool>,
    pub features: Option<BroadcastRoundStudyFeatures>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastRoundStudyFeatures {
    pub chat: Option<bool>,
    pub computer: Option<bool>,
    pub explorer: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastRoundGame {
    pub id: String,
    pub name: String,
    pub fen: Option<String>,
    pub players: Option<Vec<BroadcastRoundGamePlayer>>,
    pub last_move: Option<String>,
    pub check: Option<String>,
    pub think_time: Option<i32>,
    pub status: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastRoundGamePlayer {
    pub name: Option<String>,
    pub title: Option<Title>,
    pub rating: Option<i32>,
    pub fide_id: Option<i32>,
    pub fed: Option<String>,
    pub clock: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastGroup {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub tours: Vec<BroadcastGroupTour>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastGroupTour {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub live: bool,
}

pub type BroadcastPhotos = HashMap<String, BroadcastPhoto>;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastPhoto {
    pub small: String,
    pub medium: String,
    pub credit: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastRound {
    pub round: BroadcastRoundInfo,
    pub tour: BroadcastTour,
    pub study: BroadcastRoundStudyInfo,
    pub games: Vec<BroadcastRoundGame>,
    pub group: Option<BroadcastGroup>,
    pub is_subscribed: Option<bool>,
    pub photos: BroadcastPhotos,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastRoundNew {
    pub round: BroadcastRoundInfo,
    pub tour: BroadcastTour,
    pub study: BroadcastRoundStudyInfo,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastMyRound {
    pub round: BroadcastRoundInfo,
    pub tour: BroadcastTour,
    pub study: BroadcastRoundStudyInfo,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastWithRounds {
    pub tour: BroadcastTour,
    pub group: Option<String>,
    pub rounds: Vec<BroadcastRoundInfo>,
    pub default_round_id: Option<String>,
    pub photos: Option<BroadcastPhotos>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastWithRoundsAndFullGroup {
    pub tour: BroadcastTour,
    pub group: Option<BroadcastGroup>,
    pub rounds: Vec<BroadcastRoundInfo>,
    pub default_round_id: Option<String>,
    pub photos: Option<BroadcastPhotos>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastWithLastRound {
    pub group: Option<String>,
    pub tour: Option<BroadcastTour>,
    pub round: Option<BroadcastRoundInfo>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastByUser {
    pub tour: BroadcastTour,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTop {
    pub active: Vec<BroadcastWithLastRound>,
    pub past: BroadcastPaginator<BroadcastWithLastRound>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastPaginator<T> {
    pub current_page: u32,
    pub max_per_page: u32,
    pub current_page_results: Vec<T>,
    pub previous_page: Option<u32>,
    pub next_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_results: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_pages: Option<u32>,
}

pub type BroadcastByUserPaginator = BroadcastPaginator<BroadcastByUser>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastSearchResult {
    pub tour: BroadcastTour,
    pub round: BroadcastRoundInfo,
}

pub type BroadcastSearchPaginator = BroadcastPaginator<BroadcastSearchResult>;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum BroadcastPointStr {
    #[serde(rename = "1")]
    Win,
    #[serde(rename = "1/2")]
    Draw,
    #[serde(rename = "0")]
    Loss,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastPlayerWithFed {
    pub name: String,
    pub title: Option<Title>,
    pub rating: Option<i32>,
    pub fide_id: Option<i64>,
    pub team: Option<String>,
    pub fed: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatByFideTC {
    pub standard: Option<i32>,
    pub rapid: Option<i32>,
    pub blitz: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastPlayerTiebreak {
    pub extended_code: BroadcastTiebreakExtendedCode,
    pub description: String,
    pub points: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum BroadcastTiebreakExtendedCode {
    #[serde(rename = "AOB")]
    Aob,
    #[serde(rename = "APPO")]
    Appo,
    #[serde(rename = "APRO")]
    Apro,
    #[serde(rename = "ARO")]
    Aro,
    #[serde(rename = "ARO-C1")]
    AroC1,
    #[serde(rename = "ARO-C2")]
    AroC2,
    #[serde(rename = "ARO-M1")]
    AroM1,
    #[serde(rename = "ARO-M2")]
    AroM2,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BH-C1")]
    BhC1,
    #[serde(rename = "BH-C2")]
    BhC2,
    #[serde(rename = "BH-M1")]
    BhM1,
    #[serde(rename = "BH-M2")]
    BhM2,
    #[serde(rename = "BPG")]
    Bpg,
    #[serde(rename = "BWG")]
    Bwg,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "FB")]
    Fb,
    #[serde(rename = "FB-C1")]
    FbC1,
    #[serde(rename = "FB-C2")]
    FbC2,
    #[serde(rename = "FB-M1")]
    FbM1,
    #[serde(rename = "FB-M2")]
    FbM2,
    #[serde(rename = "KS")]
    Ks,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PS-C1")]
    PsC1,
    #[serde(rename = "PS-C2")]
    PsC2,
    #[serde(rename = "PS-M1")]
    PsM1,
    #[serde(rename = "PS-M2")]
    PsM2,
    #[serde(rename = "PTP")]
    Ptp,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SB-C1")]
    SbC1,
    #[serde(rename = "SB-C2")]
    SbC2,
    #[serde(rename = "SB-M1")]
    SbM1,
    #[serde(rename = "SB-M2")]
    SbM2,
    #[serde(rename = "TPR")]
    Tpr,
    #[serde(rename = "WON")]
    Won,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastPlayerEntry {
    pub name: String,
    pub title: Option<Title>,
    pub rating: Option<i32>,
    pub fide_id: Option<i64>,
    pub team: Option<String>,
    pub fed: Option<String>,
    pub score: Option<f64>,
    pub played: Option<i32>,
    pub rating_diffs: Option<StatByFideTC>,
    pub ratings_map: Option<StatByFideTC>,
    pub performances: Option<StatByFideTC>,
    pub tiebreaks: Option<Vec<BroadcastPlayerTiebreak>>,
    pub rank: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastPlayerFideInfo {
    pub year: Option<i32>,
    pub ratings: Option<StatByFideTC>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastPlayerEntryWithFideAndGames {
    pub name: String,
    pub title: Option<Title>,
    pub rating: Option<i32>,
    pub fide_id: Option<i64>,
    pub team: Option<String>,
    pub fed: Option<String>,
    pub score: Option<f64>,
    pub played: Option<i32>,
    pub rating_diffs: Option<StatByFideTC>,
    pub ratings_map: Option<StatByFideTC>,
    pub performances: Option<StatByFideTC>,
    pub tiebreaks: Option<Vec<BroadcastPlayerTiebreak>>,
    pub rank: Option<i32>,
    pub fide: Option<BroadcastPlayerFideInfo>,
    pub games: Option<Vec<BroadcastGameEntry>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastGameEntry {
    pub round: String,
    pub id: String,
    pub opponent: BroadcastPlayerWithFed,
    pub color: PlayerColor,
    pub points: Option<BroadcastPointStr>,
    pub custom_points: Option<f64>,
    pub rating_diff: Option<i32>,
    #[serde(rename = "fideTC")]
    pub fide_tc: FideTimeControl,
    pub ongoing: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTeamLeaderboardEntry {
    pub name: String,
    pub mp: f64,
    pub gp: f64,
    pub average_rating: Option<i32>,
    pub matches: Vec<BroadcastTeamPovMatchEntry>,
    pub players: Vec<BroadcastPlayerEntry>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastTeamPovMatchEntry {
    pub round_id: String,
    pub opponent: String,
    pub mp: Option<f64>,
    pub gp: Option<f64>,
    pub points: Option<BroadcastPointStr>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastPgnPush {
    pub games: Vec<BroadcastPgnPushGame>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BroadcastPgnPushGame {
    pub tags: HashMap<String, String>,
    pub moves: Option<i32>,
    pub error: Option<String>,
}
