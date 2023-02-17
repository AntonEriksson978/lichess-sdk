use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    perfs: Perfs,
    #[serde(rename = "createdAt")]
    created_at: i64,
    disabled: Option<bool>,
    #[serde(rename = "tosViolation")]
    tos_violation: Option<bool>,
    profile: Profile,
    #[serde(rename = "seenAt")]
    seen_at: i64,
    patron: Option<bool>,
    verified: Option<bool>,
    #[serde(rename = "playTime")]
    play_time: PlayTime,
    title: Option<String>,
    url: String,
    playing: Option<String>,
    #[serde(rename = "completionRate")]
    completion_rate: Option<i32>,
    count: Count,
    streaming: Option<bool>,
    followable: bool,
    following: bool,
    blocking: bool,
    #[serde(rename = "followsYou")]
    follows_you: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Perfs {
    chess960: Option<Perf>,
    atomic: Option<Perf>,
    #[serde(rename = "racingKings")]
    racing_kings: Option<Perf>,
    #[serde(rename = "ultraBullet")]
    ultra_bullet: Option<Perf>,
    blitz: Option<Perf>,
    #[serde(rename = "kingOfTheHill")]
    king_of_the_hill: Option<Perf>,
    bullet: Option<Perf>,
    correspondence: Option<Perf>,
    horde: Option<Perf>,
    puzzle: Option<Perf>,
    classical: Option<Perf>,
    rapid: Option<Perf>,
    storm: Option<Storm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Perf {
    games: i32,
    rating: i32,
    rd: i32,
    prog: i32,
    prov: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storm {
    runs: i32,
    score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    country: String,
    location: Option<String>,
    bio: Option<String>,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "fideRating")]
    fide_rating: Option<i32>,
    #[serde(rename = "uscfRating")]
    uscf_rating: Option<i32>,
    #[serde(rename = "ecfRating")]
    ecf_rating: Option<i32>,
    links: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayTime {
    total: i32,
    tv: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    all: i32,
    rated: i32,
    ai: i32,
    draw: i32,
    #[serde(rename = "drawH")]
    draw_h: i32,
    loss: i32,
    #[serde(rename = "lossH")]
    loss_h: i32,
    win: i32,
    #[serde(rename = "winH")]
    win_h: i32,
    bookmark: i32,
    playing: i32,
    import: i32,
    me: i32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_account() {
        let result = get_account().await;
        assert!(result.is_ok());
        assert!(result.unwrap().id.eq("fr0zenfire"));
    }

    #[tokio::test]
    async fn test_get_email() {
        let result = get_email().await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("antoneriksson978@gmail.com"));
    }
}
