// -- ./src/api.rs

//! Configure routes for api endpoints
//! 
//! # API Routes
//! 
//! API routes are abstracted into version `v1` for future breaking changes, if needed

use crate::routes;

use actix_web::web;

/// Version 1 root level routes are set up in this function
/// 
/// # Parameters
/// 
/// * `config`: is the Actix route config to build off
pub fn v1(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/ping").configure(routes::ping))
        .service(web::scope("/things").configure(routes::things))
        .service(web::scope("/companies").configure(routes::companies));
}
