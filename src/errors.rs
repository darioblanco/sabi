use axum::{body::BoxBody, http::Response, response::IntoResponse, Json};
use derive_more::{Display, Error};
use hyper::StatusCode;
use serde_json::json;

#[derive(Debug, Display, Error)]
/// The app's top level error type.
pub enum AppError {
	#[display(fmt = "An internal error occurred. Please try again later.")]
	InternalError,
	#[display(fmt = "Validation error on field: {}", field)]
	ValidationError { field: String },
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response<BoxBody> {
		let (status, error_message) = match self {
			AppError::ValidationError { .. } => (StatusCode::BAD_REQUEST, "invalid request"),
			AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "oops"),
		};

		let body = Json(json!({
			"error": error_message,
		}));

		(status, body).into_response()
	}
}
