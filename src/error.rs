/* src/error.rs */

//! Error types for the `nvr` crate.

use alloc::string::String;

/// Error types for resolution.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Template parsing failed.
	#[error("parse error: {0}")]
	Parse(#[from] mst_parser::Error),
	/// Resolution depth exceeded.
	#[error("resolution depth exceeded limit: {limit}")]
	DepthExceeded {
		/// The depth limit that was exceeded.
		limit: usize,
	},
	/// Result size exceeded.
	#[error("result size exceeded limit: {limit}")]
	SizeExceeded {
		/// The size limit that was exceeded.
		limit: usize,
	},
	/// Key contains forbidden characters (injection protection).
	#[error("injection detected in key: {key}")]
	Injection {
		/// The key that caused the injection error.
		key: String,
	},
	/// Variable not found (only when NotFound::Error is configured).
	#[error("variable not found: {key}")]
	NotFound {
		/// The key that was not found.
		key: String,
	},
}
