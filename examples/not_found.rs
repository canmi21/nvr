/* examples/not_found.rs */

//! Not found handling example.

use nvr::{Config, NotFound, resolve};
use varchain::Scope;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let scope = Scope::new(); // Empty scope

	// 1. Default: ReturnOriginal
	let r1 = resolve("{{missing}}", &scope, &Config::default()).await?;
	println!("Default (ReturnOriginal): {}", r1);

	// 2. ReturnEmpty
	let mut config = Config::default();
	config.not_found = NotFound::ReturnEmpty;
	let r2 = resolve("{{missing}}", &scope, &config).await?;
	println!("ReturnEmpty: '{}'", r2);

	// 3. Error
	config.not_found = NotFound::Error;
	let r3 = resolve("{{missing}}", &scope, &config).await;
	println!("Error mode result: {:?}", r3);

	Ok(())
}
