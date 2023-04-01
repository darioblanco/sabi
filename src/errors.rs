use serde::{Deserialize, Serialize};
use warp::reject::Reject;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
	pub message: String,
	pub code: Option<u32>,
}

impl ApiError {
	pub fn new(message: &str, code: Option<u32>) -> Self {
		ApiError {
			message: message.to_string(),
			code,
		}
	}
}

impl Reject for ApiError {}
