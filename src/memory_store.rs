use async_session::{async_trait, Result, Session};
use redis::{AsyncCommands, Client};
use std::sync::Arc;

// TODO - These methods are implemented from async_session::SessionStore, but it causes problems with the Arc if we set the SessionStore trait
#[async_trait]
pub trait MemoryStore: Send + Sync {
	/// Get a session from the storage backend.
	///
	/// The input is expected to be the value of an identifying
	/// cookie. This will then be parsed by the session middleware
	/// into a session if possible
	async fn load_session(&self, cookie_value: String) -> Result<Option<Session>>;

	/// Store a session on the storage backend.
	///
	/// The return value is the value of the cookie to store for the
	/// user that represents this session
	async fn store_session(&self, session: Session) -> Result<Option<String>>;

	/// Remove a session from the session store
	async fn destroy_session(&self, session: Session) -> Result;

	/// Empties the entire store, destroying all sessions
	async fn clear_store(&self) -> Result;
}

#[derive(Clone, Debug)]
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
	async fn load_session(&self, cookie_value: String) -> async_session::Result<Option<Session>> {
		let key = format!("session:{}", cookie_value); // TODO - parse session id from cookie value
		let mut con = self.redis_client.get_async_connection().await.unwrap();
		let session_json: Option<String> = con.get(&key).await.unwrap();
		match session_json {
			Some(json) => Ok(Some(serde_json::from_str(&json)?)),
			None => Ok(None),
		}
	}

	async fn store_session(&self, session: Session) -> async_session::Result<Option<String>> {
		let key = format!("session:{}", session.id());
		let value = serde_json::to_string(&session)?;
		let mut con = self.redis_client.get_async_connection().await.unwrap();
		con.set::<_, _, ()>(&key, &value).await.unwrap();
		Ok(Some(value))
	}

	async fn destroy_session(&self, session: Session) -> async_session::Result {
		let key = format!("session:{}", session.id());
		let mut con = self.redis_client.get_async_connection().await.unwrap();
		con.del::<_, ()>(&key).await.unwrap();
		Ok(())
	}

	async fn clear_store(&self) -> async_session::Result {
		let mut con = self.redis_client.get_async_connection().await.unwrap();
		let mut cursor: usize = 0;
		loop {
			let res: (usize, Vec<String>) = redis::cmd("SCAN")
				.arg(cursor)
				.arg("MATCH")
				.arg("session:*")
				.query_async(&mut con)
				.await
				.unwrap();

			cursor = res.0;
			let keys: Vec<String> = res.1;

			// Delete the keys
			if !keys.is_empty() {
				let _: () = con.del(keys).await.unwrap();
			}

			// If the cursor is 0, we have completed the iteration
			if cursor == 0 {
				break;
			}
		}

		Ok(())
	}
}
