use axum::Router;
use hyper::{Body, Request, StatusCode};
use senjin::services::health_service::{routes, HealthResponse};
use tower::ServiceExt;

fn create_router() -> Router {
	return Router::new().nest("/health", routes());
}

#[tokio::test]
async fn test_health() {
	let app = create_router();

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
