use std::sync::Arc;

use axum::Router;
use hyper::{Body, Request, StatusCode};
use senjin::{
	config,
	services::hello_service::{routes, HelloResponse},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_hello_world() {
	let config = Arc::new(config::Config::from_params("test".to_string()));
	let app = Router::new().nest("/hello", routes(config));

	let request = Request::builder()
		.method("GET")
		.uri("/hello")
		.body(Body::empty())
		.unwrap();
	let response = app.oneshot(request).await.unwrap();
	assert_eq!(response.status(), StatusCode::OK);
	let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
	assert_eq!(
		body,
		serde_json::to_string(&HelloResponse {
			message: "Hello, World!".to_string(),
			version: "test".to_string(),
		})
		.unwrap()
	);
}

#[tokio::test]
async fn test_hello_with_params() {
	let config = Arc::new(config::Config::from_params("test".to_string()));
	let app = Router::new().nest("/hello", routes(config));

	let request = Request::builder()
		.method("POST")
		.uri("/hello")
		.header("content-type", "application/json")
		.body(Body::from(
			json!({
				"name": "John"
			})
			.to_string(),
		))
		.unwrap();
	let response = app.oneshot(request).await.unwrap();
	assert_eq!(response.status(), StatusCode::OK);
	let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
	assert_eq!(
		body,
		serde_json::to_string(&HelloResponse {
			message: "Hello, John!".to_string(),
			version: "test".to_string(),
		})
		.unwrap()
	);
}

#[tokio::test]
async fn test_hello_with_params_empty_name() {
	let config = Arc::new(config::Config::from_params("test".to_string()));
	let app = Router::new().nest("/hello", routes(config));

	let request = Request::builder()
		.method("POST")
		.uri("/hello")
		.header("content-type", "application/json")
		.body(Body::from(
			json!({
				"name": ""
			})
			.to_string(),
		))
		.unwrap();
	let response = app.oneshot(request).await.unwrap();
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
	assert_eq!(body, "{\"error\":\"invalid request\"}");
}
