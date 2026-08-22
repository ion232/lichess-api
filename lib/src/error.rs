//!

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("urlencoded serde error: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),

    #[error("http request builder error: {0}")]
    HttpRequestBuilder(#[from] http::Error),

    #[error("lichess status error: {0}")]
    LichessStatus(String),

    #[error("page not found error (likely invalid path)")]
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

    #[cfg(feature = "oauth")]
    #[error("oauth error: {error}{}", .error_description.as_ref().map(|d| format!(" ({d})")).unwrap_or_default())]
    OAuth {
        /// The cause of the error, e.g. `access_denied` if the user cancelled
        /// authorization, or `invalid_grant`.
        error: String,
        /// The reason the request was rejected, to aid debugging.
        error_description: Option<String>,
    },

    #[cfg(feature = "oauth")]
    #[error("oauth state mismatch (possible cross site request forgery)")]
    OAuthStateMismatch,

    #[error("unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;
