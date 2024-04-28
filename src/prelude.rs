// ./src/prelude.rs

//! To be included in all models, i.e. api server prelude.
//!
//! These are the most common items used by the personal_ledger/server code in 
//! intended to be imported by all server code, for convenience.

// Re-export the crate Error.
pub use crate::error::Error;

// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for new-type pattern,
// mostly for external type to type From/TryFrom conversions
// pub struct W<T>(pub T);

// Personal preference.
// pub use std::format as f;

// #[cfg(test)]
// pub(crate) use crate::tests::*;