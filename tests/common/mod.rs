use std::{collections::HashMap, sync::Arc};

use async_session::Session;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sabi::{config::Config, memory_store::MemoryStore, AppState};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct MockRedisStore {
	data: Arc<RwLock<HashMap<String, String>>>,
}

impl MockRedisStore {
	pub fn new() -> Self {
		Self {
			data: Arc::new(RwLock::new(HashMap::new())),
		}
	}
}

#[async_trait::async_trait]
impl MemoryStore for MockRedisStore {
	async fn set_user_session(&self, user_id: &str, session: &Session) -> String {
		let mut data = self.data.write().await;
		data.insert(user_id.to_string(), serde_json::to_string(session).unwrap());
		return "test".to_string();
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
