pub mod models;

use crate::{users::models::User, LichessClient};

use self::models::UserStatus;

impl LichessClient {
    pub async fn get_realtime_users_status(
        &self,
        user_ids: Vec<&str>,
        with_game_ids: bool,
    ) -> Result<Vec<UserStatus>, reqwest::Error> {
        let url = "https://lichess.org/api/users/status";
        let response = self
            .client
            .get(url)
            .query(&[
                ("ids", user_ids.join(",")),
                ("withGameIds", with_game_ids.to_string()),
            ])
            .send()
            .await?
            .text()
            .await?;

        let status: Vec<UserStatus> = serde_json::from_str(&response).expect("correct json");
        Ok(status)
    }

    pub async fn get_user_public_data(&self, username: &str) -> Result<User, reqwest::Error> {
        let url = format!("https://lichess.org/api/user/{}", username);

        let response = self.client.get(url).send().await?.text().await?;
        let user: User = serde_json::from_str(&response).expect("json");
        Ok(user)
    }
}
