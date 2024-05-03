use sqlx::{Pool, Postgres};

use crate::helpers::spawn_app;

// Override with more flexible error
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[sqlx::test]
async fn things_works(database: Pool<Postgres>) -> Result<()> {
    //-- Setup and Fixtures (Arrange)
    let app = spawn_app(database).await?;
    let client = reqwest::Client::new();

    //-- Execute Function (Act)
    let response = client
        // Use the returned application address
        .get(&format!("{}/things", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    //-- Checks (Assertions)
    assert!(response.status().is_success());

    Ok(())
}

#[sqlx::test]
async fn post_things(database: Pool<Postgres>) -> Result<()> {
    //-- Setup and Fixtures (Arrange)
    let app = spawn_app(database).await?;
    let client = reqwest::Client::new();
    let body = "name=Test%20ThingNoSpace&description=This%20is%20a%20description";

    //-- Execute Function (Act)
    let response: reqwest::Response = client
        // Use the returned application address
        .post(&format!("{}/things", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // println!("{response:#?}");

    //-- Checks (Assertions)
    assert!(response.status().is_success());
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, description FROM things",)
        .fetch_one(&app.database_pool)
        .await
        .expect("Failed to fetch saved thing.");

    assert_eq!(saved.name, "Test ThingNoSpace");
    assert_eq!(saved.description.unwrap(), "This is a description");

    Ok(())
}