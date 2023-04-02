use actix_http::Request;
use actix_web::{dev::ServiceResponse, http, test, web, App};
use senjin::{
	config,
	services::hello_service::{routes, HelloRequest, HelloResponse},
};

async fn serve_request(req: Request) -> ServiceResponse {
	let config = web::Data::new(config::Config::from_params("test".to_string()));
	let mut app = test::init_service(App::new().configure(routes).app_data(config.clone())).await;
	return test::call_service(&mut app, req).await;
}

#[actix_rt::test]
async fn test_hello_world() {
	let req = test::TestRequest::get().uri("/hello").to_request();
	let resp = serve_request(req).await;
	assert_eq!(resp.status(), http::StatusCode::OK);
	let body = test::read_body(resp).await;
	assert_eq!(
		body,
		serde_json::to_string(&HelloResponse {
			message: "Hello, World!".to_string(),
			version: "test".to_string(),
		})
		.unwrap()
	);
}

#[actix_rt::test]
async fn test_hello_with_params() {
	let req = test::TestRequest::post()
		.uri("/hello")
		.set_json(&HelloRequest {
			name: "John".into(),
		})
		.to_request();
	let resp = serve_request(req).await;
	assert_eq!(resp.status(), http::StatusCode::OK);
	let body = test::read_body(resp).await;
	assert_eq!(
		body,
		serde_json::to_string(&HelloResponse {
			message: "Hello, John!".to_string(),
			version: "test".to_string(),
		})
		.unwrap()
	);
}

#[actix_rt::test]
async fn test_hello_with_params_empty_name() {
	let req = test::TestRequest::post()
		.uri("/hello")
		.set_json(&HelloRequest { name: "".into() })
		.to_request();
	let resp = serve_request(req).await;
	assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
	let body = test::read_body(resp).await;
	assert_eq!(body, "Name cannot be empty");
}
