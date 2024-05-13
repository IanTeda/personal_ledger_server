// -- ./src/errors.rs

//! Main error crate
//! 
//! Module (folder) error crates are imported throughs

/// Error static type enums
/// 
/// # References
/// 
/// * [Rust Error Handling - Best Practices](https://www.youtube.com/watch?v=j-VQCYP7wyw)
/// * [jeremychone-channel/rust-base](https://github.com/jeremychone-channel/rust-base)
/// * [derive(Error)](https://github.com/dtolnay/thiserror)
/// * [How to Handle Errors in Rust: A Comprehensive Guide](https://dev.to/nathan20/how-to-handle-errors-in-rust-a-comprehensive-guide-1cco)
/// * [Rust Error Types Explained: Building Robust Error Handling](https://marketsplash.com/rust-error-types/)
#[allow(dead_code)] // Remove not constructed error
#[derive(thiserror::Error, Debug)]
pub enum Error {
	//--

	/// For starter, to remove as code matures.
	#[error("Generic error: {0}")]
	Generic(String),

	/// For starter, to remove as code matures.
	#[error("Static error: {0}")]
	Static(&'static str),

	// -- Module Error Crates
	// #[error(transparent)]
	// Services(#[from] crate::personal_ledger_server::services::error::Error),

	// -- Externals
	#[error(transparent)]
	IO(#[from] std::io::Error),

	#[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

	#[error("Config error: {0}")]
    Config(#[from] config::ConfigError)
}