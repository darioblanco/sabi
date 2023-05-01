use crate::AppState;

use super::{auth_dto::DiscordUser, DiscordAuthRequest, User, COOKIE_NAME};
use async_session::Session;
use axum::{
	extract::{Query, State},
	http::{header::SET_COOKIE, HeaderMap},
	response::{IntoResponse, Redirect},
	routing::get,
	Router, TypedHeader,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope, TokenResponse};
use tracing::debug;

pub fn routes() -> Router<AppState> {
	// /auth
	Router::new()
		.route("/discord", get(discord))
		.route("/discord/authorized", get(login_authorized))
		.route("/logout", get(logout))
}

// To be called when requesting a login to Discord
async fn discord(State(app_state): State<AppState>) -> impl IntoResponse {
	let discord_oauth_client = app_state
		.oauth_providers
		.client(crate::services::auth::ProviderType::Discord);
	let (auth_url, _csrf_token) = discord_oauth_client
		.authorize_url(CsrfToken::new_random)
		.add_scope(Scope::new("identify".to_string()))
		.url();

	// Redirect to Discord's oauth service
	debug!("Redirecting to Discord's oauth service: {}", auth_url);
	Redirect::to(auth_url.as_ref())
}

// To be called as a callback when the user has successfully logged externally into Discord
async fn login_authorized(
	Query(query): Query<DiscordAuthRequest>,
	State(app_state): State<AppState>,
) -> impl IntoResponse {
	let memory_store = app_state.memory_store;
	let discord_oauth_client = app_state
		.oauth_providers
		.client(crate::services::auth::ProviderType::Discord);

	debug!("Get auth token from oauth client with the given exchange code");
	let token = discord_oauth_client
		.exchange_code(AuthorizationCode::new(query.code.clone()))
		.request_async(async_http_client)
		.await
		.unwrap();

	// Fetch user data from discord
	debug!("Fetch user data from discord");
	let client = reqwest::Client::new();
	let discord_user: DiscordUser = client
		// https://discord.com/developers/docs/resources/user#get-current-user
		.get("https://discordapp.com/api/users/@me")
		.bearer_auth(token.access_token().secret())
		.send()
		.await
		.unwrap()
		.json::<DiscordUser>()
		.await
		.unwrap();
	let user = User {
		discord: discord_user,
	};

	debug!("Create a new session filled with user data");
	let mut session = Session::new();
	session.insert("user", &user).unwrap();

	debug!("Store session and get corresponding cookie");
	let cookie = memory_store.store_session(session).await.unwrap().unwrap();

	debug!("Set the cookie and redirect");
	let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);
	let mut headers = HeaderMap::new();
	headers.insert(SET_COOKIE, cookie.parse().unwrap());
	(headers, Redirect::to("/"))
}

async fn logout(
	State(app_state): State<AppState>,
	TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
	let memory_store = app_state.memory_store;
	let cookie = match cookies.get(COOKIE_NAME) {
		Some(cookie) => cookie,
		// No cookie set, just redirect
		None => return Redirect::to("/"),
	};
	let session = match memory_store.load_session(cookie.to_string()).await.unwrap() {
		Some(s) => s,
		// No session active, just redirect
		None => return Redirect::to("/"),
	};

	// Session was active, destroy it and redirect
	memory_store.destroy_session(session).await.unwrap();
	Redirect::to("/")
}
