// -- ./startup.rs

//! A helper function to starting the Actix server.
//! ---

use crate::api;
use crate::prelude::*;
use crate::configuration::*;

use actix_web::dev::Server;
use actix_web::middleware;
use actix_web::web;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpServer;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    pub async fn build(configuration: Configuration, connection_pool: PgPool) -> Result<Self> {
        let address = format!(
            "{}:{}",
            configuration.application.address, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener, connection_pool)?;

        tracing::info!(
            "Starting API server at http://{}:{}/api/v1 in {} environment",
            configuration.application.address, 
            port,
            configuration.application.runtime_environment
        );

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        let application = self.server.await?;
        return Ok(application)
    }
}

pub async fn get_connection_pool(database: &DatabaseSettings) -> Result<PgPool> {
    let connection_pool = PgPoolOptions::new().connect_lazy_with(database.connection());
    tracing::info!("Connected to database: {}", database.connection_url());
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");
    Ok(connection_pool)
}

fn run(
    listener: TcpListener,
    database_pool: PgPool,
) -> Result<Server> {
    // Wrap database pool around Actix Data type
    let database = Data::new(database_pool);
    // Actix server
    let server = HttpServer::new(move || {
		App::new()
			// Actix tracing log middleware
			.wrap(TracingLogger::default())
			// Trim (normalise) trailing slashes `/`
			.wrap(middleware::NormalizePath::trim())
			// Configure API V1 scope
			.service(web::scope("/api/v1").configure(api::v1))
			// Attach database to the Actix application state
			.app_data(database.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}