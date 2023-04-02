use actix_web::{error, get, post, web, Error, Responder};
use serde_derive::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize)]
pub struct HelloRequest {
	pub name: String,
}

#[derive(Debug, Serialize)]
pub struct HelloResponse {
	pub message: String,
	pub version: String,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
	cfg.service(hello_world).service(hello_with_params);
}

#[get("/hello")]
async fn hello_world(config: web::Data<Config>) -> Result<impl Responder, Error> {
	let response = HelloResponse {
		message: "Hello, World!".to_string(),
		version: config.version.to_string(),
	};
	Ok(web::Json(response))
}

#[post("/hello")]
async fn hello_with_params(
	request_body: web::Json<HelloRequest>,
	config: web::Data<Config>,
) -> Result<impl Responder, Error> {
	let name = &request_body.name;
	if name.is_empty() {
		return Err(error::ErrorBadRequest("Name cannot be empty"));
	}
	let response = HelloResponse {
		message: format!("Hello, {}!", name),
		version: config.version.to_string(),
	};
	Ok(web::Json(response))
}
