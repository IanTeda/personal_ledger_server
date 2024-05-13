use once_cell::sync::Lazy;
use personal_ledger_server::{
	configuration::{Configuration, Environment, LogLevels},
	startup::Application,
	telemetry,
};
use sqlx::{PgPool, Pool, Postgres};

// Override with more flexible error
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
	let default_filter_level = LogLevels::Info;
	let subscriber_name = "test".to_string();
	if std::env::var("TEST_LOG").is_ok() {
		let tracing_subscriber = telemetry::get_tracing_subscriber(
			subscriber_name,
			std::io::stdout,
			Environment::Development,
			default_filter_level,
		);
		let _ = telemetry::init_tracing(tracing_subscriber, default_filter_level);
	} else {
		let subscriber = telemetry::get_tracing_subscriber(
			subscriber_name,
			std::io::sink,
			Environment::Development,
			default_filter_level,
		);
		let _ = telemetry::init_tracing(subscriber, default_filter_level);
	};
});

pub struct TestApp {
	pub address: String,
	pub database_pool: PgPool,
}

pub async fn spawn_app(database_pool: Pool<Postgres>) -> Result<TestApp> {
	Lazy::force(&TRACING);

	// Parse configuration files
	let configuration: Configuration = {
		let mut c = Configuration::parse().expect("Failed to read configuration.");
        // Setting port to `0` avoids conflicts as the OS will assign an unused 
        c.application.port = 0;
		c
	};

	// Launch the application as a background task
	let application =
		Application::build(configuration.clone(), database_pool.clone())
			.await
			.expect("Failed to build test application.");
	let address = format!("http://localhost:{}/api/v1", application.port());
	let _ = tokio::spawn(application.run_until_stopped());

	Ok(TestApp {
		address,
		database_pool,
	})
}
