use axum::response::{IntoResponse, Redirect, Response};
use serde_derive::{Deserialize, Serialize};

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
