// -- ./src/errors.rs
#![allow(unused)] // For beginning only.

//! Main error crate
//! 
//! Module (folder) error crates are imported through

use actix_web::{http::StatusCode, ResponseError};

/// Error static type enums
/// 
/// # References
/// 
/// * [Rust Error Handling - Best Practices](https://www.youtube.com/watch?v=j-VQCYP7wyw)
/// * [jeremychone-channel/rust-base](https://github.com/jeremychone-channel/rust-base)
/// * [derive(Error)](https://github.com/dtolnay/thiserror)
/// * [How to Handle Errors in Rust: A Comprehensive Guide](https://dev.to/nathan20/how-to-handle-errors-in-rust-a-comprehensive-guide-1cco)
/// * [Rust Error Types Explained: Building Robust Error Handling](https://marketsplash.com/rust-error-types/)
#[derive(thiserror::Error, Debug)]
pub enum Error {
	//-- Generic starter errors, remove as code matures
	#[error("Generic error: {0}")]
	Generic(String),

	#[error("Static error: {0}")]
	Static(&'static str),

	// -- Module Error Crates
    // #[error(transparent)]
	// Handlers(#[from] handlers::Error),

	#[error("There is no Thing associated with the provided id.")]
    ThingUnknownId,
	#[error("There is no Thing associated with the provided name.")]
    ThingUnknownName,
	#[error("Parameter missing from query.")]
	ParameterMissing,

	// #[error(transparent)]
	// Services(#[from] crate::personal_ledger_server::services::error::Error),

	// -- Externals
	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error(transparent)]
    Database(#[from] sqlx::Error),

	#[error(transparent)]
    Config(#[from] config::ConfigError),
}

// impl Error {
// 	pub fn custom(val: impl std::fmt::Display) -> Self {
// 		Self::custom(val.to_string())
// 	}
// }

// Better formatting
// impl std::fmt::Debug for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         error_chain_fmt(self, f)
//     }
// }

// Convert into a Actix::ResponseError 
impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
			Self::ThingUnknownName => StatusCode::BAD_REQUEST,
            Self::ThingUnknownId => StatusCode::BAD_REQUEST,
			_ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}