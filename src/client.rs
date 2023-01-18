use async_std::io::prelude::BufReadExt;
use async_std::stream::StreamExt;
use bytes::Bytes;

use futures::TryStreamExt;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::{Error, Result};

static LICHESS_BASE: &str = "https://lichess.org";

pub struct LichessApi<'a, HttpClient> {
    pub client: &'a HttpClient,
    auth_token: Option<String>
}

impl<'a, HttpClient> LichessApi<'a, HttpClient> {
    pub fn new(client: &'a HttpClient, auth_token: Option<String>) -> Self {
        Self {
            client,
            auth_token
        }
    }

    pub(crate) fn request_builder<Q>(&self, path_segments: Vec<&str>, query: Option<Q>) -> Result<http::request::Builder>
    where Q: Serialize
    {
        let url = self.make_url(path_segments, query)?;
        
        let mut builder = http::Request::builder().uri(url.as_str());
        if let Some(token) = &self.auth_token {
            builder = builder.header("Bearer", token)
        }
    
        Ok(builder)
    }

    fn make_url<Q>(&self, path_segments: Vec<&str>, query: Option<Q>) -> Result<url::Url>
    where Q: Serialize
    {
        let mut url = url::Url::parse(LICHESS_BASE).expect("Failed to parse base url.");

        if let Some(q) = query {
            let mut query_pairs = url.query_pairs_mut();
            let query_serializer = serde_urlencoded::Serializer::new(&mut query_pairs);
            q.serialize(query_serializer)?;
        }

        {
            let mut segments = url.path_segments_mut().unwrap();
            segments.extend(path_segments);
        }

        Ok(url)
    }

    pub(crate) async fn expect_one_model<Model, G>(&self, stream: &mut G) -> Result<Model>
    where G: StreamExt<Item = Result<Model>> + std::marker::Unpin
    {
        stream.next()
            .await
            .ok_or(Error::Response("empty response stream".to_string()))?
    }
}

impl<'a> LichessApi<'a, reqwest::Client> {
    pub(crate) async fn send<Model: DeserializeOwned>(&self, http_request: http::Request<Bytes>) -> Result<impl StreamExt<Item = Result<Model>>> {
        let convert_err = |e: reqwest::Error| Error::Request(e.to_string());

        let request = reqwest::Request::try_from(http_request).map_err(convert_err)?;
        let stream = self
            .client
            .execute(request)
            .await.map_err(convert_err)?
            .bytes_stream()
            .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
            .into_async_read()
            .lines()
            .map(|l| -> Result<Model> {
                let line = l?;
                println!("{}", line);
                serde_json::from_str(&line).map_err(|e| crate::error::Error::Json(e))
            });

        Ok(stream)
    }
}
