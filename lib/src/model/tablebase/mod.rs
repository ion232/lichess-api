pub mod antichess;
pub mod atomic;
pub mod standard;

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TablebaseJson {
    pub category: Category,
    pub dtz: Option<i32>,
    pub precise_dtz: Option<i32>,
    pub dtm: Option<i32>,
    pub checkmate: bool,
    pub stalemate: bool,
    pub variant_win: bool,
    pub variant_loss: bool,
    pub insufficient_material: bool,
    pub moves: Vec<Move>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Move {
    pub uci: String,
    pub san: String,
    pub category: Category,
    pub dtz: Option<i32>,
    pub precise_dtz: Option<i32>,
    pub dtm: Option<i32>,
    pub zeroing: bool,
    pub checkmate: bool,
    pub stalemate: bool,
    pub variant_win: bool,
    pub variant_loss: bool,
    pub insufficient_material: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "maybe-win")]
    MaybeWin,
    #[serde(rename = "cursed-win")]
    CursedWin,
    #[serde(rename = "loss")]
    Loss,
    #[serde(rename = "blessed-loss")]
    BlessedLoss,
    #[serde(rename = "maybe-loss")]
    MaybeLoss,
    #[serde(rename = "draw")]
    Draw,
    #[serde(rename = "unknown")]
    Unknown,
}
