# lichess-api

[![Crates.io][crates-badge]][crates-url]
[![Dependencies][deps-badge]][deps-url]
[![Documentation][docs-badge]][docs-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]

[crates-badge]: https://img.shields.io/crates/v/lichess-api.svg
[crates-url]: https://crates.io/crates/lichess-api
[deps-badge]: https://deps.rs/repo/github/ion232/lichess-api/status.svg
[deps-url]: https://deps.rs/repo/github/ion232/lichess-api
[docs-badge]: https://docs.rs/lichess-api/badge.svg
[docs-url]: https://docs.rs/lichess-api
[apache-badge]: https://img.shields.io/badge/license-Apache%202.0-blue.svg
[apache-url]: LICENSE

A Rust API client library for [the current lichess.org API](https://lichess.org/api).

The goal of this crate is to fully support the latest lichess API - a major version release will be made once all endpoints are supported.

The lichess endpoints will often change without an OpenAPI version or even schema change, so please raise an issue with relevant output if one of the endpoints is failing.

## Endpoints

- ✅ = Fully supported at the time of the most recent release crate.
- 🔶 = Partially supported.
- 🚧 = Work to support this category is currently in progress.
- ❌ = Not currently supported.

The following table shows the current level of support for each category of endpoints.

| Category              | Status  |
|-----------------------|:-------:|
| Account               |   ✅    |
| Analysis              |   ✅    |
| Arena Tournaments     |   ❌    |
| Board                 |   ✅    |
| Bot                   |   ✅    |
| Broadcasts            |   ❌    |
| Bulk Pairings         |   ❌    |
| Challenges            |   ✅    |
| External Engine       |   🔶    |
| Games                 |   ✅    |
| Messaging             |   ✅    |
| Opening Explorer      |   ✅    |
| OAuth                 |   ❌    |
| Puzzles               |   ✅    |
| Relations             |   ❌    |
| Simuls                |   ✅    |
| Studies               |   🔶    |
| Swiss Tournaments     |   ❌    |
| Tablebase             |   ✅    |
| Teams                 |   ❌    |
| TV                    |   ✅    |
| Users                 |   ✅    |

## Contributing

Contributions are much appreciated - especially if you can add support for a category of endpoints. Otherwise, if you have any ideas, bug reports, feature requests, or fixes, please make an issue or submit a pull request.

Thanks.
