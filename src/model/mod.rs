pub mod account;
pub mod board;
pub mod bot;
pub mod challenges;
pub mod games;
pub mod messaging;
pub mod puzzles;
pub mod users;

use crate::error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub trait BodyBounds: Serialize {}
impl<B: Serialize> BodyBounds for B {}

pub trait QueryBounds: Serialize + Default {}
impl<Q: Serialize + Default> QueryBounds for Q {}

pub trait ModelBounds: DeserializeOwned {}
impl<M: DeserializeOwned> ModelBounds for M {}

pub enum Body<B: BodyBounds> {
    Form(B),
    Json(B),
    PlainText(String),
    Empty,
}

impl Default for Body<()> {
    fn default() -> Self {
        Body::Empty
    }
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

pub struct Request<Q, B = ()>
where
    Q: QueryBounds,
    B: BodyBounds,
{
    pub(crate) method: http::Method,
    pub(crate) path: String,
    pub(crate) query: Option<Q>,
    pub(crate) body: Body<B>,
}

impl<Q, B> Request<Q, B>
where
    Q: QueryBounds,
    B: BodyBounds,
{
    pub(crate) fn as_http_request(self) -> error::Result<http::Request<bytes::Bytes>> {
        make_request(self.method, self.path, self.query, self.body)
    }
}

fn make_request<Q, B>(
    method: http::Method,
    path: String,
    query: Option<Q>,
    body: Body<B>,
) -> error::Result<http::Request<bytes::Bytes>>
where
    Q: QueryBounds,
    B: BodyBounds,
{
    let mut builder = http::Request::builder();

    if let Some(mime) = body.as_mime() {
        builder = builder.header(http::header::CONTENT_TYPE, mime.to_string());
    }

    let url = make_url(path, query)?;
    let body = bytes::Bytes::from(body.as_encoded_string()?);

    let request = builder
        .method(method)
        .uri(url.as_str())
        .body(body)
        .map_err(|e| error::Error::HttpRequestBuilder(e))?;

    Ok(request)
}

fn make_url<Q>(path: String, query: Option<Q>) -> error::Result<url::Url>
where
    Q: QueryBounds,
{
    let mut url = url::Url::parse("https://lichess.org").expect("Failed to parse base url.");

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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    White,
    Black,
    Random,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Speed {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PerfType {
    UltraBullet,
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Chess960,
    Crazyhouse,
    Antichess,
    Atomic,
    Horde,
    KingOfTheHill,
    RacingKings,
    ThreeCheck,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LightUser {
    pub id: String,
    pub name: String,
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patron: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariantKey {
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

impl Default for VariantKey {
    fn default() -> Self {
        VariantKey::Standard
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Room {
    Player,
    Spectator,
}

impl Default for Room {
    fn default() -> Self {
        Room::Player
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Compat {
    pub bot: bool,
    pub board: bool,
}

#[derive(Clone, Debug, Serialize)]
pub enum Days {
    One,
    Two,
    Three,
    Five,
    Seven,
    Ten,
    Fourteen,
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
