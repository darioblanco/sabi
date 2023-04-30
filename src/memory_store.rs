use async_session::{async_trait, Session};
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use tracing::error;

#[async_trait]
pub trait MemoryStore: Send + Sync {
	async fn set_user_session(&self, user_id: &str, session: &Session) -> String;
}

#[derive(Clone)]
pub struct RedisStore {
	redis_client: Arc<Client>,
}

impl RedisStore {
	pub async fn new(connection_url: String) -> Self {
		let client = redis::Client::open(connection_url).unwrap();

		// Test the connection
		let mut con = client.get_async_connection().await.unwrap();
		let _: () = con.set("test_key", "test_value").await.unwrap();
		let result: String = con.get("test_key").await.unwrap();
		println!("Redis test_key: {}", result);

		Self {
			redis_client: Arc::new(client),
		}
	}
}

#[async_trait]
impl MemoryStore for RedisStore {
	// async fn get(&self, key: &str) -> redis::RedisResult<String> {
	// 	let mut con = self.redis_client.get_async_connection().await?;
	// 	con.get(key).await
	// }

	async fn set_user_session(&self, user_id: &str, session: &Session) -> String {
		let mut con = self.redis_client.get_async_connection().await.unwrap();
		let key = format!("session:{}", user_id);
		let value = match serde_json::to_string(session) {
			Ok(val) => val,
			Err(e) => {
				error!("Failed to serialize session: {:?}", e);
				return String::new();
			}
		};
		let _: () = con.set::<_, _, ()>(&key, &value).await.unwrap(); // Specify the type parameters explicitly
		value
	}

	// pub async fn get_user_session(&self, user_id: &str) -> Option<DiscordUser> {
	// 	let key = format!("session:{}", user_id);
	// 	match self.get::<_, Option<String>>(&key).await {
	// 		Ok(Some(value)) => serde_json::from_str(&value).ok(),
	// 		_ => None,
	// 	}
	// }

	// pub async fn remove_user_session(&self, user_id: &str) -> redis::RedisResult<()> {
	// 	let key = format!("session:{}", user_id);
	// 	self.del(&key).await
	// }
}
