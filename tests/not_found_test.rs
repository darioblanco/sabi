use axum::Router;
use hyper::{Body, Request, StatusCode};
use sabi::handlers::not_found;
use tower::ServiceExt;

#[tokio::test]
async fn test_not_found() {
	let app = Router::new().fallback(not_found);

	let request = Request::builder()
		.method("GET")
		.uri("/notfound")
		.body(Body::empty())
		.unwrap();
	let response = app.oneshot(request).await.unwrap();
	assert_eq!(response.status(), StatusCode::NOT_FOUND);
	let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
	assert_eq!(body, "nothing to see here");
}
