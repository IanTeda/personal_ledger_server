// -- ./startup.rs

//! A helper function to starting the Actix server.
//! ---

use crate::api;

use actix_web::{
    dev::Server, middleware, web::{self, Data}, App, HttpServer
};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server
}

// impl Application {
//     pub async fn build(configuration: Settings) -> crate::prelude::Result<Self> {
//         let connection_pool = get_connection_pool(&configuration.database);

//         let address = format!(
//             "{}:{}",
//             configuration.application.host, configuration.application.port
//         );
//         let listener = TcpListener::bind(address)?;
//         let port = listener.local_addr().unwrap().port();
//         let server = run(listener, connection_pool, email_client)?;

//         Ok(Self, {
//             port,
//             server
//         })
//     }
// }

/// Run the Actix Web HTTP server, returning an Actix server instance or IO Error
///
/// # Parameters
///
/// * `listener`: TCP Listener
/// ---
pub fn run(listener: TcpListener, database_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap database connection in a smart pointer
    let database = Data::new(database_connection_pool);
    // Create new Actix server
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
