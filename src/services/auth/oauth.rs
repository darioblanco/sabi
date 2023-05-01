use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub enum ProviderType {
	Google,
	Discord,
}

pub trait OAuthProvider {
	fn client(&self) -> BasicClient;
}

#[derive(Clone, Debug)]
pub struct GoogleOAuthProvider {
	client: BasicClient,
}

impl GoogleOAuthProvider {
	pub fn new(config: OAuthConfig) -> Self {
		let google_client_id = ClientId::new(config.client_id.to_string());
		let google_client_secret = ClientSecret::new(config.client_secret.to_string());
		let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
			.expect("Invalid authorization endpoint URL");
		let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
			.expect("Invalid token endpoint URL");

		let client = BasicClient::new(
			google_client_id,
			Some(google_client_secret),
			auth_url,
			Some(token_url),
		)
		.set_redirect_uri(
			RedirectUrl::new(config.redirect_url.to_string()).expect("Invalid redirect URL"),
		);

		GoogleOAuthProvider { client }
	}
}

impl OAuthProvider for GoogleOAuthProvider {
	fn client(&self) -> BasicClient {
		self.client.clone()
	}
}

#[derive(Clone, Debug)]
pub struct DiscordOAuthProvider {
	client: BasicClient,
}

impl DiscordOAuthProvider {
	pub fn new(config: OAuthConfig) -> Self {
		let discord_client_id = ClientId::new(config.client_id.to_string());
		let discord_client_secret = ClientSecret::new(config.client_secret.to_string());
		let auth_url = AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())
			.expect("Invalid authorization endpoint URL");
		let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".to_string())
			.expect("Invalid token endpoint URL");

		let client = BasicClient::new(
			discord_client_id,
			Some(discord_client_secret),
			auth_url,
			Some(token_url),
		)
		.set_redirect_uri(
			RedirectUrl::new(config.redirect_url.to_string()).expect("Invalid redirect URL"),
		);

		DiscordOAuthProvider { client }
	}
}

impl OAuthProvider for DiscordOAuthProvider {
	fn client(&self) -> BasicClient {
		self.client.clone()
	}
}

#[derive(Clone, Debug)]
pub struct MultiOAuthProvider {
	google: GoogleOAuthProvider,
	discord: DiscordOAuthProvider,
}

pub struct MultiOAuthConfig {
	pub google: OAuthConfig,
	pub discord: OAuthConfig,
}

pub struct OAuthConfig {
	pub client_id: String,
	pub client_secret: String,
	pub redirect_url: String,
}

impl MultiOAuthProvider {
	pub fn new(config: MultiOAuthConfig) -> Self {
		MultiOAuthProvider {
			google: GoogleOAuthProvider::new(config.google),
			discord: DiscordOAuthProvider::new(config.discord),
		}
	}

	pub fn client(&self, provider_type: ProviderType) -> BasicClient {
		match provider_type {
			ProviderType::Google => self.google.client(),
			ProviderType::Discord => self.discord.client(),
		}
	}
}
