use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GoodbyeRequest {
	pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct GoodbyeResponse {
	pub message: String,
}
