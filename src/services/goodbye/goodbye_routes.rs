use std::sync::Arc;

use axum::{http, routing::post, Json, Router};

use crate::{config::Config, errors::AppError};

use super::goodbye_dto::{GoodbyeRequest, GoodbyeResponse};

pub fn routes() -> Router<Arc<Config>> {
	// /goodbye
	Router::new()
		.route("/", post(goodbye_world))
		.route("/reason", post(goodbye_reason))
}

async fn goodbye_world() -> Result<Json<GoodbyeResponse>, http::StatusCode> {
	let response = GoodbyeResponse {
		message: "Goodbye, World!".to_string(),
	};
	Ok(Json(response))
}

async fn goodbye_reason(body: Json<GoodbyeRequest>) -> Result<Json<GoodbyeResponse>, AppError> {
	let reason = &body.0.reason;
	if reason.is_empty() {
		return Err(AppError::ValidationError {
			field: "reason".to_string(),
		});
	}
	let response = GoodbyeResponse {
		message: format!("Goodbye World! Reason: {}", reason),
	};
	Ok(Json(response))
}
