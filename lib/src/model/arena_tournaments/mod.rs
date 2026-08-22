pub mod create;
pub mod created_by_user;
pub mod current;
pub mod games;
pub mod join;
pub mod played_by_user;
pub mod results;
pub mod show;
pub mod team_battle;
pub mod teams;
pub mod terminate;
pub mod update;
pub mod withdraw;

use crate::model::{
    ArenaMinRatedGames, ArenaRatingObj, ArenaSchedule, ArenaTournament, LightUser, PerfType, Title,
    Verdicts,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaTournaments {
    pub created: Vec<ArenaTournament>,
    pub started: Vec<ArenaTournament>,
    pub finished: Vec<ArenaTournament>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArenaTournamentFull {
    pub id: String,
    pub full_name: String,
    pub rated: Option<bool>,
    pub spotlight: Option<ArenaSpotlight>,
    pub berserkable: Option<bool>,
    pub only_titled: Option<bool>,
    pub clock: crate::model::ArenaClock,
    pub minutes: u32,
    pub created_by: Option<String>,
    pub system: Option<String>,
    pub seconds_to_start: Option<i32>,
    pub seconds_to_finish: Option<i32>,
    pub is_finished: Option<bool>,
    pub is_recently_finished: Option<bool>,
    pub pairings_closed: Option<bool>,
    pub starts_at: Option<String>,
    pub nb_players: u32,
    pub verdicts: Option<Verdicts>,
    pub quote: Option<ArenaQuote>,
    pub great_player: Option<ArenaGreatPlayer>,
    pub allow_list: Option<Vec<String>>,
    pub has_max_rating: Option<bool>,
    pub max_rating: Option<ArenaRatingObj>,
    pub min_rating: Option<ArenaRatingObj>,
    pub min_rated_games: Option<ArenaMinRatedGames>,
    pub bots_allowed: Option<bool>,
    pub min_account_age_in_days: Option<i32>,
    pub perf: Option<ArenaFullPerf>,
    pub schedule: Option<ArenaSchedule>,
    pub description: Option<String>,
    pub variant: Option<String>,
    pub duels: Option<Vec<ArenaDuel>>,
    pub standing: Option<ArenaStanding>,
    pub featured: Option<ArenaFeaturedGame>,
    pub podium: Option<Vec<ArenaPodiumPlayer>>,
    pub stats: Option<ArenaStats>,
    pub my_username: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaSpotlight {
    pub headline: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaQuote {
    pub text: Option<String>,
    pub author: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaGreatPlayer {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaFullPerf {
    pub icon: Option<String>,
    pub key: PerfType,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaDuel {
    pub id: Option<String>,
    pub p: Option<Vec<ArenaDuelPlayer>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaDuelPlayer {
    pub n: Option<String>,
    pub r: Option<i32>,
    pub k: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaStanding {
    pub page: Option<i32>,
    pub players: Vec<ArenaStandingPlayer>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaStandingPlayer {
    pub name: String,
    pub title: Option<Title>,
    pub patron: Option<bool>,
    #[serde(rename = "patronColor")]
    pub patron_color: Option<u8>,
    pub flair: Option<String>,
    pub rank: Option<i32>,
    pub rating: Option<i32>,
    pub score: Option<i32>,
    pub sheet: Option<ArenaSheet>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaSheet {
    pub scores: String,
    pub fire: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaFeaturedGame {
    pub id: Option<String>,
    pub fen: Option<String>,
    pub orientation: Option<String>,
    pub color: Option<String>,
    #[serde(rename = "lastMove")]
    pub last_move: Option<String>,
    pub white: Option<ArenaFeaturedPlayer>,
    pub black: Option<ArenaFeaturedPlayer>,
    pub c: Option<ArenaFeaturedClock>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaFeaturedPlayer {
    pub name: Option<String>,
    pub id: Option<String>,
    pub rank: Option<i32>,
    pub rating: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaFeaturedClock {
    pub white: Option<i32>,
    pub black: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaPodiumPlayer {
    pub name: String,
    pub title: Option<Title>,
    pub patron: Option<bool>,
    #[serde(rename = "patronColor")]
    pub patron_color: Option<u8>,
    pub flair: Option<String>,
    pub rank: Option<i32>,
    pub rating: Option<i32>,
    pub score: Option<i32>,
    pub nb: Option<ArenaPodiumNb>,
    pub performance: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaPodiumNb {
    pub game: Option<i32>,
    pub berserk: Option<i32>,
    pub win: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArenaStats {
    pub games: i32,
    pub moves: i32,
    pub white_wins: i32,
    pub black_wins: i32,
    pub draws: i32,
    pub berserks: i32,
    pub average_rating: i32,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaResult {
    pub rank: i32,
    pub score: i32,
    pub rating: i32,
    pub username: String,
    pub performance: i32,
    pub title: Option<Title>,
    pub team: Option<String>,
    pub flair: Option<String>,
    #[serde(rename = "patronColor")]
    pub patron_color: Option<u8>,
    pub sheet: Option<ArenaSheet>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaTeamStanding {
    pub id: String,
    pub teams: Vec<ArenaTeamStandingEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaTeamStandingEntry {
    pub rank: i32,
    pub id: String,
    pub score: i32,
    pub players: Vec<ArenaTeamStandingPlayer>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaTeamStandingPlayer {
    pub user: LightUser,
    pub score: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaTournamentPlayed {
    pub tournament: ArenaTournament,
    pub player: ArenaTournamentPlayer,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaTournamentPlayer {
    pub games: i32,
    pub score: i32,
    pub rank: i32,
    pub performance: Option<i32>,
}
