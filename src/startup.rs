//!! ./src/startup.rs
//!
//! # STARTUP
//!
//!  API server startup
use crate::api;
use actix_web::{
    dev::Server, middleware, web::{self, Data}, App, HttpServer
};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;


/// # RUN
///
/// Run the Actix Web HTTP server
///
/// ## ATTRIBUTES
///
/// * `listener`: TCP Listener
///
///  ## RETURNS
///
/// Returns a result with a Actix server instance or IO Error
pub fn run(listener: TcpListener, database_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap database connection in a smart pointer
    let database = Data::new(database_connection_pool);
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
