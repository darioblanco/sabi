use axum::{extract::FromRef, routing::get, Router};
use memory_store::MemoryStore;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tower_http::trace::{self, TraceLayer};
use tracing::{debug, info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub mod config;
pub mod errors;
pub mod handlers;
pub mod memory_store;
pub mod middleware;
pub mod services;

pub struct AppState {
	pub config: Arc<config::Config>,
	pub memory_store: Arc<dyn MemoryStore>,
	pub oauth_client: BasicClient,
}

impl Clone for AppState {
	fn clone(&self) -> Self {
		Self {
			config: self.config.clone(),
			oauth_client: self.oauth_client.clone(),
			memory_store: self.memory_store.clone(),
		}
	}
}

impl FromRef<AppState> for BasicClient {
	fn from_ref(state: &AppState) -> Self {
		state.oauth_client.clone()
	}
}

#[tokio::main]
#[cfg(not(tarpaulin_include))]
pub async fn main() {
	// Load environment variables into the config
	let config = Arc::new(config::Config::from_env(&config::SystemEnvironment));
	let api_address: SocketAddr = config.api_address;
	debug!("Loaded environment variables {:?}", config);

	// Set up logging
	tracing_subscriber::registry()
		.with(EnvFilter::try_new(config.log_level.to_string()).unwrap()) // Only log the configured level or above
		.with(tracing_subscriber::fmt::layer())
		.init();

	// Load MemoryStore
	let memory_store = Arc::new(memory_store::RedisStore::new(config.redis_url.to_string()).await);

	// Load Oauth client
	let oauth_client = BasicClient::new(
		ClientId::new(config.discord.client_id.to_string()),
		Some(ClientSecret::new(config.discord.client_secret.to_string())),
		AuthUrl::new(config.discord.auth_url.to_string()).unwrap(),
		Some(TokenUrl::new(config.discord.token_url.to_string()).unwrap()),
	)
	.set_redirect_uri(RedirectUrl::new(config.discord.redirect_url.to_string()).unwrap());

	// add routes and their global state
	let app_state = AppState {
		config,
		memory_store,
		oauth_client,
	};
	let app = Router::new()
		.route("/health", get(handlers::health))
		.nest("/auth", services::auth::routes())
		.nest("/hello", services::hello::routes())
		.nest("/goodbye", services::goodbye::routes())
		.with_state(app_state);

	// add middlewares
	let app = app
		.layer(
			TraceLayer::new_for_http()
				.make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
				.on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
		)
		.layer(middleware::cors(api_address));

	// add a fallback service for handling routes to unknown paths
	let app = app.fallback(handlers::not_found);

	let addr = SocketAddr::from(api_address);
	info!("Starting server on {}", api_address);
	hyper::Server::bind(&addr)
		.serve(app.into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap();
}

#[cfg(not(tarpaulin_include))]
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
