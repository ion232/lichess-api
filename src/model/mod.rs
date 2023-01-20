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

use serde::Serialize;

use crate::error::{Error, Result};

#[derive(Default, Clone, Debug, Serialize)]
pub struct EmptyBody;

impl ToString for EmptyBody {
    fn to_string(&self) -> String {
        Default::default()
    }
}

pub struct Request<Q, B = EmptyBody> where 
    Q: Serialize + Default,
    B: Serialize + ToString
{
    pub(crate) method: http::Method,
    pub(crate) path: String,
    pub(crate) query: Option<Q>,
    pub(crate) body: B
}

impl<Q, B> Request<Q, B> where 
    Q: Serialize + Default,
    B: Serialize + ToString
{
    pub(crate) fn as_http_request(self) -> Result<http::Request<bytes::Bytes>> {
        make_request(self.method, self.path, self.query, self.body)
    }
}

fn make_request<Q, B>(method: http::Method, path: String, query: Option<Q>, body: B) -> Result<http::Request<bytes::Bytes>> where
    Q: Default + Serialize,
    B: Serialize + ToString
{
    let url = make_url(path, query)?;
    http::Request::builder()
        .method(method)
        .uri(url.as_str())
        .body(bytes::Bytes::from(body.to_string()))
        .map_err(|e| Error::HttpRequestBuilder(e))
}

fn make_url<Q>(path: String, query: Option<Q>) -> Result<url::Url> where
    Q: Default + Serialize
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

pub(crate) fn to_json_string<B: Serialize>(body: &B) -> Result<String> {
    serde_json::to_string(&body)
        .map_err(|e| Error::Json(e))
}

pub(crate) fn to_form_string<B: Serialize>(body: &B) -> Result<String> {
    serde_urlencoded::to_string(&body)
        .map_err(|e| Error::UrlEncoded(e))
}
