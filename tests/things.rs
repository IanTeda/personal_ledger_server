// Override with more flexible error
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

use once_cell::sync::Lazy;
use personal_ledger_server::{
    startup, telemetry,
};
use sqlx::{PgPool, Pool, Postgres};
use std::net::TcpListener;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = personal_ledger_server::configuration::LogLevels::Info;
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let tracing_subscriber = telemetry::get_tracing_subscriber(
            subscriber_name,
            std::io::stdout,
            personal_ledger_server::configuration::Environment::Development,
            default_filter_level,
        );
        telemetry::init_tracing(tracing_subscriber, default_filter_level);
    } else {
        let subscriber = telemetry::get_tracing_subscriber(
            subscriber_name,
            std::io::sink,
            personal_ledger_server::configuration::Environment::Development,
            default_filter_level,
        );
        telemetry::init_tracing(subscriber, default_filter_level);
    };
});

pub struct TestApp {
    pub address: String,
    pub database_pool: PgPool,
}

#[cfg(test)]
async fn spawn_app(pool: Pool<Postgres>) -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // We retrieve the port assigned to us by the OS through port "0"
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random OS port..");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}/api/v1", port);

    // let mut configuration: configuration::Settings =
    //     configuration::Settings::new()
    //         .expect("Failed to read configuration...");
        
    // // Assign a random database name to void test conflicts
    // configuration.database.database_name = Uuid::new_v4().to_string();
    // let connection_pool = init_test_database(&configuration.database).await;

    let server = startup::run(listener, pool.clone())
        .expect("Failed startup server...");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        database_pool: pool,
    }
}

#[sqlx::test]
async fn things_works(pool: Pool<Postgres>) -> Result<()> {
    //-- Setup and Fixtures (Arrange)
    let app = spawn_app(pool).await;
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
async fn post_things(pool: Pool<Postgres>) -> Result<()> {
    //-- Setup and Fixtures (Arrange)
    let app = spawn_app(pool).await;
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