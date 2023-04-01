use std::env;
use std::net::SocketAddr;
use tracing::Level;

pub trait Environment {
	fn get_var(&self, var: &str) -> Result<String, env::VarError>;
}

pub struct SystemEnvironment;

impl Environment for SystemEnvironment {
	fn get_var(&self, var: &str) -> Result<String, env::VarError> {
		env::var(var)
	}
}

#[derive(Debug)]
pub struct Config {
	pub api_address: SocketAddr,
	pub log_level: Level,
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

		let api_address = format!("{}:{}", api_address, api_port)
			.parse()
			.expect("Failed to parse API_ADDRESS and API_PORT");

		let log_level = match log_level.to_lowercase().as_str() {
			"trace" => Level::TRACE,
			"debug" => Level::DEBUG,
			"info" => Level::INFO,
			"warn" => Level::WARN,
			"error" => Level::ERROR,
			_ => Level::INFO,
		};

		Config {
			api_address,
			log_level,
		}
	}
}
