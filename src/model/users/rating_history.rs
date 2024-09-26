use serde::{
    ser::{SerializeSeq, SerializeTuple},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery {
    pub trophies: bool,
}

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new(username: &str) -> Self {
        Self::get(format!("/api/user/{username}/rating-history"), None, None)
    }
}

impl<S: AsRef<str>> From<S> for GetRequest {
    fn from(s: S) -> Self {
        Self::new(s.as_ref())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum RatingType {
    Bullet,
    Blitz,
    Rapid,
    Classical,
    Correspondence,
    Chess960,
    #[serde(rename = "King of the Hill")]
    KingOfTheHill,
    #[serde(rename = "Three-check")]
    ThreeCheck,
    Antichess,
    Atomic,
    Horde,
    #[serde(rename = "Racing Kings")]
    RacingKings,
    Crazyhouse,
    UltraBullet,
    Puzzles,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RatingEntry {
    pub name: RatingType,
    pub points: Vec<RatingPoint>,
}

// Lichess returns a hard to use tuple of `[year, month, day, rating]`,
// because of this Serialize / Deserialize are manually implemented.
#[derive(Clone, Debug, Deserialize)]
pub struct RatingPoint {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub rating: u32,
}

impl Serialize for RatingPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(4)?;
        seq.serialize_element(&self.year)?;
        seq.serialize_element(&self.month)?;
        seq.serialize_element(&self.day)?;
        seq.serialize_element(&self.rating)?;
        seq.end()
    }
}

#[derive(Default, Clone, Debug)]
pub struct RatingHistory {
    pub bullet: Option<Box<[RatingPoint]>>,
    pub blitz: Option<Box<[RatingPoint]>>,
    pub rapid: Option<Box<[RatingPoint]>>,
    pub classical: Option<Box<[RatingPoint]>>,
    pub correspondence: Option<Box<[RatingPoint]>>,
    pub chess960: Option<Box<[RatingPoint]>>,
    pub king_of_the_hill: Option<Box<[RatingPoint]>>,
    pub three_check: Option<Box<[RatingPoint]>>,
    pub antichess: Option<Box<[RatingPoint]>>,
    pub atomic: Option<Box<[RatingPoint]>>,
    pub horde: Option<Box<[RatingPoint]>>,
    pub racing_kings: Option<Box<[RatingPoint]>>,
    pub crazyhouse: Option<Box<[RatingPoint]>>,
    pub puzzles: Option<Box<[RatingPoint]>>,
    pub ultra_bullet: Option<Box<[RatingPoint]>>,
}

macro_rules! index {
    ($(($field: ident, $type: ident)),* $(,)?) => {
        impl RatingHistory {
            fn get(&self, index: RatingType) -> Option<&[RatingPoint]> {
                match index {
                    $(RatingType::$type => &self.$field),*
                }.as_ref().map(|points|points.as_ref())

            }

            fn get_mut(&mut self, index: RatingType) -> &mut Option<Box<[RatingPoint]>> {
                match index {
                    $(RatingType::$type => &mut self.$field),*
                }
            }
        }

        const RATING_TYPES: [RatingType; 15] = [$(RatingType::$type),*];
    };
}

index!(
    (bullet, Bullet),
    (blitz, Blitz),
    (rapid, Rapid),
    (classical, Classical),
    (correspondence, Correspondence),
    (chess960, Chess960),
    (king_of_the_hill, KingOfTheHill),
    (three_check, ThreeCheck),
    (antichess, Antichess),
    (atomic, Atomic),
    (horde, Horde),
    (racing_kings, RacingKings),
    (crazyhouse, Crazyhouse),
    (puzzles, Puzzles),
    (ultra_bullet, UltraBullet),
);

impl Serialize for RatingHistory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct RefRatingEntry<'a> {
            name: RatingType,
            points: &'a [RatingPoint],
        }

        let mut seq = serializer.serialize_seq(None)?;

        for name in RATING_TYPES {
            if let Some(points) = &self.get(name) {
                seq.serialize_element(&RefRatingEntry { name, points })?;
            }
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for RatingHistory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let points = <Vec<RatingEntry>>::deserialize(deserializer)?;
        let mut history = Self::default();

        for point in points {
            *history.get_mut(point.name) = Some(point.points.into_boxed_slice());
        }

        Ok(history)
    }
}
