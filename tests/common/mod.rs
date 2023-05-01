use std::sync::Arc;

use async_session::Session;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sabi::{config::Config, memory_store::MemoryStore, AppState};

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
	let oauth_client = BasicClient::new(
		ClientId::new("secret".to_string()),
		Some(ClientSecret::new("secret".to_string())),
		AuthUrl::new("https://localhost".to_string()).unwrap(),
		Some(TokenUrl::new("https://localhost".to_string()).unwrap()),
	)
	.set_redirect_uri(RedirectUrl::new("https://localhost".to_string()).unwrap());
	return AppState {
		config,
		memory_store,
		oauth_client,
	};
}
