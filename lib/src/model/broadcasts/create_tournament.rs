use crate::model::broadcasts::BroadcastTiebreakExtendedCode;
use crate::model::{Body, Request};
use serde::Serialize;
use serde::ser::SerializeMap;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Clone, Debug, Serialize)]
pub struct CreateBroadcastTournamentForm {
    pub name: String,
    #[serde(rename = "info.format")]
    pub info_format: Option<String>,
    #[serde(rename = "info.tc")]
    pub info_tc: Option<String>,
    #[serde(rename = "info.fideTC")]
    pub info_fide_tc: Option<String>,
    #[serde(rename = "info.timeZone")]
    pub info_time_zone: Option<String>,
    #[serde(rename = "info.location")]
    pub info_location: Option<String>,
    #[serde(rename = "info.players")]
    pub info_players: Option<String>,
    #[serde(rename = "info.website")]
    pub info_website: Option<String>,
    #[serde(rename = "info.standings")]
    pub info_standings: Option<String>,
    #[serde(rename = "info.regulations")]
    pub info_regulations: Option<String>,
    pub markdown: Option<String>,
    #[serde(rename = "showScores")]
    pub show_scores: Option<bool>,
    #[serde(rename = "showRatingDiffs")]
    pub show_rating_diffs: Option<bool>,
    #[serde(rename = "teamTable")]
    pub team_table: Option<bool>,
    pub visibility: Option<String>,
    pub players: Option<String>,
    pub teams: Option<String>,
    pub tier: Option<i32>,
    pub tiebreaks: Option<Vec<BroadcastTiebreakExtendedCode>>,
    #[serde(flatten)]
    pub grouping: Option<BroadcastGrouping>,
}

/// Groups this broadcast tournament together with others.
///
/// `score_groups` serializes as indexed form keys (`grouping.scoreGroups[0]`,
/// `grouping.scoreGroups[1]`, ...) per the Lichess API's non-standard array
/// encoding for this field, which a single struct field cannot otherwise
/// produce with `serde_urlencoded` - hence the manual `Serialize` impl below.
#[derive(Default, Clone, Debug)]
pub struct BroadcastGrouping {
    pub info_name: Option<String>,
    /// Linebreak separated list of tournament IDs to group together.
    pub info_tours: Option<String>,
    /// Each entry is a comma separated list of tournament IDs grouped for scoring.
    pub score_groups: Option<Vec<String>>,
}

impl Serialize for BroadcastGrouping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let score_groups_len = self.score_groups.as_ref().map_or(0, Vec::len);
        let len = self.info_name.is_some() as usize
            + self.info_tours.is_some() as usize
            + score_groups_len;
        let mut map = serializer.serialize_map(Some(len))?;
        if let Some(name) = &self.info_name {
            map.serialize_entry("grouping.info.name", name)?;
        }
        if let Some(tours) = &self.info_tours {
            map.serialize_entry("grouping.info.tours", tours)?;
        }
        if let Some(score_groups) = &self.score_groups {
            for (i, group) in score_groups.iter().enumerate() {
                map.serialize_entry(&format!("grouping.scoreGroups[{i}]"), group)?;
            }
        }
        map.end()
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, CreateBroadcastTournamentForm>;

impl PostRequest {
    pub fn new(form: CreateBroadcastTournamentForm) -> Self {
        Self::post("/broadcast/new", None, Body::Form(form), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grouping_serializes_score_groups_as_indexed_keys() {
        let form = CreateBroadcastTournamentForm {
            name: "Sinquefield Cup".to_string(),
            grouping: Some(BroadcastGrouping {
                info_name: Some("Chess Olympiad | Open".to_string()),
                info_tours: Some("wYigbpXq\nM5YHvpOX".to_string()),
                score_groups: Some(vec![
                    "wYigbpXq,M5YHvpOX".to_string(),
                    "q6ezoCXP".to_string(),
                ]),
            }),
            ..Default::default()
        };

        let encoded = serde_urlencoded::to_string(&form).unwrap();

        assert_eq!(
            encoded,
            "name=Sinquefield+Cup\
             &grouping.info.name=Chess+Olympiad+%7C+Open\
             &grouping.info.tours=wYigbpXq%0AM5YHvpOX\
             &grouping.scoreGroups%5B0%5D=wYigbpXq%2CM5YHvpOX\
             &grouping.scoreGroups%5B1%5D=q6ezoCXP"
        );
    }

    #[test]
    fn grouping_omitted_when_none() {
        let form = CreateBroadcastTournamentForm {
            name: "Sinquefield Cup".to_string(),
            ..Default::default()
        };

        let encoded = serde_urlencoded::to_string(&form).unwrap();

        assert_eq!(encoded, "name=Sinquefield+Cup");
    }
}
