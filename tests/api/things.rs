//-- Override with more flexible error
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

//-- Common helper modules
use crate::helpers::spawn_app;
// use personal_ledger_server::domain::ThingName;

//-- External crate development dependencies
use fake::faker::lorem::en::{Sentence, Word};
use fake::Fake;
use sqlx::{Pool, Postgres};
use url::form_urlencoded;
use actix_web::body::MessageBody;
use personal_ledger_server::domain::{Thing, ThingDescription, ThingName};

#[sqlx::test]
async fn things_endpoint_works(database_pool: Pool<Postgres>) -> Result<()> {
    //-- Setup and Fixtures (Arrange)
    let app = spawn_app(database_pool).await?;
    let client = reqwest::Client::new();

    //-- Execute Test (Act)
    let response = client
        // Use the returned application address
        .get(&format!("{}/things", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    //-- Checks (Assertions)
    // Check response status is success
    assert!(response.status().is_success());
    // Check http status is ok (200)
    assert_eq!(200, response.status().as_u16());

    Ok(())
}

#[sqlx::test]
async fn post_things(database_pool: Pool<Postgres>) -> Result<()> {
    //-- Setup and Fixtures (Arrange)
    let app = spawn_app(database_pool).await?;
    let client = reqwest::Client::new();
    let name: String = Word().fake();
    let name_url: String = form_urlencoded::byte_serialize(name.as_bytes()).collect();
    let description: String = Sentence(3..7).fake();
    let description_url: String = form_urlencoded::byte_serialize(description.as_bytes()).collect();
    let body = format!("name={name}&description={description}", name=name_url, description=description_url);

    //-- Execute Test (Act)
    let response = client
        // Use the returned application address
        .post(&format!("{}/things", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;
    // println!("{response:#?}");

    //-- Checks (Assertions)
    // Check response status is success
    assert!(response.status().is_success());
    // Check http status is ok (200)
    assert_eq!(200, response.status().as_u16());

    // Parse the response body into bytes
    let body = response
        .text()
        .await?
        .try_into_bytes()?;
    // println!("{body:#?}");
    // Parse bytes into a Thing
    let thing: Thing = serde_json::from_slice(&body)?;
    // println!("{response_thing:#?}");

    //-- Check response contains to random name
    let response_thing_name = thing.name;
    let random_name = ThingName::parse(name)?;
    assert_eq!(response_thing_name, random_name);

    //-- Check response contains random description
    let response_thing_description = thing.description.unwrap();
    let random_description = ThingDescription::parse(description)?;
    assert_eq!(response_thing_description, random_description);

    Ok(())
}

// #[sqlx::test]
// async fn get_things_index(database_pool: Pool<Postgres>) -> Result<()> {
//     //-- Setup and Fixtures (Arrange)
//
//
//     //-- Execute Test (Act)
//
//
//     //-- Checks (Assertions)
//
//
//     Ok(())
// }