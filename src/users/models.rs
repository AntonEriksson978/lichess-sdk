use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserStatus {
    pub id: String,
    pub name: String,
    // consider making these default to false instead of being options.
    pub title: Option<String>,
    pub online: Option<bool>,
    pub playing: Option<bool>,
    pub streaming: Option<bool>,
    pub patron: Option<bool>,
}

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
