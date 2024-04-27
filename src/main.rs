use crate::prelude::*;

use personal_ledger_server::{configuration, startup, telemetry};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing::{debug, info};

//-- Re-export modules
mod error;
mod prelude;
mod utils;

/// The API main entry function 
/// 
/// Main functions are not async so we need some magic with `#[actix_web::main]``
#[actix_web::main]
async fn main() -> Result<()> {
    // Load configuration file
    let config: configuration::Settings =
        configuration::Settings::new().expect("Failed to read configuration.");

    let tracing_subscriber = telemetry::get_tracing_subscriber(
        "personal_ledger_server".into(),
        std::io::stdout,
        config.server.env,
        config.server.log_level
    );

    telemetry::init_tracing(tracing_subscriber, config.server.log_level);

    debug!(
        "\n----------- CONFIGURATION ----------- \n{:?} \n-------------------------------------",
        config
    );

    // TODO: Create configuration trait
    let address = format!("{}:{}", config.server.address, config.server.port);

    // TODO: do we need to reduce connection time in Docker builds
    let connection_pool = PgPoolOptions::new()
        .connect_lazy_with(config.database.with_database_name());
    info!("Connected to database {}", config.database.connection_url());
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    let listener = TcpListener::bind(address.clone())?;

    info!(
        "Starting API server at http://{}/api/v1 in {} environment",
        address, config.server.env
    );
    startup::run(listener, connection_pool)?.await?;
    Ok(())
}

// References
// https://github.com/actix/examples/tree/master/basics/nested-routing
// https://masteringbackend.com/posts/actix-web-the-ultimate-guide
