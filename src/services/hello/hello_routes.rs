use axum::{
	extract::State,
	http,
	routing::{get, post},
	Json, Router,
};
use std::sync::Arc;

use crate::config::Config;
use crate::errors::AppError;

use super::{HelloRequest, HelloResponse};

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
