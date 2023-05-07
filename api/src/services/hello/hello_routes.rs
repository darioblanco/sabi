use axum::{
	extract::State,
	http,
	routing::{get, post},
	Json, Router,
};

use crate::errors::AppError;
use crate::AppState;

use super::{HelloRequest, HelloResponse};

pub fn routes() -> Router<AppState> {
	// /hello
	Router::new()
		.route("/", get(hello_world))
		.route("/", post(hello_with_params))
}

async fn hello_world(
	State(app_state): State<AppState>,
) -> Result<Json<HelloResponse>, http::StatusCode> {
	let response = HelloResponse {
		message: "Hello, World!".to_string(),
		version: app_state.config.version.to_string(),
	};
	Ok(Json(response))
}

async fn hello_with_params(
	State(app_state): State<AppState>,
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
		version: app_state.config.version.to_string(),
	};
	Ok(Json(response))
}
