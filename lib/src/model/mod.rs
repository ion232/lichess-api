pub mod account;
pub mod analysis;
pub mod board;
pub mod bot;
pub mod challenges;
pub mod external_engine;
pub mod fide;
pub mod games;
pub mod messaging;
pub mod oauth;
pub mod openings;
pub mod puzzles;
pub mod relations;
pub mod simuls;
pub mod studies;
pub mod tablebase;
pub mod teams;
pub mod tv;
pub mod users;

use crate::error;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_with::skip_serializing_none;

pub trait BodyBounds: Serialize {}
impl<B: Serialize> BodyBounds for B {}

pub trait QueryBounds: Serialize + Default {}
impl<Q: Serialize + Default> QueryBounds for Q {}

pub trait ModelBounds: DeserializeOwned {}
impl<M: DeserializeOwned> ModelBounds for M {}

#[derive(Default, Clone, Debug)]
pub enum Body<B: BodyBounds> {
    Form(B),
    Json(B),
    PlainText(String),
    #[default]
    Empty,
}

impl<B: BodyBounds> Body<B> {
    fn as_mime(&self) -> Option<mime::Mime> {
        match &self {
            Body::Form(_) => Some(mime::APPLICATION_WWW_FORM_URLENCODED),
            Body::Json(_) => Some(mime::APPLICATION_JSON),
            Body::PlainText(_) => Some(mime::TEXT_PLAIN),
            Body::Empty => None,
        }
    }

