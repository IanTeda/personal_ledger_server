//! An end point route template for /api/v1/things

use crate::handlers::things;

use actix_web::web::{self};

/// # THINGS ROUTES
/// 
/// Configure Things end point calls 
/// 
/// # ATTRIBUTES
/// 
/// * `config`: is the Actix route config to build off
pub fn things(config: &mut web::ServiceConfig) {
    config
        .service(things::index)
        // .service(things::create)
        .route("/", web::post().to(things::create))
        .service(things::read)
        .service(things::update)
        .service(things::delete);
}