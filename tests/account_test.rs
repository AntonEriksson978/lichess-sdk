pub use lichess_sdk::account::*;

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

#[tokio::test]
async fn test_get_preferences() {
    let result = get_preferences().await;
    assert!(result.is_ok());
}