    fn as_encoded_string(&self) -> error::Result<String> {
        let body = match &self {
            Body::Form(form) => to_form_string(&form)?,
            Body::Json(json) => to_json_string(&json)?,
            Body::PlainText(text) => text.to_string(),
            Body::Empty => "".to_string(),
        };
        Ok(body)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Domain {
    #[default]
    Lichess,
    Tablebase,
    Engine,
    Explorer,
}

impl AsRef<str> for Domain {
    fn as_ref(&self) -> &str {
        match self {
            Domain::Lichess => "lichess.org",
            Domain::Tablebase => "tablebase.lichess.ovh",
            Domain::Engine => "engine.lichess.ovh",
            Domain::Explorer => "explorer.lichess.ovh",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Request<Q, B = ()>
where
    Q: QueryBounds,
    B: BodyBounds,
{
    pub(crate) domain: Domain,
    pub(crate) method: http::Method,
    pub(crate) path: String,
    pub(crate) query: Option<Q>,
    pub(crate) body: Body<B>,
}

impl<Q, B> Request<Q, B>
where
    Q: QueryBounds + Default,
    B: BodyBounds,
{
    pub(crate) fn create(
        path: impl Into<String>,
        query: impl Into<Option<Q>>,
        body: impl Into<Option<Body<B>>>,
        domain: impl Into<Option<Domain>>,
        method: http::Method,
    ) -> Self {
        Self {
            domain: domain.into().unwrap_or_default(),
            method,
            path: path.into(),
            query: query.into(),
            body: body.into().unwrap_or_default(),
        }
    }

    pub(crate) fn get(
        path: impl Into<String>,
        query: impl Into<Option<Q>>,
        domain: impl Into<Option<Domain>>,
    ) -> Self {
        Self::create(path, query, None, domain, http::Method::GET)
    }

    pub(crate) fn post(
        path: impl Into<String>,
        query: impl Into<Option<Q>>,
        body: impl Into<Option<Body<B>>>,
        domain: impl Into<Option<Domain>>,
    ) -> Self {
        Self::create(path, query, body, domain, http::Method::POST)
    }

    pub(crate) fn put(
        path: impl Into<String>,
        query: impl Into<Option<Q>>,
        body: impl Into<Option<Body<B>>>,
        domain: impl Into<Option<Domain>>,
    ) -> Self {
        Self::create(path, query, body, domain, http::Method::PUT)
    }

    pub(crate) fn delete(
        path: impl Into<String>,
        query: impl Into<Option<Q>>,
        body: impl Into<Option<Body<B>>>,
        domain: Option<Domain>,
    ) -> Self {
        Self::create(path, query, body, domain, http::Method::DELETE)
    }
}

impl<Q, B> Request<Q, B>
where
    Q: QueryBounds,
    B: BodyBounds,
{
    pub(crate) fn as_http_request(
        self,
        accept: &str,
    ) -> error::Result<http::Request<bytes::Bytes>> {
        make_request(
            self.domain,
            self.method,
            self.path,
            self.query,
            self.body,
            accept,
        )
    }
}

fn make_request<Q, B>(
    domain: Domain,
    method: http::Method,
    path: String,
    query: Option<Q>,
    body: Body<B>,
    accept: &str,
) -> error::Result<http::Request<bytes::Bytes>>
where
    Q: QueryBounds,
    B: BodyBounds,
{
    let mut builder = http::Request::builder();

    if let Some(mime) = body.as_mime() {
        builder = builder.header(http::header::CONTENT_TYPE, mime.to_string());
    }
    let accept_header = http::HeaderValue::from_str(accept)
        .map_err(|e| error::Error::HttpRequestBuilder(http::Error::from(e)))?;
    builder = builder.header(http::header::ACCEPT, accept_header);

    let url = make_url(domain, path, query)?;
    let body = bytes::Bytes::from(body.as_encoded_string()?);

    let request = builder
        .method(method)
        .uri(url.as_str())
        .body(body)
        .map_err(|e| error::Error::HttpRequestBuilder(e))?;

    Ok(request)
}

fn make_url<Q>(domain: Domain, path: String, query: Option<Q>) -> error::Result<url::Url>
where
    Q: QueryBounds,
{
    let base_url = format!("https://{}", domain.as_ref());
    let mut url = url::Url::parse(&base_url).expect("invalid base url");

    if let Some(query) = query {
        let mut query_pairs = url.query_pairs_mut();
        let query_serializer = serde_urlencoded::Serializer::new(&mut query_pairs);
        query.serialize(query_serializer)?;
    }

    url.set_path(&path.to_string());

    Ok(url)
}

fn to_json_string<B: BodyBounds>(body: &B) -> error::Result<String> {
    serde_json::to_string(&body).map_err(|e| error::Error::Json(e))
}

fn to_form_string<B: BodyBounds>(body: &B) -> error::Result<String> {
    serde_urlencoded::to_string(&body).map_err(|e| error::Error::UrlEncoded(e))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ok {
    pub ok: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response<M> {
    Model(M),
    Error { error: String },
}

#[derive(Default, Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    #[default]
    White,
    Black,
    Random,
}

#[derive(Default, Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlayerColor {
    #[default]
    White,
    Black,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Speed {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
pub enum PerfType {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
}

impl PerfType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UltraBullet => "ultraBullet",
            Self::Bullet => "bullet",
            Self::Blitz => "blitz",
            Self::Rapid => "rapid",
            Self::Classical => "classical",
            Self::Correspondence => "correspondence",
            Self::Chess960 => "chess960",
            Self::Crazyhouse => "crazyhouse",
            Self::Antichess => "antichess",
            Self::Atomic => "atomic",
            Self::Horde => "horde",
            Self::KingOfTheHill => "kingOfTheHill",
            Self::RacingKings => "racingKings",
            Self::ThreeCheck => "threeCheck",
        }
    }
}

impl std::fmt::Display for PerfType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LightUser {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    pub flair: Option<String>,
    pub patron: Option<bool>,
    #[serde(rename = "patronColor")]
    pub patron_color: Option<u8>,
    pub online: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Title {
    Gm,
    Wgm,
    Im,
    Wim,
    Fm,
    Wfm,
    Nm,
    Cm,
    Wcm,
    Wnm,
    Lm,
    Bot,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Variant {
    pub key: VariantKey,
    pub name: String,
    pub short: Option<String>,
    pub icon: Option<String>,
}

#[derive(Default, Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariantKey {
    #[default]
    Standard,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
    FromPosition,
}

#[derive(Default, Clone, Debug, Serialize, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Room {
    #[default]
    Player,
    Spectator,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameCompat {
    pub bot: Option<bool>,
    pub board: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub initial: u32,
    pub increment: u32,
    pub total_time: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Days {
    One,
    Two,
    Three,
    Five,
    Seven,
    Ten,
    Fourteen,
}

impl Serialize for Days {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: u32 = (*self).into();
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Days {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(Days::from(value))
    }
}

impl From<u32> for Days {
    fn from(value: u32) -> Self {
        match value {
            1 => Days::One,
            2 => Days::Two,
            3 => Days::Three,
            5 => Days::Five,
            7 => Days::Seven,
            10 => Days::Ten,
            14 => Days::Fourteen,
            _ => panic!("Invalid days {}", value),
        }
    }
}

impl Into<u32> for Days {
    fn into(self) -> u32 {
        match self {
            Days::One => 1,
            Days::Two => 2,
            Days::Three => 3,
            Days::Five => 5,
            Days::Seven => 7,
            Days::Ten => 10,
            Days::Fourteen => 14,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArenaTournament {
    pub id: String,
    pub created_by: String,
    pub system: String,
    pub minutes: u32,
    pub clock: ArenaClock,
    pub rated: bool,
    pub full_name: String,
    pub nb_players: u32,
    pub variant: Variant,
    pub starts_at: i64,
    pub finishes_at: i64,
    pub status: ArenaStatus,
    pub perf: ArenaPerf,
    pub seconds_to_start: Option<i32>,
    pub has_max_rating: Option<bool>,
    pub max_rating: Option<ArenaRatingObj>,
    pub min_rating: Option<ArenaRatingObj>,
    pub min_rated_games: Option<ArenaMinRatedGames>,
    pub bots_allowed: Option<bool>,
    pub min_account_age_in_days: Option<i32>,
    pub only_titled: Option<bool>,
    pub team_member: Option<String>,
    pub private: Option<bool>,
    pub position: Option<ArenaPosition>,
    pub schedule: Option<ArenaSchedule>,
    pub team_battle: Option<ArenaTeamBattle>,
    pub winner: Option<LightUser>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaClock {
    pub limit: u32,
    pub increment: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArenaStatus {
    Created,
    Started,
    Finished,
}

impl Serialize for ArenaStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: u8 = match self {
            ArenaStatus::Created => 10,
            ArenaStatus::Started => 20,
            ArenaStatus::Finished => 30,
        };
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ArenaStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            10 => Ok(ArenaStatus::Created),
            20 => Ok(ArenaStatus::Started),
            30 => Ok(ArenaStatus::Finished),
            _ => Err(serde::de::Error::custom(format!(
                "invalid arena tournament status {value}"
            ))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArenaStatusName {
    Created,
    Started,
    Finished,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaPerf {
    pub key: PerfType,
    pub name: String,
    pub position: i32,
    pub icon: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaRatingObj {
    pub perf: Option<PerfType>,
    pub rating: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaMinRatedGames {
    pub nb: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ArenaPosition {
    Thematic {
        eco: String,
        name: String,
        fen: String,
        url: String,
    },
    Custom {
        name: String,
        fen: String,
    },
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaSchedule {
    pub freq: Option<String>,
    pub speed: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArenaTeamBattle {
    pub teams: Option<Vec<String>>,
    pub nb_leaders: Option<i32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwissTournament {
    pub id: String,
    pub created_by: String,
    pub starts_at: String,
    pub name: String,
    pub clock: SwissClock,
    pub variant: VariantKey,
    pub round: i32,
    pub nb_rounds: i32,
    pub nb_players: i32,
    pub nb_ongoing: i32,
    pub status: SwissStatus,
    pub stats: Option<SwissStats>,
    pub rated: bool,
    pub verdicts: Verdicts,
    pub next_round: Option<SwissNextRound>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SwissClock {
    pub limit: i32,
    pub increment: i32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SwissStatus {
    Created,
    Started,
    Finished,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwissStats {
    pub games: i32,
    pub white_wins: i32,
    pub black_wins: i32,
    pub draws: i32,
    pub byes: i32,
    pub absences: i32,
    pub average_rating: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SwissNextRound {
    pub at: String,
    #[serde(rename = "in")]
    pub in_seconds: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verdicts {
    pub accepted: bool,
    pub list: Vec<Verdict>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verdict {
    pub condition: String,
    pub verdict: String,
}
