//! ./src/routes/mod.rs
//! 
//! # API ROUTES
//! 
//! Set up end point routes for calling handler functions

// TODO: implement a `/health_check` endpoint to provide.
mod ping;

pub use ping::*;