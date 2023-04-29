use axum::{http::Request, response::IntoResponse, Router};
use hyper::StatusCode;
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

pub mod config;
pub mod errors;
pub mod middlewares;
pub mod services;

#[tokio::main]
pub async fn main() {
	let config = Arc::new(config::Config::from_env(&config::SystemEnvironment));
	let api_address: SocketAddr = config.api_address;

	tracing_subscriber::fmt()
		.with_max_level(config.log_level)
		.init();

	debug!("Loaded environment variables {:?}", config);

	// add routes
	let app = Router::new()
		.nest("/health", services::health_service::routes())
		.nest("/hello", services::hello_service::routes(config))
		.nest("/goodbye", services::goodbye_service::routes());

	// add middlewares
	let app = app
		.layer(TraceLayer::new_for_http()
			.make_span_with(|request: &Request<_>| {
				tracing::info_span!("request", method = %request.method(), uri = %request.uri())
			})
		).layer(middlewares::cors_middleware(api_address));

	// add a fallback service for handling routes to unknown paths
	let app = app.fallback(handler_404);

	let addr = SocketAddr::from(api_address);
	info!("Starting server on {}", api_address);
	hyper::Server::bind(&addr)
		.serve(app.into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap();
}

async fn handler_404() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "nothing to see here")
}

async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

	info!("signal received, starting graceful shutdown");
}
