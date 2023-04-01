use super::errors::ApiError;
use super::models::Greeting;
use std::convert::Infallible;
use tracing::{info, instrument};
use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[instrument]
pub async fn handle_hello(name: String) -> Result<impl Reply, Rejection> {
	info!("Received request for name: {}", name);

	// Just an example to demonstrate error handling; replace with actual error conditions
	if name.len() < 3 {
		return Err(warp::reject::custom(ApiError::new(
			"Name is too short",
			Some(1001),
		)));
	}

	let greeting = Greeting {
		message: format!("Hello, {}!", name),
	};
	Ok(warp::reply::json(&greeting))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
	let code;
	let message;

	if err.is_not_found() {
		code = StatusCode::NOT_FOUND;
		message = "Not Found";
	} else if let Some(api_error) = err.find::<ApiError>() {
		code = StatusCode::BAD_REQUEST;
		message = &api_error.message;
	} else {
		eprintln!("unhandled rejection: {:?}", err);
		code = StatusCode::INTERNAL_SERVER_ERROR;
		message = "Internal Server Error";
	}

	let error = ApiError::new(message, None);
	let json = warp::reply::json(&error);
	Ok(warp::reply::with_status(json, code))
}
