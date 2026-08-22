use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod authorize;
pub mod revoke;
pub mod test;
pub mod token;

/// Maps each tested token to its details, or `None` if the token is invalid.
pub type TestResults = HashMap<String, Option<Token>>;

/// An access token obtained by exchanging an authorization code.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub token_type: String,
    pub access_token: String,
    /// Lifetime of the token in seconds.
    pub expires_in: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    /// Comma separated
    pub scopes: String,
    pub user_id: String,
    /// Unix timestamp in milliseconds, or `None` if the token never expires.
    pub expires: Option<u64>,
}
