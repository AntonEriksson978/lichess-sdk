use dotenv::dotenv;

pub async fn get_account() -> Result<String, reqwest::Error> {
    let url = "https://lichess.org/api/account";
    dotenv().ok();
    let token = std::env::var("PERSONAL_ACCESS_TOKEN")
        .expect("a valid PERSONAL_ACCESS_TOKEN in a .env file");

    let auth_header_value = format!("Bearer {}", token);

    let response = reqwest::Client::new()
        .get("https://lichess.org/api/account")
        .header(reqwest::header::AUTHORIZATION, auth_header_value)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
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
        assert!(result.unwrap().contains("\"id\":\"fr0zenfire\""));
    }

    #[tokio::test]
    async fn test_get_email() {
        let result = get_email().await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("antoneriksson978@gmail.com"));
    }
}
