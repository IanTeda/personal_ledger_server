//-- ./src/handlers/ping.rs

//! Server ping response handler, separate to health_check which provides metrics on the server
//!
//! # Ping Handler
//!
//! This module is used for handling requests and responses to `/ping`. The ping endpoint confirms
//! the server is up and running
//! 
//! # References
//!
//! * [A guide to API health check](https://testfully.io/blog/api-health-check-monitoring/)

// #![allow(unused)] // For beginning only.

use crate::prelude::*;

// use actix_web::HttpResponse;
use actix_web::HttpResponse;

/// [GET] `/ping` index endpoint handler
///
/// Handle the GET request and provide a response. Used by client services to confirm that api is
/// up and running.
pub async fn index() -> Result<HttpResponse> {
    let response = "Pong...".to_string();
    Ok(
        HttpResponse::Ok().body(response)
    )
}