//! ./src/handlers/mod.rs
//! 
//! # HANDLER FUNCTIONS
//! 
//! Handlers are async functions that receives request-based arguments, request
//! a service for data and return something a response.
pub mod ping;

pub use ping::*;
