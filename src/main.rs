use actix_web::{middleware::Logger, web, App, HttpServer};
use log::{debug, info};

pub mod config;
pub mod errors;
pub mod services;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
	// Load configuration from environment variables
	let config = web::Data::new(config::Config::from_env(&config::SystemEnvironment));
	env_logger::builder().filter_level(config.log_level).init();

	debug!("Loaded environment variables {:?}", config);

	let api_address = config.get_ref().api_address;
	info!("Starting server on {}", api_address);
	HttpServer::new(move || {
		App::new()
			.wrap(Logger::default())
			.configure(services::hello_service::routes)
			.configure(services::goodbye_service::routes)
			.app_data(config.clone())
	})
	.bind(api_address)?
	.run()
	.await
}
