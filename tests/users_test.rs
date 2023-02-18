use lichess_sdk::LichessClient;

#[tokio::test]
async fn test_get_status() {
    let lichess_client = LichessClient::new();
    assert!(lichess_client.is_ok());
    let ids = vec!["fr0zenfire"];
    let with_game_id = false;
    let result = lichess_client.unwrap().get_status(ids, with_game_id).await;
    assert!(result.is_ok());
    let status = result.unwrap();
    dbg!(&status);
    assert_eq!(status.len(), 1);
    assert_eq!(status.first().unwrap().id, "fr0zenfire");
}
