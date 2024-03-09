use once_cell::sync::Lazy;
use personal_ledger_server::{startup, telemetry};
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
        telemetry::init_tracing(
            tracing_subscriber,
            default_filter_level,
        );
    } else {
        let subscriber = telemetry::get_tracing_subscriber(
            subscriber_name,
            std::io::sink,
            personal_ledger_server::configuration::Env::Development,
            default_filter_level,
        );
        telemetry::init_tracing(
            subscriber,
            default_filter_level,
        );
    };
});

pub struct TestApp {
    pub address: String,
}

async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let server = startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}/api/v1/", port);
    TestApp { address }
}

#[tokio::test]
async fn ping_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/ping", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}
