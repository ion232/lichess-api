pub mod create;
pub mod games;
pub mod list;
pub mod remove;
pub mod show;
pub mod start_clocks;

use crate::model::{ArenaClock, VariantKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkPairing {
    pub id: String,
    pub games: Vec<BulkPairingGame>,
    pub variant: VariantKey,
    pub clock: ArenaClock,
    pub pair_at: i64,
    pub paired_at: Option<i64>,
    pub rated: bool,
    pub start_clocks_at: i64,
    pub scheduled_at: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BulkPairingGame {
    pub id: String,
    pub black: String,
    pub white: String,
}
