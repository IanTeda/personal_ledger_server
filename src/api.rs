// ./src/api.rs

///////////////////////////////////////////////////////////////////////////////
/// API SCOPE
/// API scope abstracted in versions to allow for breaking changes in the
/// future, if needed.

use actix_web::web;

use crate::routes;

pub fn v1(config: &mut web::ServiceConfig) {
    config.service(web::scope("/ping").configure(routes::ping));
}