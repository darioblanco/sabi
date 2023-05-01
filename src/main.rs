use axum::{response::IntoResponse, routing::get, Router};
use memory_store::MemoryStore;
use services::auth::{MultiOAuthConfig, MultiOAuthProvider, OAuthConfig, User};
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
	pub oauth_providers: Arc<MultiOAuthProvider>,
}

impl Clone for AppState {
	fn clone(&self) -> Self {
		Self {
			config: self.config.clone(),
			oauth_providers: self.oauth_providers.clone(),
			memory_store: self.memory_store.clone(),
		}
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
	let filter = EnvFilter::try_new(format!(
		"sabi={},hyper={}",
		config.log_level.to_string(), // Only log the configured level or above
		match config.log_level {
			Level::TRACE => "trace", // Only activate hyper logging when trace is given
			_ => "off",
		}
	))
	.unwrap();
	tracing_subscriber::registry()
		.with(filter)
		.with(tracing_subscriber::fmt::layer())
		.init();

	// Load MemoryStore
	let memory_store = Arc::new(memory_store::RedisStore::new(config.redis_url.to_string()).await);

	// Load Oauth providers
	let oauth_providers = Arc::new(MultiOAuthProvider::new(MultiOAuthConfig {
		discord: OAuthConfig {
			client_id: config.discord.client_id.to_string(),
			client_secret: config.discord.client_secret.to_string(),
			redirect_url: config.discord.redirect_url.to_string(),
		},
		google: OAuthConfig {
			client_id: "secret".to_string(),
			client_secret: "secret".to_string(),
			redirect_url: "https://localhost".to_string(),
		},
	}));

	// add routes and their global state
	let app_state = AppState {
		config,
		memory_store,
		oauth_providers,
	};
	let app = Router::new()
		.route("/", get(index))
		.route("/health", get(handlers::health))
		.route("/protected", get(protected))
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

// Session is optional
async fn index(user: Option<User>) -> impl IntoResponse {
	match user {
		Some(u) => format!(
			"Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
			u.discord.username
		),
		None => "You're not logged in.\nVisit `/auth/discord` to do so.".to_string(),
	}
}

// Valid user session required. If there is none, redirect to the auth page
async fn protected(user: User) -> impl IntoResponse {
	format!(
		"Welcome to the protected area :)\nHere's your info:\n{:?}",
		user
	)
}
