use std::sync::Arc;

use async_session::Session;
use sabi_api::{
	config::Config,
	memory_store::MemoryStore,
	services::auth::{MultiOAuthConfig, MultiOAuthProvider, OAuthConfig},
	AppState,
};

#[derive(Clone)]
pub struct MockRedisStore {}

impl MockRedisStore {
	pub fn new() -> Self {
		Self {}
	}
}

#[async_trait::async_trait]
impl MemoryStore for MockRedisStore {
	async fn load_session(&self, cookie_value: String) -> async_session::Result<Option<Session>> {
		let mut session = Session::new();
		session.insert("user", cookie_value).unwrap();
		return Ok(Some(session));
	}

	async fn store_session(&self, _: Session) -> async_session::Result<Option<String>> {
		let value = "test".to_string();
		Ok(Some(value))
	}

	async fn destroy_session(&self, _: Session) -> async_session::Result {
		Ok(())
	}

	async fn clear_store(&self) -> async_session::Result {
		Ok(())
	}
}

pub fn create_state() -> AppState {
	let config = Arc::new(Config::from_params("test".to_string()));
	let memory_store: Arc<dyn MemoryStore> = Arc::new(MockRedisStore::new());
	let oauth_providers = Arc::new(MultiOAuthProvider::new(MultiOAuthConfig {
		discord: OAuthConfig {
			client_id: "secret".to_string(),
			client_secret: "secret".to_string(),
			redirect_url: "https://localhost".to_string(),
		},
		google: OAuthConfig {
			client_id: "secret".to_string(),
			client_secret: "secret".to_string(),
			redirect_url: "https://localhost".to_string(),
		},
	}));
	return AppState {
		config,
		memory_store,
		oauth_providers,
	};
}
