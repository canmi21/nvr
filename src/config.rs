/* src/config.rs */

//! Configuration types for nested variable resolution.

/// Behavior when a variable is not found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotFound {
	/// Return the original template string, e.g., "{{key}}".
	#[default]
	ReturnOriginal,
	/// Return an empty string.
	ReturnEmpty,
	/// Return an error.
	Error,
}

/// Configuration for resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
	/// Parser limits (max depth, max nodes).
	pub parse: mst_parser::Limits,
	/// Resolution recursion limit.
	pub max_resolve_depth: usize,
	/// Result size limit in bytes.
	pub max_result_size: usize,
	/// Behavior when a variable is not found.
	pub not_found: NotFound,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			parse: mst_parser::Limits::default(),
			max_resolve_depth: 5,
			max_result_size: 65536,
			not_found: NotFound::default(),
		}
	}
}
