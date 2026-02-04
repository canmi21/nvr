/* examples/simple.rs */

//! Simple resolution example.

use nvr::resolve_default;
use std::collections::HashMap;
use varchain::Scope;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut kv = HashMap::new();
	kv.insert("name".to_string(), "Gemini".to_string());

	let scope = Scope::new().push(kv);

	let r = resolve_default("Hello, {{name}}!", &scope).await?;
	println!("Result: {}", r);
	assert_eq!(r, "Hello, Gemini!");

	Ok(())
}
