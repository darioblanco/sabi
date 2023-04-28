use axum::{
	extract::State,
	http,
	routing::{get, post},
	Json, Router,
};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

use crate::config::Config;
use crate::errors::AppError;

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRequest {
	pub name: String,
}

#[derive(Debug, Serialize)]
pub struct HelloResponse {
	pub message: String,
	pub version: String,
}

pub fn routes(config: Arc<Config>) -> Router {
	// /hello
	Router::new()
		.route("/", get(hello_world))
		.route("/", post(hello_with_params))
		.with_state(config)
}

async fn hello_world(
	State(config): State<Arc<Config>>,
) -> Result<Json<HelloResponse>, http::StatusCode> {
	let response = HelloResponse {
		message: "Hello, World!".to_string(),
		version: config.version.to_string(),
	};
	Ok(Json(response))
}

async fn hello_with_params(
	State(config): State<Arc<Config>>,
	Json(request_body): Json<HelloRequest>,
) -> Result<Json<HelloResponse>, AppError> {
	let name = &request_body.name;
	if name.is_empty() {
		return Err(AppError::ValidationError {
			field: "name".to_string(),
		});
	}
	let response = HelloResponse {
		message: format!("Hello, {}!", name),
		version: config.version.to_string(),
	};
	Ok(Json(response))
}
