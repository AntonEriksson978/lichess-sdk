use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Preferences {
    pub dark: bool,
    #[serde(rename = "transp")]
    pub transparent: bool,
    #[serde(rename = "bgImg")]
    pub background_image: String,
    #[serde(rename = "is3d")]
    pub is3d: bool,
    #[serde(rename = "theme")]
    pub theme: String,
    #[serde(rename = "pieceSet")]
    pub piece_set: String,
    #[serde(rename = "theme3d")]
    pub theme_3d: String,
    #[serde(rename = "pieceSet3d")]
    pub piece_set_3d: String,
    #[serde(rename = "soundSet")]
    pub sound_set: String,
    pub blindfold: i32,
    #[serde(rename = "autoQueen")]
    pub auto_queen: i32,
    #[serde(rename = "autoThreefold")]
    pub auto_threefold: i32,
    pub takeback: i32,
    pub moretime: i32,
    #[serde(rename = "clockTenths")]
    pub clock_tenths: i32,
    #[serde(rename = "clockBar")]
    pub clock_bar: bool,
    #[serde(rename = "clockSound")]
    pub clock_sound: bool,
    pub premove: bool,
    pub animation: i32,
    pub captured: bool,
    pub follow: bool,
    pub highlight: bool,
    pub destination: bool,
    pub coords: i32,
    pub replay: i32,
    pub challenge: i32,
    pub message: i32,
    #[serde(rename = "coordColor")]
    pub coord_color: Option<i32>,
    #[serde(rename = "submitMove")]
    pub submit_move: i32,
    #[serde(rename = "confirmResign")]
    pub confirm_resign: i32,
    #[serde(rename = "insightShare")]
    pub insight_share: i32,
    #[serde(rename = "keyboardMove")]
    pub keyboard_move: i32,
    pub zen: i32,
    #[serde(rename = "moveEvent")]
    pub move_event: i32,
    #[serde(rename = "rookCastle")]
    pub rook_castle: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPreferences {
    pub prefs: Preferences,
    pub language: String,
}
