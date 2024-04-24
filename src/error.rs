/// Error static type enums
/// 
/// # References
/// 
/// * [Rust Error Handling - Best Practices](https://www.youtube.com/watch?v=j-VQCYP7wyw)
/// * [jeremychone-channel/rust-base](https://github.com/jeremychone-channel/rust-base)
#[derive(thiserror::Error, Debug)]
pub enum Error {
	/// For starter, to remove as code matures.
	#[error("Generic error: {0}")]
	Generic(String),
	/// For starter, to remove as code matures.
	#[error("Static error: {0}")]
	Static(&'static str),

	#[error(transparent)]
	IO(#[from] std::io::Error),
}