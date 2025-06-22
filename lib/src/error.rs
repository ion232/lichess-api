//!

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("urlencoded serde error: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),

    #[error("http request builder error: {0}")]
    HttpRequestBuilder(#[from] http::Error),

    #[error("lichess status error: {0}")]
    LichessStatus(String),

    #[error("page not found error")]
    PageNotFound(),

    #[error("request parameters error: {0}")]
    RequestParams(String),

    #[error("request error: {0}")]
    Request(String),

    #[error("response error: {0}")]
    Response(String),

    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("json serde error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;
