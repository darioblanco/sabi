pub mod config;
pub mod errors;
pub mod handlers;
pub mod models;

use config::Config;
use handlers::handle_hello;
use handlers::handle_rejection;
use tracing::{debug, info};
use warp::Filter;

use crate::config::SystemEnvironment;

#[tokio::main]
pub async fn main() {
	// Load configuration from environment variables
	let config = Config::from_env(&SystemEnvironment);

	// Initialize the logger
	tracing_subscriber::fmt()
		.with_max_level(config.log_level)
		.init();

	debug!("Loaded environment variables {:?}", config);

	// Routes
	let hello = warp::path!("hello" / String)
		.and(warp::get())
		.and_then(handle_hello)
		.with(warp::log("hello"));
	let routes = hello.recover(handle_rejection);

	// Start the server
	info!("Starting server on {}", config.api_address);
	warp::serve(routes).run(config.api_address).await;
}
