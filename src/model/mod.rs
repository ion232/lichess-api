pub mod account;
pub mod analysis;
pub mod arena_tournaments;
pub(crate) mod base_components;
pub mod board;
pub mod bot;
pub mod broadcasts;
pub mod bulk_pairings;
pub mod challenges;
pub mod external_engine;
pub mod games;
pub mod messaging;
pub mod oauth;
pub mod opening_explorer;
pub mod puzzles;
pub mod relations;
pub mod simuls;
pub mod studies;
pub mod swiss_tournaments;
pub mod tablebase;
pub mod teams;
pub mod tv;
pub mod users;

use serde::{Serialize, de::DeserializeOwned};
use crate::error::{Error, Result};

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
    Empty
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
            Body::Empty => None
        }
    }

    fn as_encoded_string(&self) -> Result<String> {
        let body = match &self {
            Body::Form(form) => to_form_string(&form)?,
            Body::Json(json) => to_json_string(&json)?,
            Body::PlainText(text) => text.to_string(),
            Body::Empty => "".to_string()
        };
        Ok(body)
    }
}

pub struct Request<Q, B = ()> where 
    Q: QueryBounds,
    B: BodyBounds
{
    pub(crate) method: http::Method,
    pub(crate) path: String,
    pub(crate) query: Option<Q>,
    pub(crate) body: Body<B>
}

impl<Q, B> Request<Q, B> where 
    Q: QueryBounds,
    B: BodyBounds
{
    pub(crate) fn as_http_request(self) -> Result<http::Request<bytes::Bytes>> {
        make_request(self.method, self.path, self.query, self.body)
    }
}

fn make_request<Q, B>(method: http::Method, path: String, query: Option<Q>, body: Body<B>) -> Result<http::Request<bytes::Bytes>> where
    Q: QueryBounds,
    B: BodyBounds
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
        .map_err(|e| Error::HttpRequestBuilder(e))?;

    Ok(request)
}

fn make_url<Q>(path: String, query: Option<Q>) -> Result<url::Url> where
    Q: QueryBounds
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

fn to_json_string<B: BodyBounds>(body: &B) -> Result<String> {
    serde_json::to_string(&body)
        .map_err(|e| Error::Json(e))
}

fn to_form_string<B: BodyBounds>(body: &B) -> Result<String> {
    serde_urlencoded::to_string(&body)
        .map_err(|e| Error::UrlEncoded(e))
}
