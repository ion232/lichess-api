use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize)]
pub struct GetQuery;

pub type GetRequest = crate::model::Request<GetQuery>;

impl GetRequest {
    pub fn new() -> Self {
        Self {
            path: "/api/account/preferences".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub dark: bool,
    pub transp: bool,
    pub bg_img: String,
    pub is3d: bool,
    pub theme: Theme,
    pub piece_set: PieceSet,
    pub theme3d: Theme3D,
    pub piece_set3d: PieceSet3D,
    pub sound_set: SoundSet,
    pub blindfold: u32,
    pub auto_queen: u32,
    pub auto_threefold: u32,
    pub takeback: u32,
    pub moretime: u32,
    pub clock_tenths: u32,
    pub clock_bar: bool,
    pub clock_sound: bool,
    pub premove: bool,
    pub animation: u32,
    pub captured: bool,
    pub follow: bool,
    pub highlight: bool,
    pub destination: bool,
    pub coords: u32,
    pub replay: u32,
    pub challenge: u32,
    pub message: u32,
    pub coord_color: u32,
    pub submit_move: u32,
    pub confirm_resign: u32,
    pub insight_share: u32,
    pub keybord_move: u32,
    pub zen: u32,
    pub move_event: u32,
    pub rook_castle: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    Blue,
    Blue2,
    Blue3,
    BlueMarble,
    Canvas,
    Wood,
    Wood2,
    Wood3,
    Wood4,
    Maple,
    Maple2,
    Brown,
    Leather,
    Green,
    Marble,
    GreenPlastic,
    Grey,
    Metal,
    Olive,
    Newspaper,
    Purple,
    PurpleDiag,
    Pink,
    Ic,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PieceSet {
    Cburnett,
    Merida,
    Alpha,
    Pirouetti,
    Chessnut,
    Chess7,
    Reillycraig,
    Companion,
    Riohacha,
    Kosal,
    Leipzig,
    Fantasy,
    Spatial,
    California,
    Pixel,
    Maestro,
    Fresca,
    Cardinal,
    Gioco,
    Tatiana,
    Staunty,
    Governor,
    Dubrovny,
    Icpieces,
    Shapes,
    Letter,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum Theme3D {
    #[serde(rename = "Black-White-Aluminium")]
    BlackWhiteAluminium,
    #[serde(rename = "Brushed-Aluminium")]
    BrushedAluminium,
    #[serde(rename = "China-Blue")]
    ChinaBlue,
    #[serde(rename = "China-Green")]
    ChinaGreen,
    #[serde(rename = "China-Grey")]
    ChinaGrey,
    #[serde(rename = "China-Scarlet")]
    ChinaScarlet,
    #[serde(rename = "Classic-Blue")]
    ClassicBlue,
    #[serde(rename = "Gold-Silver")]
    GoldSilver,
    #[serde(rename = "Light-Wood")]
    LightWood,
    #[serde(rename = "Power-Coated")]
    PowerCoated,
    Rosewood,
    Marble,
    Wax,
    Jade,
    Woodi,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum PieceSet3D {
    Basic,
    Wood,
    Metal,
    RedVBlue,
    ModernJade,
    ModernWood,
    Glass,
    Trimmed,
    Experimental,
    Staunton,
    CubesAndPi,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoundSet {
    Silent,
    Standard,
    Piano,
    Nes,
    Sfx,
    Futuristic,
    Robot,
    Music,
    Speech,
}
