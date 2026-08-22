# lichess-api

[![Crates.io][crates-badge]][crates-url]
[![Dependencies][deps-badge]][deps-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]

[crates-badge]: https://img.shields.io/crates/v/lichess-api.svg
[crates-url]: https://crates.io/crates/lichess-api
[deps-badge]: https://deps.rs/repo/github/ion232/lichess-api/status.svg
[deps-url]: https://deps.rs/repo/github/ion232/lichess-api
[apache-badge]: https://img.shields.io/badge/license-Apache%202.0-blue.svg
[apache-url]: LICENSE

A Rust API client library for [the current lichess.org API](https://lichess.org/api).
All endpoints are supported.

## Features

| Feature | Default | Description |
|---------|:-------:|-------------|
| `oauth` |   yes   | OAuth2 authorization code flow with PKCE, for acting on behalf of other users. Pulls in `rand` and `sha2`. |

Most clients authenticate with a [personal API token](https://lichess.org/account/oauth/token) and don't need the OAuth flow. If you only ever act as yourself, you can drop the dependencies:

```toml
lichess-api = { version = "0.7", default-features = false }
```

## Contributing

If you have any ideas, bug reports, feature requests, or fixes, please make an issue or submit a pull request.

Thanks.
