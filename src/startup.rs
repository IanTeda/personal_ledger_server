//!! ./src/startup.rs
//!
//! # STARTUP
//!
//!  API server startup
use crate::api;
use actix_web::{
    dev::Server, middleware, web, App, HttpServer
};
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
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            // Trim (normalise) trailing slashes `/`
            .wrap(middleware::NormalizePath::trim())
            // Configure API V1 scope
            .service(web::scope("/api/v1").configure(api::v1))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
