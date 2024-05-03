use sqlx::{Pool, Postgres};

use crate::helpers::spawn_app;

// Override with more flexible error
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[sqlx::test]
async fn ping_works(database: Pool<Postgres>) -> Result<()> {
    // Arrange application for test
    let app = spawn_app(database).await?;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/ping", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Test assertion
    assert!(response.status().is_success());

    Ok(())
}