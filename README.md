# nvr

Nested Variable Resolution for mustache-style template strings.

`nvr` resolves template strings containing variables like `{{variable}}`, with full support for recursive nesting such as `{{kv.{{proto}}_backend}}`. It bridges `mst-parser` for template structure and `varchain` for flexible, async variable lookups.

## Features

- **Nested Resolution**: Resolve variables whose keys are themselves templates (e.g., `{{prefix_{{id}}}}`).
- **Async Lookup**: Integrated with `varchain` for non-blocking, chainable variable sources.
- **Safety Limits**: Configurable recursion depth and result size limits to prevent resource exhaustion.
- **Injection Protection**: Prevents malicious variable injection by validating resolved keys.
- **Flexible Missing Logic**: Choose between returning original tags, empty strings, or errors for missing variables.
- **no_std Support**: Compatible with `no_std + alloc` environments.

## Usage Examples

Check the `examples` directory for runnable code:

- **Simple Resolution**: [`examples/simple.rs`](examples/simple.rs) - Basic variable lookup.
- **Nested Resolution**: [`examples/nested.rs`](examples/nested.rs) - Resolving variables within variables.
- **Not Found Handling**: [`examples/not_found.rs`](examples/not_found.rs) - Different behaviors for missing variables.
- **Limits & Safety**: [`examples/limits.rs`](examples/limits.rs) - Depth and size limit enforcement.

### Quick Start

```rust
use nvr::{resolve, Config, NotFound};
use varchain::Scope;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kv = HashMap::new();
    kv.insert("proto".into(), "http".into());
    kv.insert("kv.http_backend".into(), "server-01".into());

    let scope = Scope::new().push(kv);
    let config = Config::default();

    // Nested variable: {{kv.http_backend}}
    let r = resolve("{{kv.{{proto}}_backend}}", &scope, &config).await?;
    assert_eq!(r, "server-01");
    
    Ok(())
}
```

## Installation

```toml
[dependencies]
nvr = { version = "0.x", features = ["full"] }
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `std` | Enables standard library support - enabled by default. |
| `tracing` | Enables optional tracing instrumentation for resolution steps. |
| `full` | Enables all features above. |

## License

Released under the MIT License © 2026 [Canmi](https://github.com/canmi21)