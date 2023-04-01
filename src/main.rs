pub mod errors;
pub mod handlers;
pub mod models;

use handlers::handle_hello;
use handlers::handle_rejection;
use tracing::info;
use warp::Filter;

#[tokio::main]
pub async fn main() {
	// Set up tracing (logging) subscriber
	// ...

	// Define our endpoint
	let hello = warp::path!("hello" / String)
		.and(warp::get())
		.and_then(handle_hello)
		.with(warp::log("hello"));

	// Add error handling using the handle_rejection function
	let routes = hello.recover(handle_rejection);

	// Start the server
	let port = 3030;
	info!("Starting server on port {}", port);
	warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
