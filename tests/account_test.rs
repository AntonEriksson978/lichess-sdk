pub use lichess_sdk::account::*;
pub use lichess_sdk::LichessClient;

#[tokio::test]
async fn test_get_account() {
    let lichess_client = LichessClient::new();
    assert!(lichess_client.is_ok());
    let result = lichess_client.unwrap().get_account().await;
    assert!(result.is_ok());
    assert!(result.unwrap().id.eq("fr0zenfire"));
}

#[tokio::test]
async fn test_get_email() {
    let lichess_client = LichessClient::new();
    assert!(lichess_client.is_ok());

    let result = lichess_client.unwrap().get_email().await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("antoneriksson978@gmail.com"));
}

#[tokio::test]
async fn test_get_preferences() {
    let lichess_client = LichessClient::new();
    assert!(lichess_client.is_ok());

    let result = lichess_client.unwrap().get_preferences().await;
    assert!(result.is_ok());
}
