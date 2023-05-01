use async_session::async_trait;
use axum::{
	extract::{rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts, TypedHeader},
	response::{IntoResponse, Redirect, Response},
	RequestPartsExt,
};
use http::{header, request::Parts};
use serde_derive::{Deserialize, Serialize};
use tracing::debug;

use crate::AppState;

pub static COOKIE_NAME: &str = "SESSION";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	pub discord: DiscordUser,
}

// The user data we'll get back from Discord.
// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
	pub id: String,
	pub avatar: Option<String>,
	pub username: String,
	pub discriminator: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DiscordAuthRequest {
	pub code: String,
	pub state: String,
}

pub struct DiscordAuthRedirect;

impl IntoResponse for DiscordAuthRedirect {
	fn into_response(self) -> Response {
		Redirect::temporary("/auth/discord").into_response()
	}
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
	AppState: FromRef<S>,
	S: Send + Sync,
{
	// If anything goes wrong or no session is found, redirect to the auth page
	type Rejection = DiscordAuthRedirect;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		debug!("Analyzing request in User middleware {:?}", parts);
		let memory_store = <AppState>::from_ref(state).memory_store;

		let cookies = parts
			.extract::<TypedHeader<headers::Cookie>>()
			.await
			.map_err(|e| match *e.name() {
				header::COOKIE => match e.reason() {
					TypedHeaderRejectionReason::Missing => DiscordAuthRedirect,
					_ => panic!("unexpected error getting Cookie header(s): {}", e),
				},
				_ => panic!("unexpected error getting cookies: {}", e),
			})?;
		debug!("Loaded cookies {:?}", cookies);
		let session_cookie = cookies.get(COOKIE_NAME).ok_or(DiscordAuthRedirect)?;

		debug!("Loaded session cookie {:?}", session_cookie);
		let session = memory_store
			.load_session(session_cookie.to_string())
			.await
			.unwrap()
			.ok_or(DiscordAuthRedirect)?;

		debug!("Loaded session {:?}", session);
		let user = session.get::<User>("user").ok_or(DiscordAuthRedirect)?;

		Ok(user)
	}
}
