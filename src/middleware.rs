use std::net::SocketAddr;

use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub fn cors(api_address: SocketAddr) -> CorsLayer {
	CorsLayer::new()
		.allow_origin(
			format!("http://{}", api_address)
				.parse::<HeaderValue>()
				.unwrap(),
		)
		.allow_methods(vec![Method::GET, Method::POST])
}
