# lichess-api

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Dependencies][deps-badge]][deps-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]

[crates-badge]: https://img.shields.io/crates/v/lichess-api.svg
[crates-url]: https://crates.io/crates/lichess-api
[docs-badge]: https://docs.rs/lichess-api/badge.svg
[docs-url]: https://docs.rs/lichess-api
[deps-badge]: https://deps.rs/repo/github/ion232/lichess-api/status.svg
[deps-url]: https://deps.rs/repo/github/ion232/lichess-api
[apache-badge]: https://img.shields.io/badge/license-Apache%202.0-blue.svg
[apache-url]: LICENSE

An asynchronous client library for [the current lichess.org API](https://lichess.org/api) with all endpoints supported.

## Quick start

Add the dependencies:

```toml
[dependencies]
lichess-api = "0.7"
tokio = { version = "1", features = ["full"] }
```

Example request:

```rust,no_run
use lichess_api::client::LichessApi;

#[tokio::main]
async fn main() -> lichess_api::error::Result<()> {
    let client = reqwest::Client::new();
    let token = std::env::var("LICHESS_TOKEN").ok();
    let api = LichessApi::new(client, token);

    let profile = api.get_profile().await?;
    println!("username: {}", profile.user.username);

    Ok(())
}
```

## Authentication

Most endpoints require a bearer token but some don't. E.g. the daily puzzle, etc.

- **Acting as yourself**: generate a [personal API
  token](https://lichess.org/account/oauth/token) and pass it to
  `LichessApi::new` as above.
- **Acting on behalf of another user**: Ensure the oauth feature is enabled and use the OAuth2 authorization code flow
  with PKCE, via [`AuthorizationUrl`](https://docs.rs/lichess-api/latest/lichess_api/model/oauth/authorize/struct.AuthorizationUrl.html) and
  [`PendingAuthorization`](https://docs.rs/lichess-api/latest/lichess_api/model/oauth/struct.PendingAuthorization.html).

## Features

| Feature | Default | Description |
|---------|:-------:|-------------|
| `oauth` |   yes   | OAuth2 authorization code flow with PKCE, for acting on behalf of other users. |

## Contributing

If you have any ideas, bug reports, feature requests, or fixes, please make an issue or submit a pull request.

Thanks.
