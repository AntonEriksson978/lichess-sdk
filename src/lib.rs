pub mod account;

use dotenv::dotenv;
use reqwest::header;

pub struct LichessClient {
    client: reqwest::Client,
}

impl LichessClient {
    pub fn new() -> Result<LichessClient, String> {
        dotenv().ok();
        let token = match std::env::var("PERSONAL_ACCESS_TOKEN") {
            Ok(val) => val,
            Err(_) => return Err("PERSONAL_ACCESS_TOKEN is not set. Add a .env file with PERSONAL_ACCESS_TOKEN=your_token".to_string()),
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(format!("Bearer {}", &token).as_str())
                .map_err(|err| format!("Invalid header value: {:?}", err))?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("a working client");

        Ok(LichessClient { client })
    }
}
