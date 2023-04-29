use std::sync::Arc;

use axum::Router;
use hyper::{Body, Request, StatusCode};
use sabi::{
	config::Config,
	services::goodbye::{routes, GoodbyeResponse},
	AppState,
};
use serde_json::json;
use tower::ServiceExt;

fn create_router() -> Router {
	let config = Arc::new(Config::from_params("test".to_string()));
	return Router::new()
		.nest("/goodbye", routes())
		.with_state(AppState {
			config,
			oauth_client: None,
		});
}

#[tokio::test]
async fn test_goodbye_world() {
	let app = create_router();

	let request = Request::builder()
		.method("POST")
		.uri("/goodbye")
		.body(Body::empty())
		.unwrap();
	let response = app.oneshot(request).await.unwrap();
	assert_eq!(response.status(), StatusCode::OK);
	let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
	assert_eq!(
		body,
		serde_json::to_string(&GoodbyeResponse {
			message: "Goodbye, World!".to_string()
		})
		.unwrap()
	);
}

#[tokio::test]
async fn test_goodbye_reason() {
	let app = create_router();

	let request = Request::builder()
		.method("POST")
		.uri("/goodbye/reason")
		.header("content-type", "application/json")
		.body(Body::from(
			json!({
				"reason": "test"
			})
			.to_string(),
		))
		.unwrap();
	let response = app.oneshot(request).await.unwrap();
	assert_eq!(response.status(), StatusCode::OK);
	let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
	assert_eq!(
		body,
		serde_json::to_string(&GoodbyeResponse {
			message: "Goodbye World! Reason: test".to_string()
		})
		.unwrap()
	);
}

#[tokio::test]
async fn test_goodbye_reason_empty() {
	let app = create_router();

	let request = Request::builder()
		.method("POST")
		.uri("/goodbye/reason")
		.header("content-type", "application/json")
		.body(Body::from(
			json!({
				"reason": ""
			})
			.to_string(),
		))
		.unwrap();
	let response = app.oneshot(request).await.unwrap();
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
	assert_eq!(body, "{\"error\":\"invalid request\"}");
}
