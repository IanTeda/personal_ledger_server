//-- ./src/routes/ping.rs

//! End point route for /api/v1/ping

use crate::handlers::ping;

use actix_web::web;

/// Configure ping end point calls
/// 
/// # Parameters
/// 
/// * `config`: is the Actix route config to build off
pub fn ping(config: &mut web::ServiceConfig) {
    config.route("",web::get().to(ping::index));
}