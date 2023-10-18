use crate::helpers::{spawn_app, drop_database};

#[tokio::test]
async fn health_check_works() {
    let (app, database_config) = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    drop_database(&database_config).await;
}