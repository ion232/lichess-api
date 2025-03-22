pub mod player;
pub mod search;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::model::Title;

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub federation: Federation,
    pub title: Option<Title>,
    pub year: Option<u32>,
    pub inactive: Option<bool>,
    pub standard: Option<u32>,
    pub rapid: Option<u32>,
    pub blitz: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Federation {
    Ind,
    Usa,
    Rus,
    Chn,
    Nor,
    Fra,
    Ger,
    Arm,
    Aze,
    Esp,
    Nld,
    Pol,
    Hun,
    Ukr,
    Rou,
    Cze,
    Isr,
    Ita,
    Eng,
    Geo,
    Irn,
    Uzb,
    Vie,
    Phi,
    #[serde(other)]
    Other,
}
