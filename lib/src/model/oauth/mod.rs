use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod revoke;
pub mod test;

/// Maps each tested token to its details, or `None` if the token is invalid.
pub type TestResults = HashMap<String, Option<Token>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    /// Comma separated
    pub scopes: String,
    pub user_id: String,
    /// Unix timestamp in milliseconds, or `None` if the token never expires.
    pub expires: Option<u64>,
}
