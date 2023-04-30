use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
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

#[derive(Clone, Debug)]
pub struct Config {
	pub api_address: SocketAddr,
	pub discord: DiscordConfig,
	pub log_level: Level,
	pub redis_url: Arc<String>,
	pub version: Arc<String>,
}

#[derive(Clone, Debug)]
pub struct DiscordConfig {
	pub auth_url: Arc<String>,
	pub client_id: Arc<String>,
	pub client_secret: Arc<String>,
	pub redirect_url: Arc<String>,
	pub token_url: Arc<String>,
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
		let discord_auth_url = env.get_var("DISCORD_AUTH_URL").unwrap_or_else(|_| {
			"https://discord.com/api/oauth2/authorize?response_type=code".to_string()
		});
		let discord_client_id = env
			.get_var("DISCORD_CLIENT_ID")
			.expect("Missing Discord client id!");
		let discord_client_secret = env
			.get_var("DISCORD_CLIENT_SECRET")
			.expect("Missing Discord client secret!");
		let discord_redirect_url = env
			.get_var("DISCORD_REDIRECT_URL")
			.unwrap_or_else(|_| "http://127.0.0.1:3000/auth/authorized".to_string());
		let discord_token_url = env
			.get_var("DISCORD_TOKEN_URL")
			.unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());
		let log_level = env
			.get_var("LOG_LEVEL")
			.unwrap_or_else(|_| "info".to_string());
		let redis_url = env
			.get_var("REDIS_URL")
			.unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
		let version = env
			.get_var("VERSION")
			.unwrap_or_else(|_| "experimental".to_string());
		let version = Arc::new(version);

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
			discord: DiscordConfig {
				auth_url: Arc::new(discord_auth_url),
				client_id: Arc::new(discord_client_id),
				client_secret: Arc::new(discord_client_secret),
				redirect_url: Arc::new(discord_redirect_url),
				token_url: Arc::new(discord_token_url),
			},
			log_level,
			redis_url: Arc::new(redis_url),
			version,
		}
	}

	pub fn from_params(version: String) -> Config {
		let version = Arc::new(version);

		let api_address = "127.0.0.1".to_string();
		let api_port: u16 = 3030;
		let api_address = format!("{}:{}", api_address, api_port)
			.parse()
			.expect("Failed to parse API_ADDRESS and API_PORT");

		Config {
			api_address,
			discord: DiscordConfig {
				auth_url: Arc::new("test".to_string()),
				client_id: Arc::new("test".to_string()),
				client_secret: Arc::new("test".to_string()),
				redirect_url: Arc::new("test".to_string()),
				token_url: Arc::new("test".to_string()),
			},
			log_level: Level::INFO,
			redis_url: Arc::new("redis://127.0.0.1/".to_string()),
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
		let mut vars = std::collections::HashMap::new();
		vars.insert("DISCORD_CLIENT_ID".to_string(), "secret".to_string());
		vars.insert("DISCORD_CLIENT_SECRET".to_string(), "secret".to_string());
		let env = MockEnvironment { vars };
		let config = Config::from_env(&env);
		assert_eq!(config.api_address, "127.0.0.1:3030".parse().unwrap());
		assert_eq!(
			config.discord.auth_url.to_string(),
			"https://discord.com/api/oauth2/authorize?response_type=code".to_string()
		);
		assert_eq!(config.discord.client_id.to_string(), "secret".to_string());
		assert_eq!(
			config.discord.client_secret.to_string(),
			"secret".to_string()
		);
		assert_eq!(
			config.discord.redirect_url.to_string(),
			"http://127.0.0.1:3000/auth/authorized".to_string()
		);
		assert_eq!(
			config.discord.token_url.to_string(),
			"https://discord.com/api/oauth2/token".to_string()
		);
		assert_eq!(
			config.redis_url.to_string(),
			"redis://127.0.0.1/".to_string()
		);
		assert_eq!(config.log_level, Level::INFO);
		assert_eq!(config.version.to_string(), "experimental".to_string());
	}

	#[test]
	fn test_config_from_env_custom() {
		let mut vars = std::collections::HashMap::new();
		vars.insert("API_ADDRESS".to_string(), "0.0.0.0".to_string());
		vars.insert("API_PORT".to_string(), "8080".to_string());
		vars.insert(
			"DISCORD_AUTH_URL".to_string(),
			"https://authurl".to_string(),
		);
		vars.insert("DISCORD_CLIENT_ID".to_string(), "secret".to_string());
		vars.insert("DISCORD_CLIENT_SECRET".to_string(), "secret".to_string());
		vars.insert(
			"DISCORD_REDIRECT_URL".to_string(),
			"https://redirecturl".to_string(),
		);
		vars.insert(
			"DISCORD_TOKEN_URL".to_string(),
			"https://tokenurl".to_string(),
		);
		vars.insert("REDIS_URL".to_string(), "myredis://127.0.0.1/".to_string());
		vars.insert("LOG_LEVEL".to_string(), "warn".to_string());
		let env = MockEnvironment { vars };
		let config = Config::from_env(&env);
		assert_eq!(config.api_address, "0.0.0.0:8080".parse().unwrap());
		assert_eq!(
			config.discord.auth_url.to_string(),
			"https://authurl".to_string()
		);
		assert_eq!(config.discord.client_id.to_string(), "secret".to_string());
		assert_eq!(
			config.discord.client_secret.to_string(),
			"secret".to_string()
		);
		assert_eq!(
			config.discord.redirect_url.to_string(),
			"https://redirecturl".to_string()
		);
		assert_eq!(
			config.discord.token_url.to_string(),
			"https://tokenurl".to_string()
		);
		assert_eq!(
			config.redis_url.to_string(),
			"myredis://127.0.0.1/".to_string()
		);
		assert_eq!(config.log_level, Level::WARN);
		assert_eq!(config.version.to_string(), "experimental".to_string());
	}

	#[test]
	fn test_config_from_params() {
		let config = Config::from_params("test".to_string());
		assert_eq!(config.api_address, "127.0.0.1:3030".parse().unwrap());
		assert_eq!(config.discord.auth_url.to_string(), "test".to_string());
		assert_eq!(config.discord.client_id.to_string(), "test".to_string());
		assert_eq!(config.discord.client_secret.to_string(), "test".to_string());
		assert_eq!(config.discord.redirect_url.to_string(), "test".to_string());
		assert_eq!(config.discord.token_url.to_string(), "test".to_string());
		assert_eq!(config.log_level, Level::INFO);
		assert_eq!(
			config.redis_url.to_string(),
			"redis://127.0.0.1/".to_string()
		);
		assert_eq!(config.version.to_string(), "test".to_string());
	}
}
