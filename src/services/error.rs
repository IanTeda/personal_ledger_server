//! Services (Database) error crate
//! 
//! This is passed through to the main error crate

#[derive(thiserror::Error, Debug)]
pub enum Error {

	#[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
	
	/// For starter, to remove as code matures.
	#[error("Generic error: {0}")]
	Generic(String),

	/// For starter, to remove as code matures.
	#[error("Static error: {0}")]
	Static(&'static str),

	#[error(transparent)]
	IO(#[from] std::io::Error),
}