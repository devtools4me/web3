use web3_trader::api;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let server = api::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}