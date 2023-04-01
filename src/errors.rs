use actix_web::{
	error,
	http::{header::ContentType, StatusCode},
	HttpResponse,
};
use derive_more::{Display, Error};
use serde_derive::Serialize;

#[derive(Debug, Display, Error)]
pub enum UserError {
	#[display(fmt = "An internal error occurred. Please try again later.")]
	InternalError,
	#[display(fmt = "Validation error on field: {}", field)]
	ValidationError { field: String },
}

impl error::ResponseError for UserError {
	fn error_response(&self) -> HttpResponse {
		HttpResponse::build(self.status_code())
			.insert_header(ContentType::html())
			.body(self.to_string())
	}
	fn status_code(&self) -> StatusCode {
		match *self {
			UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
			UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}

#[derive(Debug, Display, Serialize)]
struct ErrorResponse {
	error: String,
}

impl error::ResponseError for ErrorResponse {}

pub async fn error_handler(
	err: actix_web::Error,
	_: &actix_web::dev::ServiceRequest,
) -> actix_web::Result<HttpResponse> {
	let status_code = err.as_response_error().status_code();
	let error_response = ErrorResponse {
		error: err.to_string(),
	};
	Ok(HttpResponse::build(status_code).json(error_response))
}
