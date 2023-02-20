use lichess_sdk::LichessClient;

#[tokio::test]
async fn test_get_realtime_users_status_without_game_id() {
    let lichess_client = LichessClient::new();
    assert!(lichess_client.is_ok());
    let ids = vec!["fr0zenfire"];
    let with_game_id = false;
    let result = lichess_client
        .unwrap()
        .get_realtime_users_status(ids, with_game_id)
        .await;
    assert!(result.is_ok());
    let status = result.unwrap();
    dbg!(&status);
    assert_eq!(status.len(), 1);
    assert_eq!(status.first().unwrap().id, "fr0zenfire");
}

#[tokio::test]
async fn test_get_realtime_users_status_with_game_id() {
    let lichess_client = LichessClient::new();
    assert!(lichess_client.is_ok());
    let ids = vec!["fr0zenfire"];
    let with_game_id = true;
    let result = lichess_client
        .unwrap()
        .get_realtime_users_status(ids, with_game_id)
        .await;
    assert!(result.is_ok());
    let status = result.unwrap();
    dbg!(&status);
    assert_eq!(status.len(), 1);
    assert_eq!(status.first().unwrap().id, "fr0zenfire");
}

#[tokio::test]
async fn test_get_user_public_data() {
    let lichess_client = LichessClient::new();
    assert!(lichess_client.is_ok());
    let username = "fr0zenfire";
    let result = lichess_client.unwrap().get_user_public_data(username).await;
    assert!(result.is_ok());
    let user = result.unwrap();
    dbg!(&user);
    assert_eq!(user.id, "fr0zenfire");
}
