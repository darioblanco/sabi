use actix_http::Request;
use actix_web::{dev::ServiceResponse, http, test, web, App};
use senjin::{
	config,
	services::goodbye_service::{routes, GoodbyeRequest, GoodbyeResponse},
};

async fn serve_request(req: Request) -> ServiceResponse {
	let config = web::Data::new(config::Config::from_params("test".to_string()));
	let mut app = test::init_service(App::new().configure(routes).app_data(config.clone())).await;
	return test::call_service(&mut app, req).await;
}

#[actix_rt::test]
async fn test_goodbye_world() {
	let req = test::TestRequest::post().uri("/goodbye").to_request();
	let resp = serve_request(req).await;
	assert_eq!(resp.status(), http::StatusCode::OK);
	let body = test::read_body(resp).await;
	assert_eq!(
		body,
		serde_json::to_string(&GoodbyeResponse {
			message: "Goodbye, World!".to_string()
		})
		.unwrap()
	);
}

#[actix_rt::test]
async fn test_goodbye_reason() {
	let req = test::TestRequest::post()
		.uri("/goodbye/reason")
		.set_json(&GoodbyeRequest {
			reason: "test".into(),
		})
		.to_request();
	let resp = serve_request(req).await;
	assert_eq!(resp.status(), http::StatusCode::OK);
	let body = test::read_body(resp).await;
	assert_eq!(
		body,
		serde_json::to_string(&GoodbyeResponse {
			message: "Goodbye World! Reason: test".to_string()
		})
		.unwrap()
	);
}

#[actix_rt::test]
async fn test_goodbye_reason_empty() {
	let req = test::TestRequest::post()
		.uri("/goodbye/reason")
		.set_json(&GoodbyeRequest { reason: "".into() })
		.to_request();
	let resp = serve_request(req).await;
	assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
	let body = test::read_body(resp).await;
	assert_eq!(body, "Reason cannot be empty");
}
