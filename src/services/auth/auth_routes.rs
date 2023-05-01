use crate::AppState;

use super::{auth_dto::DiscordUser, DiscordAuthRequest, User, COOKIE_NAME};
use async_session::Session;
use axum::{
	extract::{Query, State},
	http::{header::SET_COOKIE, HeaderMap},
	response::{IntoResponse, Redirect},
	routing::get,
	Router,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope, TokenResponse};

pub fn routes() -> Router<AppState> {
	// /auth
	Router::new()
		.route("/discord", get(discord))
		.route("/discord/authorized", get(login_authorized))
}

async fn discord(State(app_state): State<AppState>) -> impl IntoResponse {
	let (auth_url, _csrf_token) = app_state
		.oauth_client
		.authorize_url(CsrfToken::new_random)
		.add_scope(Scope::new("identify".to_string()))
		.url();

	// Redirect to Discord's oauth service
	Redirect::to(auth_url.as_ref())
}

async fn login_authorized(
	Query(query): Query<DiscordAuthRequest>,
	State(app_state): State<AppState>,
) -> impl IntoResponse {
	let memory_store = app_state.memory_store;
	let oauth_client = app_state.oauth_client;
	// Get an auth token
	let token = oauth_client
		.exchange_code(AuthorizationCode::new(query.code.clone()))
		.request_async(async_http_client)
		.await
		.unwrap();

	// Fetch user data from discord
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

	// Create a new session filled with user data
	let mut session = Session::new();
	session.insert("user", &user).unwrap();

	// Store session and get corresponding cookie
	let cookie = memory_store.store_session(session).await.unwrap().unwrap();

	// Build the cookie
	let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

	// Set cookie
	let mut headers = HeaderMap::new();
	headers.insert(SET_COOKIE, cookie.parse().unwrap());

	(headers, Redirect::to("/"))
}
