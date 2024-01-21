use crate::api;
use actix_web::{
    dev::Server,
    middleware::{self, Logger},
    web, App, HttpServer,
};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // Initate logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server = HttpServer::new(move || {
        App::new()
            // Wrap env_loger
            .wrap(Logger::default())
            // Trim (normalise) trailing slashes `/`
            .wrap(middleware::NormalizePath::trim())
            // Configure API V1 scope
            .service(web::scope("/api/v1").configure(api::v1))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
