/* tests/integration.rs */

//! Integration tests for nvr crate.

use nvr::{Config, Error, NotFound, resolve};
use std::collections::HashMap;
use varchain::Scope;

#[tokio::test]
async fn test_simple_variable() {
	let mut kv = HashMap::new();
	kv.insert("key".to_string(), "value".to_string());
	let scope = Scope::new().push(kv);
	let config = Config::default();

	let r = resolve("Hello {{key}}!", &scope, &config).await.unwrap();
	assert_eq!(r, "Hello value!");
}

#[tokio::test]
async fn test_nested_variable() {
	let mut kv = HashMap::new();
	kv.insert("a".to_string(), "b".to_string());
	kv.insert("key_b".to_string(), "success".to_string());
	let scope = Scope::new().push(kv);
	let config = Config::default();

	let r = resolve("Result: {{key_{{a}}}}", &scope, &config)
		.await
		.unwrap();
	assert_eq!(r, "Result: success");
}

#[tokio::test]
async fn test_multi_level_nesting() {
	let mut kv = HashMap::new();
	kv.insert("1".to_string(), "2".to_string());
	kv.insert("2".to_string(), "3".to_string());
	kv.insert("val_3".to_string(), "deep".to_string());
	let scope = Scope::new().push(kv);
	let config = Config::default();

	let r = resolve("{{val_{{{{1}}}}}}", &scope, &config).await.unwrap();
	assert_eq!(r, "deep");
}

#[tokio::test]
async fn test_depth_limit() {
	let mut kv = HashMap::new();
	kv.insert("a".to_string(), "{{a}}".to_string()); // recursion if we resolved values, but we resolve keys
	let scope = Scope::new().push(kv);

	// Testing depth of nested keys
	let config = Config {
		max_resolve_depth: 2,
		..Config::default()
	};

	// {{ {{ {{a}} }} }} -> depth 3
	let res = resolve("{{{{{{a}}}}}}", &scope, &config).await;
	match res {
		Err(Error::DepthExceeded { limit }) => assert_eq!(limit, 2),
		_ => panic!("Expected DepthExceeded error, got {:?}", res),
	}
}

#[tokio::test]
async fn test_size_limit() {
	let mut kv = HashMap::new();
	kv.insert("long".to_string(), "1234567890".to_string());
	let scope = Scope::new().push(kv);

	let config = Config {
		max_result_size: 5,
		..Config::default()
	};

	let res = resolve("{{long}}", &scope, &config).await;
	match res {
		Err(Error::SizeExceeded { limit }) => assert_eq!(limit, 5),
		_ => panic!("Expected SizeExceeded error, got {:?}", res),
	}
}

#[tokio::test]
async fn test_injection_protection() {
	let mut kv = HashMap::new();
	kv.insert("malicious".to_string(), "secret}}".to_string());
	let scope = Scope::new().push(kv);
	let config = Config::default();

	// {{ {{malicious}} }} -> key becomes "secret}}"
	let res = resolve("{{{{malicious}}}}", &scope, &config).await;
	match res {
		Err(Error::Injection { key }) => assert_eq!(key, "secret}}"),
		_ => panic!("Expected Injection error, got {:?}", res),
	}
}

#[tokio::test]
async fn test_not_found_behaviors() {
	let scope = Scope::new(); // Empty scope

	// 1. ReturnOriginal
	let c1 = Config {
		not_found: NotFound::ReturnOriginal,
		..Config::default()
	};
	assert_eq!(
		resolve("{{missing}}", &scope, &c1).await.unwrap(),
		"{{missing}}"
	);

	// 2. ReturnEmpty
	let c2 = Config {
		not_found: NotFound::ReturnEmpty,
		..Config::default()
	};
	assert_eq!(resolve("{{missing}}", &scope, &c2).await.unwrap(), "");

	// 3. Error
	let c3 = Config {
		not_found: NotFound::Error,
		..Config::default()
	};
	let res = resolve("{{missing}}", &scope, &c3).await;
	match res {
		Err(Error::NotFound { key }) => assert_eq!(key, "missing"),
		_ => panic!("Expected NotFound error"),
	}
}

#[tokio::test]
async fn test_plain_text() {
	let scope = Scope::new();
	let config = Config::default();
	assert_eq!(
		resolve("just plain text", &scope, &config).await.unwrap(),
		"just plain text"
	);
}

#[tokio::test]

async fn test_empty_template() {
	let scope = Scope::new();

	let config = Config::default();

	assert_eq!(resolve("", &scope, &config).await.unwrap(), "");
}

#[tokio::test]

async fn test_depth_zero() {
	let scope = Scope::new();

	let config = Config {
		max_resolve_depth: 0,

		..Config::default()
	};

	// Any variable resolution requires at least depth 1 (the variable itself)

	let res = resolve("{{a}}", &scope, &config).await;

	match res {
		Err(Error::DepthExceeded { limit }) => assert_eq!(limit, 0),

		_ => panic!("Expected DepthExceeded(0), got {:?}", res),
	}

	// Plain text should still work at depth 0

	assert_eq!(resolve("hello", &scope, &config).await.unwrap(), "hello");
}

#[tokio::test]

async fn test_size_zero() {
	let scope = Scope::new();

	let config = Config {
		max_result_size: 0,

		..Config::default()
	};

	// Any output should fail

	let res = resolve("a", &scope, &config).await;

	match res {
		Err(Error::SizeExceeded { limit }) => assert_eq!(limit, 0),

		_ => panic!("Expected SizeExceeded(0), got {:?}", res),
	}

	// Empty template with size 0 should work

	assert_eq!(resolve("", &scope, &config).await.unwrap(), "");
}

#[tokio::test]

async fn test_mixed_variables() {
	let mut kv = HashMap::new();

	kv.insert("a".into(), "val_a".into());

	kv.insert("b".into(), "val_b".into());

	let scope = Scope::new().push(kv);

	let config = Config::default();

	let r = resolve("Start {{a}} and {{b}} End", &scope, &config)
		.await
		.unwrap();

	assert_eq!(r, "Start val_a and val_b End");
}
