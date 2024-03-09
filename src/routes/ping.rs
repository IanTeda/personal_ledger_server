//! ./src/routes/ping.rs
//! 
//! # PING ROUTE
//! 
//! End point route for /api/v1/ping
//! 
use actix_web::web;
use crate::handlers;

/// # PING
/// 
/// Configure ping end point calls 
/// 
/// ## ATTRIBUTES
/// 
/// * `config`: is the Actix route config to build off
pub fn ping(config: &mut web::ServiceConfig) {
    config.service(handlers::ping::index);
}
