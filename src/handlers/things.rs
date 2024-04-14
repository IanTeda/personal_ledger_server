//! ./src/handlers/things.rs
//!
//! # THINGS HANDLER
//!
//! A template for creating a CRUD route.
//!
//! * `C`reate implements `POST`
//! * `R`ead implements `GET`
//! * `U`pdate implements `PUT/PATCH`
//! * `D`elete implements `DELETE`
//!
//!
//! |- NAME -|- DESCRIPTION -|- SQL EQUIVALENT -|
//! | Create | Adds one or more new entries | Insert |
//! | Read | Retrieves entries that match certain criteria (if there are any) | Select |
//! | Update | Changes specific fields in existing entries | Update |
//! | Delete | Entirely removes one or more existing entries | Delete |
//!
use actix_web::{delete, get, post, put, HttpResponse, Responder};

/// # CREATE (POST) THING
///
/// Create a thing record and respond its created instance
///
#[tracing::instrument(name = "Create things")]
#[post("")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Create a Thing record and respond its created instance...")
}

/// # READ (GET) THING INDEX
///
/// Return a thing by ID
///
#[tracing::instrument(name = "Index things")]
#[get("")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Respond with a list (index) of things...")
}

/// # READ (GET) A THING
///
/// Return a thing by ID
///
#[tracing::instrument(name = "Read things")]
#[get("{thing_id}")]
pub async fn read() -> impl Responder {
    HttpResponse::Ok().body("Find a Thing by {thing_id} and return instance...")
}

/// # UPDATE (PUT) A THING
///
/// Find a Thing by {thing_id}, update and return instance
///
#[tracing::instrument(name = "Update things")]
#[put("{thing_id}")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Find a Thing by {thing_id}, update and return instance...")
}

/// # DELETE (DELETE) A THING
///
/// Find a Thing by {thing_id}, update and return confirmation
///
#[tracing::instrument(name = "Delete things")]
#[delete{"{thing_id}"}]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Find a Thing by {thing_id}, update and return confirmation...")
}
