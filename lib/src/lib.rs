//! A Rust client for [the Lichess API](https://lichess.org/api).
//!
//! [`client::LichessApi`] wraps an HTTP client (currently only `reqwest::Client`
//! is supported) with an optional bearer token. Every operation is a method on
//! `LichessApi<reqwest::Client>`, grouped into one [`api`] module per Lichess API
//! category (`api::account`, `api::games`, `api::tv`, ...). The request/response
//! types for a given operation live in the matching [`model`] module
//! (`model::account`, `model::games`, ...).
//!
//! # Authentication
//!
//! Most endpoints take a bearer token. For your own account, generate a
//! [personal API token](https://lichess.org/account/oauth/token) and pass it to
//! [`client::LichessApi::new`]:
//!
//! ```no_run
//! use lichess_api::client::LichessApi;
//!
//! # async fn run() -> lichess_api::error::Result<()> {
//! let http_client = reqwest::Client::new();
//! let token = std::env::var("LICHESS_TOKEN").ok();
//! let api = LichessApi::new(http_client, token);
//!
//! let profile = api.get_profile().await?;
//! println!("logged in as {}", profile.user.username);
//! # Ok(())
//! # }
//! ```
//!
//! Public endpoints that don't require a token accept `LichessApi::new(client, None)`.
//!
//! To act on behalf of *other* users, use the OAuth2 authorization code flow
//! with PKCE instead of a personal token — see
//! [`model::oauth::authorize::AuthorizationUrl`] and
//! [`model::oauth::PendingAuthorization`] for a full walkthrough, gated behind
//! the default-on `oauth` feature.
//!
//! # Streamed endpoints
//!
//! Endpoints that stream newline-delimited JSON (board game state, TV feeds,
//! broadcast rounds, ...) return a `Stream` of results instead of a single
//! value, so results arrive as they're produced rather than after the whole
//! response body has been read.
//!
//! # Errors
//!
//! Every operation returns [`error::Result`]; see [`error::Error`] for the
//! failure cases (transport errors, non-2xx responses, deserialization
//! failures, and OAuth-specific errors).

pub mod api;
pub mod client;
pub mod error;
pub mod model;
