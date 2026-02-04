/* src/lib.rs */

//! `nvr` (Nested Variable Resolution) resolves mustache-style template strings.
//!
//! It supports nested variables like `{{kv.{{proto}}_backend}}` and leverages
//! `mst-parser` for template parsing and `varchain` for asynchronous variable lookups.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

extern crate alloc;

mod config;
mod error;

pub use config::{Config, NotFound};
pub use error::Error;

use alloc::boxed::Box;
use alloc::string::String;

#[cfg(feature = "tracing")]
use tracing::{debug, instrument};

/// Resolve with default configuration.
///
/// This is a convenience function that uses `Config::default()`.
pub async fn resolve_default(template: &str, scope: &varchain::Scope) -> Result<String, Error> {
	resolve(template, scope, &Config::default()).await
}

/// Resolve a template string using the given scope for variable lookup.
///
/// Parses the template with `mst-parser`, then recursively resolves variables
/// by looking them up in the `varchain` Scope.
#[cfg_attr(feature = "tracing", instrument(skip(scope, config)))]
pub async fn resolve(
	template: &str,
	scope: &varchain::Scope,
	config: &Config,
) -> Result<String, Error> {
	let nodes = mst_parser::Parser::new(config.parse).parse(template)?;
	let mut out = String::new();
	resolve_nodes(&nodes, scope, config, 0, &mut out).await?;
	Ok(out)
}

/// Recursively resolve AST nodes.
#[cfg_attr(feature = "tracing", instrument(skip(nodes, scope, config, out)))]
async fn resolve_nodes(
	nodes: &[mst_parser::Node],
	scope: &varchain::Scope,
	config: &Config,
	depth: usize,
	out: &mut String,
) -> Result<(), Error> {
	if depth > config.max_resolve_depth {
		return Err(Error::DepthExceeded {
			limit: config.max_resolve_depth,
		});
	}

	for node in nodes {
		match node {
			mst_parser::Node::Text(s) => {
				append_with_limit(out, s, config.max_result_size)?;
			}
			mst_parser::Node::Variable { parts } => {
				let mut key = String::new();
				Box::pin(resolve_nodes(parts, scope, config, depth + 1, &mut key)).await?;

				// Security check
				if key.contains('{') || key.contains('}') {
					return Err(Error::Injection { key });
				}

				#[cfg(feature = "tracing")]
				debug!(key = %key, "looking up variable");

				match scope.lookup(&key).await {
					Some(val) => {
						append_with_limit(out, &val, config.max_result_size)?;
					}
					None => match config.not_found {
						NotFound::ReturnOriginal => {
							append_with_limit(out, "{{", config.max_result_size)?;
							append_with_limit(out, &key, config.max_result_size)?;
							append_with_limit(out, "}}", config.max_result_size)?;
						}
						NotFound::ReturnEmpty => {}
						NotFound::Error => {
							return Err(Error::NotFound { key });
						}
					},
				}
			}
		}
	}

	Ok(())
}

/// Append a string to the output while checking against a size limit.
fn append_with_limit(out: &mut String, s: &str, limit: usize) -> Result<(), Error> {
	if out.len() + s.len() > limit {
		return Err(Error::SizeExceeded { limit });
	}
	out.push_str(s);
	Ok(())
}
