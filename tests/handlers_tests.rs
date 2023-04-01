use hyper::{Body, Response};
use senjin::errors::ApiError;
use senjin::handlers::handle_rejection;
use serde_json;
use warp::{reject::Reject, Reply};

// Wrapper for ApiError in tests
#[derive(Debug)]
struct TestApiError(ApiError);

// Implement Reject for TestApiError
impl Reject for TestApiError {}

async fn retrieve_body_bytes(response: Response<Body>) -> Vec<u8> {
	return hyper::body::to_bytes(response.into_body())
		.await
		.unwrap()
		.to_vec();
}

#[tokio::test]
async fn test_handle_rejection_internal_server_error() {
	let rejection = warp::reject::custom(TestApiError(ApiError::new("Unexpected error", None)));
	let reply = handle_rejection(rejection).await.unwrap();

	let response = reply.into_response();

	// Assert status
	assert_eq!(
		response.status(),
		warp::http::StatusCode::INTERNAL_SERVER_ERROR
	);

	// Assert message
	let error: senjin::errors::ApiError =
		serde_json::from_slice(&retrieve_body_bytes(response).await).unwrap();
	assert_eq!(error.message, "Internal Server Error");
	assert_eq!(error.code, None);
}
