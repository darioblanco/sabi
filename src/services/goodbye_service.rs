use actix_web::{post, web, HttpResponse, Responder};
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
async fn goodbye_world() -> impl Responder {
	let response = GoodbyeResponse {
		message: "Goodbye, World!".to_string(),
	};
	HttpResponse::Ok().json(response)
}

#[post("/goodbye/reason")]
async fn goodbye_reason(body: web::Json<GoodbyeRequest>) -> impl Responder {
	let reason = &body.reason;
	if reason.is_empty() {
		return HttpResponse::BadRequest().json("Reason cannot be empty");
	}
	let response = GoodbyeResponse {
		message: format!("Goodbye World! Reason: {}", reason),
	};
	HttpResponse::Ok().json(response)
}
