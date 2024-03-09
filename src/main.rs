//! ./main.rs
//!
//! # MAIN
//! 
//! The API main function.
//! 
//! Main functions are not async so we need some magic with #[actix_web::main]
use personal_ledger_server::{configuration, startup, telemetry};
use std::net::TcpListener;
use tracing::{debug, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    let address = format!("{}:{}", config.server.address, config.server.port);
    let listener = TcpListener::bind(address.clone())?;

    info!(
        "Starting API server at http://{}/api/v1 in {} environment",
        address, config.server.env
    );
    startup::run(listener)?.await?;
    Ok(())
}

// References
// https://github.com/actix/examples/tree/master/basics/nested-routing
// https://masteringbackend.com/posts/actix-web-the-ultimate-guide
