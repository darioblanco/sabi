use actix_web::{http, test, App};
use senjin::services::hello_service::{routes, HelloRequest, HelloResponse};

#[actix_rt::test]
async fn test_hello_world() {
	let mut app = test::init_service(App::new().configure(routes)).await;
	let req = test::TestRequest::get().uri("/hello").to_request();
	let resp = test::call_service(&mut app, req).await;
	assert_eq!(resp.status(), http::StatusCode::OK);
	let body = test::read_body(resp).await;
	assert_eq!(
		body,
		serde_json::to_string(&HelloResponse {
			message: "Hello, World!".to_string()
		})
		.unwrap()
	);
}

#[actix_rt::test]
async fn test_hello_with_params() {
	let mut app = test::init_service(App::new().configure(routes)).await;
	let req = test::TestRequest::post()
		.uri("/hello")
		.set_json(&HelloRequest {
			name: "John".into(),
		})
		.to_request();
	let resp = test::call_service(&mut app, req).await;
	assert_eq!(resp.status(), http::StatusCode::OK);
	let body = test::read_body(resp).await;
	assert_eq!(
		body,
		serde_json::to_string(&HelloResponse {
			message: "Hello, John!".to_string()
		})
		.unwrap()
	);
}

#[actix_rt::test]
async fn test_hello_with_params_empty_name() {
	let mut app = test::init_service(App::new().configure(routes)).await;
	let req = test::TestRequest::post()
		.uri("/hello")
		.set_json(&HelloRequest { name: "".into() })
		.to_request();
	let resp = test::call_service(&mut app, req).await;
	assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
	let body = test::read_body(resp).await;
	assert_eq!(body, "Name cannot be empty");
}
