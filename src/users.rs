pub mod models;
use std::vec;

use crate::LichessClient;

use self::models::Status;

impl LichessClient {
    pub async fn get_status(
        &self,
        user_ids: Vec<&str>,
        with_game_ids: bool,
    ) -> Result<Vec<Status>, reqwest::Error> {
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

        let status: Vec<Status> = serde_json::from_str(&response).expect("correct json");
        Ok(status)
    }
}
