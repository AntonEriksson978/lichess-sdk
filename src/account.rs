pub mod models;

use self::models::Account;
use self::models::UserPreferences;
use crate::LichessClient;

impl LichessClient {
    pub async fn get_account(&self) -> Result<Account, reqwest::Error> {
        let url = "https://lichess.org/api/account";
        let response = self.client.get(url).send().await?.text().await?;
        let account: Account = serde_json::from_str(&response)
            .expect("a json object that corresponds with User struct");
        Ok(account)
    }

    pub async fn get_email(&self) -> Result<String, reqwest::Error> {
        let url = "https://lichess.org/api/account/email";
        let response = self.client.get(url).send().await?.text().await?;
        Ok(response)
    }

    pub async fn get_preferences(&self) -> Result<UserPreferences, reqwest::Error> {
        let url = "https://lichess.org/api/account/preferences";
        let response = self.client.get(url).send().await?.text().await?;
        let prefs: UserPreferences = serde_json::from_str(&response)
            .expect("a json object that corresponds with UserPreferences struct");
        Ok(prefs)
    }
}
