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
            personal_ledger_server::configuration::Env::Development,
            default_filter_level,
        );
        telemetry::init_tracing(tracing_subscriber, default_filter_level);
    } else {
        let subscriber = telemetry::get_tracing_subscriber(
            subscriber_name,
            std::io::sink,
            personal_ledger_server::configuration::Env::Development,
            default_filter_level,
        );
        telemetry::init_tracing(subscriber, default_filter_level);
    };
});

pub struct TestApp {
    pub address: String,
    pub database_pool: PgPool,
}

// TODO: Consider deleting as #[sqlx::test] macro does this for us, including deleting test database
// Initialise database for each test
// pub async fn init_test_database(database_config: &Database) -> PgPool {
//     debug!("Test database config used to initiate random test database: {:?}", database_config );

//     // Connect to database
//     let mut connection = PgConnection::connect_with(&database_config.without_database_name())
//         .await
//         .expect("Failed to connect to database instance...");

//     // Create random test database
//     connection
//         .execute(&*format!(r#"CREATE DATABASE "{}";"#, database_config.database_name))
//         .await
//         .expect("Failed to create random test database...");

//     // Connect to database pool using random test database
//     let connection_pool = sqlx::PgPool::connect_with(database_config.with_database_name())
//         .await
//         .expect("Failed to connect to random test database connection pool...");

//     // Apply database migrations to random test database
//     sqlx::migrate!("./migrations")
//         .run(&connection_pool)
//         .await
//         .expect("Failed to apply migrations to random test database...");

//     connection_pool
// }

async fn spawn_app(pool: Pool<Postgres>) -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // We retrieve the port assigned to us by the OS through port "0"
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random OS port..");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}/api/v1/", port);

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
async fn ping_works(pool: Pool<Postgres>) -> Result<()> {
    // Arrange application for test
    let app = spawn_app(pool).await;
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
