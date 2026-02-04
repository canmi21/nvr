/* examples/limits.rs */

use nvr::{Config, resolve};
use varchain::Scope;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let scope = Scope::new();

	// 1. Depth limit
	let mut config = Config::default();
	config.max_resolve_depth = 2;
	let r1 = resolve("{{{{{{too_deep}}}}}}", &scope, &config).await;
	println!("Depth limit (max 2) with depth 3: {:?}", r1);

	// 2. Size limit
	config.max_result_size = 10;
	let r2 = resolve("This is a very long string", &scope, &config).await;
	println!("Size limit (max 10) with long text: {:?}", r2);

	Ok(())
}
