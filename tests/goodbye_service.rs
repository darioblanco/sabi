use actix_web::{http, test, App};
use senjin::services::goodbye_service::{routes, GoodbyeRequest, GoodbyeResponse};

#[actix_rt::test]
async fn test_goodbye_world() {
	let mut app = test::init_service(App::new().configure(routes)).await;
	let req = test::TestRequest::post().uri("/goodbye").to_request();
	let resp = test::call_service(&mut app, req).await;
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
	let mut app = test::init_service(App::new().configure(routes)).await;
	let req = test::TestRequest::post()
		.uri("/goodbye/reason")
		.set_json(&GoodbyeRequest {
			reason: "test".into(),
		})
		.to_request();
	let resp = test::call_service(&mut app, req).await;
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
	let mut app = test::init_service(App::new().configure(routes)).await;
	let req = test::TestRequest::post()
		.uri("/goodbye/reason")
		.set_json(&GoodbyeRequest { reason: "".into() })
		.to_request();
	let resp = test::call_service(&mut app, req).await;
	assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
	let body = test::read_body(resp).await;
	assert_eq!(body, "\"Reason cannot be empty\"");
}
