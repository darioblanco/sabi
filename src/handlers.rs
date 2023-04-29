use axum::response::IntoResponse;
use axum::{http, Json};
use hyper::StatusCode;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
	pub status: String,
}

pub async fn health() -> Result<Json<HealthResponse>, http::StatusCode> {
	let response = HealthResponse {
		status: "OK".to_string(),
	};
	Ok(Json(response))
}

pub async fn not_found() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "nothing to see here")
}
