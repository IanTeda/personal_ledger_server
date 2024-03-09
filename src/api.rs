//! ./src/api.rs

//! API ROUTES
//! 
//! API routes are abstracted into version `v1` for future breaking changes, if 
//! needed

use actix_web::web;
use crate::routes;

/// VERSION 1 API ROUTES
/// 
/// Root level routes are set up in this function
/// 
/// ## ATTRIBUTES
/// 
/// * `config`: is the Actix route config to build off
pub fn v1(config: &mut web::ServiceConfig) {
    config.service(web::scope("/ping").configure(routes::ping));
}