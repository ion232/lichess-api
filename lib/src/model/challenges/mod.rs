pub mod accept;
pub mod add_time;
pub mod ai;
pub mod cancel;
pub mod create;
pub mod decline;
pub mod list;
pub mod open;
pub mod start_clocks;

use crate::model::{Color, Days, GameCompat, Speed, Title, Variant, VariantKey};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Serialize)]
pub struct OpenChallenge {
    #[serde(flatten)]
    pub base: ChallengeBase,
    pub name: String,
    pub rules: String,
    pub users: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChallenge {
    #[serde(flatten)]
    pub base: ChallengeBase,
    pub rated: bool,
    pub keep_alive_stream: bool,
    pub accept_by_token: Option<String>,
    pub message: Option<String>,
    pub rules: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Rules {
    NoAbort,
    NoRematch,
    NoGiveTime,
    NoClaimWin,
}

#[derive(Clone, Debug, Serialize)]
pub struct AIChallenge {
    #[serde(flatten)]
    pub base: ChallengeBase,
    pub level: u32,
    pub color: Color,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct ChallengeBase {
    #[serde(rename = "clock.limit")]
    pub clock_limit: Option<u32>,
    #[serde(rename = "clock.increment")]
    pub clock_increment: Option<u32>,
    pub days: Option<Days>,
    pub variant: VariantKey,
    pub fen: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeOpenJson {
    pub id: String,
    pub url: String,
    pub status: Status,
    pub challenger: Option<ChallengeUser>,
    pub dest_user: Option<ChallengeUser>,
    pub variant: Variant,
    pub rated: bool,
    pub speed: Speed,
    pub time_control: TimeControl,
    pub color: Color,
    pub final_color: Option<Color>,
    pub perf: Perf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_fen: Option<String>,
    pub url_white: String,
    pub url_black: String,
    pub open: OpenChallengeUsers,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenChallengeUsers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeDeclinedJson {
    #[serde(flatten)]
    pub base: ChallengeJson,
    pub decline_reason: String,
    pub decline_reason_key: DeclineReasonKey,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeclineReasonKey {
    Generic,
    Later,
    #[serde(rename = "toofast")]
    TooFast,
    #[serde(rename = "tooslow")]
    TooSlow,
    #[serde(rename = "timecontrol")]
    TimeControl,
    Rated,
    Casual,
    Standard,
    Variant,
    #[serde(rename = "nobot")]
    NoBot,
    #[serde(rename = "onlybot")]
    OnlyBot,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeJson {
    #[serde(flatten)]
    pub base: ChallengeJsonBase,
    pub initial_fen: Option<String>,
    pub decline_reason: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeJsonBase {
    pub id: String,
    pub url: String,
    pub color: Color,
    pub direction: Option<Direction>,
    pub time_control: TimeControl,
    pub variant: Variant,

    /// The api docs suggest this is non-nullable,
    /// this is true only if the user does not accept anonymous challengers.
    pub challenger: Option<ChallengeUser>,
    pub dest_user: Option<ChallengeUser>,
    pub perf: Perf,
    pub rated: bool,
    pub speed: Speed,
    pub status: Status,
    pub final_color: Option<Color>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    In,
    Out,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Perf {
    pub icon: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Created,
    Offline,
    Canceled,
    Declined,
    Accepted,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeEvent {
    #[serde(rename = "type")]
    pub event_type: String, // Always "challenge"
    pub challenge: ChallengeJson,
    pub compat: Option<GameCompat>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeCanceledEvent {
    #[serde(rename = "type")]
    pub event_type: String, // Always "challengeCanceled"
    pub challenge: ChallengeJson,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeDeclinedEvent {
    #[serde(rename = "type")]
    pub event_type: String, // Always "challengeDeclined"
    pub challenge: ChallengeDeclinedJson,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum TimeControl {
    Clock {
        increment: u32,
        limit: u32,
        show: String,
    },
    Correspondence {
        days_per_turn: u32,
    },
    Unlimited,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChallengeUser {
    pub id: String,
    pub name: String,
    pub rating: u32,
    pub title: Option<Title>,
    pub flair: Option<String>,
    pub patron: Option<bool>,
    pub provisional: Option<bool>,
    pub online: Option<bool>,
    pub lag: Option<u32>,
}
