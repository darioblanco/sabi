use log::LevelFilter;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

pub trait Environment {
	fn get_var(&self, var: &str) -> Result<String, env::VarError>;
}

pub struct SystemEnvironment;

impl Environment for SystemEnvironment {
	fn get_var(&self, var: &str) -> Result<String, env::VarError> {
		env::var(var)
	}
}

#[derive(Clone, Debug)]
pub struct Config {
	pub api_address: SocketAddr,
	pub log_level: LevelFilter,
	pub version: Arc<String>,
}

impl Config {
	pub fn from_env<T: Environment>(env: &T) -> Config {
		dotenv::dotenv().ok();

		let api_address = env
			.get_var("API_ADDRESS")
			.unwrap_or_else(|_| "127.0.0.1".to_string());
		let api_port: u16 = env
			.get_var("API_PORT")
			.unwrap_or_else(|_| "3030".to_string())
			.parse()
			.unwrap_or(3030);
		let log_level = env
			.get_var("LOG_LEVEL")
			.unwrap_or_else(|_| "info".to_string());
		let version = env
			.get_var("VERSION")
			.unwrap_or_else(|_| "experimental".to_string());
		let version = Arc::new(version);

		let api_address = format!("{}:{}", api_address, api_port)
			.parse()
			.expect("Failed to parse API_ADDRESS and API_PORT");

		let log_level = match log_level.to_lowercase().as_str() {
			"trace" => LevelFilter::Trace,
			"debug" => LevelFilter::Debug,
			"info" => LevelFilter::Info,
			"warn" => LevelFilter::Warn,
			"error" => LevelFilter::Error,
			_ => LevelFilter::Info,
		};

		Config {
			api_address,
			log_level,
			version,
		}
	}

	pub fn from_params(version: String) -> Config {
		let api_address = "127.0.0.1".to_string();
		let api_port: u16 = 3030;
		let log_level = LevelFilter::Info;
		let version = Arc::new(version);

		let api_address = format!("{}:{}", api_address, api_port)
			.parse()
			.expect("Failed to parse API_ADDRESS and API_PORT");

		Config {
			api_address,
			log_level,
			version,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	struct MockEnvironment {
		vars: std::collections::HashMap<String, String>,
	}

	impl Environment for MockEnvironment {
		fn get_var(&self, var: &str) -> Result<String, env::VarError> {
			match self.vars.get(var) {
				Some(val) => Ok(val.to_owned()),
				None => Err(env::VarError::NotPresent),
			}
		}
	}

	#[test]
	fn test_config_from_env_defaults() {
		let env = MockEnvironment {
			vars: std::collections::HashMap::new(),
		};
		let config = Config::from_env(&env);
		assert_eq!(config.api_address, "127.0.0.1:3030".parse().unwrap());
		assert_eq!(config.log_level, LevelFilter::Info);
		assert_eq!(config.version.to_string(), "experimental".to_string());
	}

	#[test]
	fn test_config_from_env_custom() {
		let mut vars = std::collections::HashMap::new();
		vars.insert("API_ADDRESS".to_string(), "0.0.0.0".to_string());
		vars.insert("API_PORT".to_string(), "8080".to_string());
		vars.insert("LOG_LEVEL".to_string(), "warn".to_string());
		let env = MockEnvironment { vars };
		let config = Config::from_env(&env);
		assert_eq!(config.api_address, "0.0.0.0:8080".parse().unwrap());
		assert_eq!(config.log_level, LevelFilter::Warn);
		assert_eq!(config.version.to_string(), "experimental".to_string());
	}

	#[test]
	fn test_config_from_params() {
		let config = Config::from_params("test".to_string());
		assert_eq!(config.api_address, "127.0.0.1:3030".parse().unwrap());
		assert_eq!(config.log_level, LevelFilter::Info);
		assert_eq!(config.version.to_string(), "test".to_string());
	}
}
