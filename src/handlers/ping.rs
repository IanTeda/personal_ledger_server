// ./src/handlers/ping.rs

///////////////////////////////////////////////////////////////////////////////
/// PING END POINT
/// Used by client services to confirm that api is up.
use actix_web::{get, HttpResponse, Responder};

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Pong.")
}

// REFERENCES
// https://testfully.io/blog/api-health-check-monitoring/
