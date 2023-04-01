use actix_web::{middleware::Logger, App, HttpServer};
use log::{debug, info};

pub mod config;
pub mod errors;
pub mod services;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
	// Load configuration from environment variables
	let config = config::Config::from_env(&config::SystemEnvironment);
	env_logger::builder().filter_level(config.log_level).init();

	debug!("Loaded environment variables {:?}", config);

	info!("Starting server on {}", config.api_address);
	HttpServer::new(move || {
		App::new()
			.wrap(Logger::default())
			.configure(services::hello_service::routes)
			.configure(services::goodbye_service::routes)
			.app_data(config.clone())
			.wrap(actix_web::middleware::Logger::default())
	})
	.bind(config.api_address)?
	.run()
	.await
}
