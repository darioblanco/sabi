use actix_web::{error, post, web, Error, Responder};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GoodbyeRequest {
	pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct GoodbyeResponse {
	pub message: String,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
	cfg.service(goodbye_world).service(goodbye_reason);
}

#[post("/goodbye")]
async fn goodbye_world() -> Result<impl Responder, Error> {
	let response = GoodbyeResponse {
		message: "Goodbye, World!".to_string(),
	};
	Ok(web::Json(response))
}

#[post("/goodbye/reason")]
async fn goodbye_reason(body: web::Json<GoodbyeRequest>) -> Result<impl Responder, Error> {
	let reason = &body.reason;
	if reason.is_empty() {
		return Err(error::ErrorBadRequest("Reason cannot be empty"));
	}
	let response = GoodbyeResponse {
		message: format!("Goodbye World! Reason: {}", reason),
	};
	Ok(web::Json(response))
}
