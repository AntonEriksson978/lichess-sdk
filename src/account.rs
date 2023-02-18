use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub perfs: Perfs,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    pub disabled: Option<bool>,
    #[serde(rename = "tosViolation")]
    pub tos_violation: Option<bool>,
    pub profile: Profile,
    #[serde(rename = "seenAt")]
    pub seen_at: i64,
    pub patron: Option<bool>,
    pub verified: Option<bool>,
    #[serde(rename = "playTime")]
    pub play_time: PlayTime,
    pub title: Option<String>,
    pub url: String,
    pub playing: Option<String>,
    #[serde(rename = "completionRate")]
    pub completion_rate: Option<i32>,
    pub count: Count,
    pub streaming: Option<bool>,
    pub followable: bool,
    pub following: bool,
    pub blocking: bool,
    #[serde(rename = "followsYou")]
    pub follows_you: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Perfs {
    pub chess960: Option<Perf>,
    pub atomic: Option<Perf>,
    #[serde(rename = "racingKings")]
    pub racing_kings: Option<Perf>,
    #[serde(rename = "ultraBullet")]
    pub ultra_bullet: Option<Perf>,
    pub blitz: Option<Perf>,
    #[serde(rename = "kingOfTheHill")]
    pub king_of_the_hill: Option<Perf>,
    pub bullet: Option<Perf>,
    pub correspondence: Option<Perf>,
    pub horde: Option<Perf>,
    pub puzzle: Option<Perf>,
    pub classical: Option<Perf>,
    pub rapid: Option<Perf>,
    pub storm: Option<Storm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Perf {
    pub games: i32,
    pub rating: i32,
    pub rd: i32,
    pub prog: i32,
    pub prov: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storm {
    pub runs: i32,
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub country: String,
    pub location: Option<String>,
    pub bio: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "fideRating")]
    pub fide_rating: Option<i32>,
    #[serde(rename = "uscfRating")]
    pub uscf_rating: Option<i32>,
    #[serde(rename = "ecfRating")]
    pub ecf_rating: Option<i32>,
    pub links: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayTime {
    pub total: i32,
    pub tv: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    pub all: i32,
    pub rated: i32,
    pub ai: i32,
    pub draw: i32,
     #[serde(rename = "drawH")]
    pub draw_h: i32,
    pub loss: i32,
    #[serde(rename = "lossH")]
    pub loss_h: i32,
    pub win: i32,
    #[serde(rename = "winH")]
    pub win_h: i32,
    pub bookmark: i32,
    pub playing: i32,
    pub import: i32,
    pub me: i32,
}

pub async fn get_account() -> Result<User, reqwest::Error> {
    let url = "https://lichess.org/api/account";
    dotenv().ok();
    let token = std::env::var("PERSONAL_ACCESS_TOKEN")
        .expect("a valid PERSONAL_ACCESS_TOKEN in a .env file");

    let auth_header_value = format!("Bearer {}", token);

    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::AUTHORIZATION, auth_header_value)
        .send()
        .await?
        .text()
        .await?;
    println!("{}", response);
    let user: User =
        serde_json::from_str(&response).expect("a json object that corresponds with User struct");
    Ok(user)
}

pub async fn get_email() -> Result<String, reqwest::Error> {
    let url = "https://lichess.org/api/account/email";

    dotenv().ok();
    let token = std::env::var("PERSONAL_ACCESS_TOKEN")
        .expect("a valid PERSONAL_ACCESS_TOKEN in a .env file");

    let auth_header_value = format!("Bearer {}", token);

    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::AUTHORIZATION, auth_header_value)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

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

pub async fn get_preferences() -> Result<UserPreferences, reqwest::Error> {
    let url = "https://lichess.org/api/account/preferences";
    dotenv().ok();
    let token = std::env::var("PERSONAL_ACCESS_TOKEN")
        .expect("a valid PERSONAL_ACCESS_TOKEN in a .env file");

    let auth_header_value = format!("Bearer {}", token);

    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::AUTHORIZATION, auth_header_value)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", response);
    let prefs: UserPreferences = serde_json::from_str(&response)
        .expect("a json object that corresponds with UserPreferences struct");
    Ok(prefs)
}
