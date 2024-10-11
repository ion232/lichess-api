use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod revoke;
pub mod test;

pub type TestResults = HashMap<String, Token>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    /// Comma separated
    pub scopes: String,
    pub user_id: String,
    /// Unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<u64>,
}
