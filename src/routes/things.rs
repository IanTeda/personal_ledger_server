// -- ./src/routes/things.rs

//! A template for defining the end point route for `/api/v1/things``

use crate::handlers::things;

use actix_web::web;

/// Configure endpoint routes for `things`
/// 
/// # Things Routes 
/// 
/// Define the CRUD routes for `/things`
/// 
/// # Parameters
/// 
/// * `config`: is the Actix route config to build off
/// ---
pub fn things(config: &mut web::ServiceConfig) {
    config
        // .service(things::index)
        .route("", web::get().to(things::read_index))
        // .service(things::create)
        .route("", web::post().to(things::create))
        // .service(things::read)
        .route("{thing_id}", web::get().to(things::read_by_id))
        // .service(things::update)
        .route("{thing_id}", web::put().to(things::update_by_id))
        // .service(things::delete);
        .route("{thing_id}", web::delete().to(things::delete_by_id));
}