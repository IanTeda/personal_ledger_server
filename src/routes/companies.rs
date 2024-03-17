//! `api/v1/companies` endpoint configuration
//! 
//! ./src/routes/companies.rs
//! 
//! # COMPANIES ROUTE
//! 
//! An end point route template for /api/v1/companies
//! 
use actix_web::web;
use crate::handlers;

/// # COMPANY ROUTES
/// 
/// Configure Companies end point calls 
/// 
/// ## ATTRIBUTES
/// 
/// * `config`: is the Actix route config to build off
pub fn companies(config: &mut web::ServiceConfig) {
    config
        .service(handlers::companies::index)
        .service(handlers::companies::create)
        .service(handlers::companies::read)
        .service(handlers::companies::update)
        .service(handlers::companies::delete);
}