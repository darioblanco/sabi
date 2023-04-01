use senjin::config::{Config, Environment};
use std::collections::HashMap;
use std::env;
use std::net::{Ipv4Addr, SocketAddr};
use tracing::Level;

pub struct TestEnvironment {
	vars: HashMap<String, String>,
}

impl TestEnvironment {
	pub fn new() -> Self {
		TestEnvironment {
			vars: HashMap::new(),
		}
	}

	pub fn set_var(&mut self, key: &str, value: &str) {
		self.vars.insert(key.to_string(), value.to_string());
	}
}

impl Environment for TestEnvironment {
	fn get_var(&self, var: &str) -> Result<String, env::VarError> {
		self.vars
			.get(var)
			.map(|v| v.clone())
			.ok_or(env::VarError::NotPresent)
	}
}

#[test]
fn test_config_default_values() {
	let config = Config::from_env(&TestEnvironment::new());

	assert_eq!(
		config.api_address,
		SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 3030)
	);
	assert_eq!(config.log_level, Level::INFO);
}

#[test]
fn test_config_custom_values() {
	let mut test_env = TestEnvironment::new();
	test_env.set_var("API_ADDRESS", "192.168.1.1");
	test_env.set_var("API_PORT", "8080");
	test_env.set_var("LOG_LEVEL", "debug");

	let config = Config::from_env(&test_env);

	assert_eq!(
		config.api_address,
		SocketAddr::new(Ipv4Addr::new(192, 168, 1, 1).into(), 8080)
	);
	assert_eq!(config.log_level, Level::DEBUG);
}
