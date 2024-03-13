//! ./src/handlers/mod.rs
//! 
//! # HANDLER FUNCTIONS
//! 
//! Handlers are async functions that receives request-based arguments, request
//! a service for data and return something a response.
//! 
pub mod ping;
pub mod things;
pub mod companies;

// pub use ping::*; // There is only one function so we do not need the glob `*`
// pub use things::*;
// pub use companies::*;
