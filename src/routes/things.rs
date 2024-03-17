//! ./src/routes/things.rs
//! 
//! # THINGS ROUTE
//! 
//! An end point route template for /api/v1/things

use actix_web::web;
use crate::handlers;

/// # THINGS ROUTES
/// 
/// Configure Things end point calls 
/// 
/// ## ATTRIBUTES
/// 
/// * `config`: is the Actix route config to build off
pub fn things(config: &mut web::ServiceConfig) {
    config
        .service(handlers::things::index)
        .service(handlers::things::create)
        .service(handlers::things::read)
        .service(handlers::things::update)
        .service(handlers::things::delete);
}