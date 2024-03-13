//! ./src/handlers/companies.rs
//!
//! # COMPANIES HANDLER
//!
//! Companies ledger CRUD route.
//!
//! * `C`reate implements `POST`
//! * `R`ead implements `GET`
//! * `U`pdate implements `PUT/PATCH`
//! * `D`elete implements `DELETE`
//!
use actix_web::{delete, get, post, put, HttpResponse, Responder};

/// # CREATE (POST) COMPANY
///
/// Create a Company record and respond with created instance
///
#[tracing::instrument(name = "Create Company")]
#[post("")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Create a Company record and respond its created instance...")
}

/// # READ (GET) COMPANY INDEX
///
/// Return a Company by ID
///
#[tracing::instrument(name = "Index Companies")]
#[get("")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Respond with a list (index) of Companies...")
}

/// # READ (GET) A COMPANY
///
/// Return a Company by ID
///
#[tracing::instrument(name = "Read Company")]
#[get("{company_id}")]
pub async fn read() -> impl Responder {
    HttpResponse::Ok().body("Find a Company by {company_id} and return instance...")
}

/// # UPDATE (PUT) A COMPANY
///
/// Find a Company by {company_id}, update and return instance
///
#[tracing::instrument(name = "Update Company")]
#[put("{company_id}")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Find a Company by {company_id}, update and return instance...")
}

/// # DELETE (DELETE) A COMPANY
///
/// Find a Company by {company_id}, update and return confirmation
///
#[tracing::instrument(name = "Delete Company")]
#[delete{"{company_id}"}]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Find a Company by {company_id}, update and return confirmation...")
}
