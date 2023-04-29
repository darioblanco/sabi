use axum::{http, routing::get, Json, Router};
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
	pub status: String,
}

pub fn routes() -> Router {
	// /health
	Router::new().route("/", get(health))
}

async fn health() -> Result<Json<HealthResponse>, http::StatusCode> {
	let response = HealthResponse {
		status: "OK".to_string(),
	};
	Ok(Json(response))
}
