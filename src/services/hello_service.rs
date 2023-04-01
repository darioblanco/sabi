use actix_web::{error, get, post, web, Error, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRequest {
	pub name: String,
}

#[derive(Debug, Serialize)]
pub struct HelloResponse {
	pub message: String,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
	cfg.service(hello_world).service(hello_with_params);
}

#[get("/hello")]
async fn hello_world() -> impl Responder {
	let response = HelloResponse {
		message: "Hello, World!".to_string(),
	};
	HttpResponse::Ok().json(response)
}

#[post("/hello")]
async fn hello_with_params(request_body: web::Json<HelloRequest>) -> Result<HttpResponse, Error> {
	let name = &request_body.name;
	if name.is_empty() {
		return Err(error::ErrorBadRequest("Name cannot be empty"));
	}
	let response = HelloResponse {
		message: format!("Hello, {}!", name),
	};
	Ok(HttpResponse::Ok().json(response))
}
