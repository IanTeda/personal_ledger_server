//! ./src/routes/mod.rs
//! 
//! # API ROUTES
//! 
//! Configure methods for each api end point.
//! Each method calls a handler

// TODO: implement a `/health_check` endpoint to provide.
mod ping;
mod things;
mod companies;

pub use ping::*;
pub use things::*;
pub use companies::*;
