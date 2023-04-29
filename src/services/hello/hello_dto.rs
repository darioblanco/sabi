use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRequest {
	pub name: String,
}

#[derive(Debug, Serialize)]
pub struct HelloResponse {
	pub message: String,
	pub version: String,
}
