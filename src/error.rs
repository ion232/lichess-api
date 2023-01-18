//!

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("url serialization error: {0}")]
    Url(#[from] serde_urlencoded::ser::Error),

    #[error("http request builder error: {0}")]
    HttpRequestBuilder(#[from] http::Error),

    #[error("request error: {0}")]
    Request(String),

    #[error("response error: {0}")]
    Response(String),

    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("json deserialiation error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unknown error: {0}")]
    Unknown(String)
}

pub type Result<T> = std::result::Result<T, Error>;
