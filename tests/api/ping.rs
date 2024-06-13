use crate::helpers::*;

use sqlx::{Pool, Postgres};

// Override with more flexible error
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[sqlx::test]
async fn ping_returns_200(database_pool: Pool<Postgres>) -> Result<()> {
    //-- Setup and Fixtures (Arrange)
    let app = spawn_app(database_pool).await?;
    let client = reqwest::Client::new();

    //-- Execute Test (Act)
    let response = client
        // Use the returned application address
        .get(&format!("{}/ping", &app.address))
        .send()
        .await
        .expect("Failed to execute ping request.");
    // println!("{response:#?}");

    //-- Checks (Assertions)
    // Check response status is success
    assert!(response.status().is_success());
    // Check http status is ok (200)
    assert_eq!(200, response.status().as_u16());
    // Check response contains "Pong...", length 7
    assert_eq!(Some(7), response.content_length());

    Ok(())
}