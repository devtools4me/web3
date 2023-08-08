use dydx_bot::api;
use dydx_bot::configuration::Settings;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!("{\"value\":\"OK\"}", response.text().await.unwrap());
}

async fn spawn_app() {
    let server = api::routes::run_with_config(Settings::empty()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
}