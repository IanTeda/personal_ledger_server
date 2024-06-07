//! Module containing handler functions
//! 
//! Handlers are async functions that receives request-based arguments from routes.
//! The Handlers then request data from a service and return a response to the route.
//! 
pub mod ping;
pub mod things;
pub mod companies;
pub mod errors;

pub use errors::*;

// pub use ping::*; // There is only one function so we do not need the glob `*`
// pub use things::*;
// pub use companies::*;
