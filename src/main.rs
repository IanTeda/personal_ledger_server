use crate::prelude::*;
use crate::configuration::Configuration;

use personal_ledger_server::{configuration, startup::{self, Application}, telemetry};

//-- Re-export modules
mod error;
mod prelude;
mod utils;

/// The API main entry function 
/// 
/// Main functions are not async so we need some magic with `#[actix_web::main]``
#[actix_web::main]
async fn main() -> Result<()> {
    // Parse configuration files
    let configuration: Configuration =
        Configuration::parse()
        .expect("Failed to read configuration.");

    // Build tracing subscriber
    let tracing_subscriber = telemetry::get_tracing_subscriber(
        "personal_ledger_server".into(),
        std::io::stdout,
        configuration.application.runtime_environment,
        configuration.application.log_level
    );
    telemetry::init_tracing(
        tracing_subscriber, 
        configuration.application.log_level
    );

    let database_pool = startup::get_connection_pool(&configuration.database)
        .await
        .expect("Error connecting to database");

    let application = Application::build(configuration, database_pool)
        .await
        .expect("Error building application");
    application.run_until_stopped()
        .await
        .expect("Error running application");

    Ok(())
}

// References
// https://github.com/actix/examples/tree/master/basics/nested-routing
// https://masteringbackend.com/posts/actix-web-the-ultimate-guide
