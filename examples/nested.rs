/* examples/nested.rs */

//! Nested resolution example.

use nvr::resolve_default;
use std::collections::HashMap;
use varchain::Scope;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut kv = HashMap::new();
	kv.insert("env".to_string(), "prod".to_string());
	kv.insert("host_prod".to_string(), "10.0.0.1".to_string());
	kv.insert("host_dev".to_string(), "127.0.0.1".to_string());

	let scope = Scope::new().push(kv);

	// Resolves {{host_{{env}}}} -> {{host_prod}} -> 10.0.0.1
	let r = resolve_default("Connect to: {{host_{{env}}}}", &scope).await?;
	println!("Result: {}", r);
	assert_eq!(r, "Connect to: 10.0.0.1");

	Ok(())
}
