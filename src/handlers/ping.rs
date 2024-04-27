//! ./src/handlers/ping.rs
//! 
//! # PING HANDLER
//! 
//! This module is used for handling all requests and responses to `/ping`
//! 
//! #### REFERENCES
//! [A guide to API health check](https://testfully.io/blog/api-health-check-monitoring/)

use actix_web::{get, HttpResponse};

/// # PING - GET (INDEX)
/// 
/// A GET request on the `/ping` end point.
/// 
/// Used by client services to confirm that api is up.
#[get("")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Pong...")
}

