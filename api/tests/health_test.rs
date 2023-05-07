use axum::{routing::get, Router};
use hyper::{Body, Request, StatusCode};
use sabi_api::handlers::{health, HealthResponse};
use tower::ServiceExt;

#[tokio::test]
async fn test_health() {
	let app = Router::new().route("/health", get(health));

	let request = Request::builder()
		.method("GET")
		.uri("/health")
		.body(Body::empty())
		.unwrap();
	let response = app.oneshot(request).await.unwrap();
	assert_eq!(response.status(), StatusCode::OK);
	let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
	assert_eq!(
		body,
		serde_json::to_string(&HealthResponse {
			status: "OK".to_string(),
		})
		.unwrap()
	);
}
