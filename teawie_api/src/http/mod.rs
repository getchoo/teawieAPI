use std::{env::VarError, error::Error as StdError};

use axum::http::{HeaderMap, HeaderValue};
use tracing::{instrument, trace};

pub mod teawie_archive;

pub type Client = reqwest::Client;
pub type Response = reqwest::Response;
pub type Error = reqwest::Error;

pub trait HttpClientExt {
	async fn get_request(&self, url: &str) -> Result<Response, Error>;
	fn default() -> Self;
}

pub trait GitHubClient {
	fn token_from_env(&self) -> Result<String, VarError>;
	async fn get_authenticated_request(
		&self,
		token: &str,
		url: &str,
	) -> Result<Response, Box<dyn StdError + Send + Sync>>;
}

impl HttpClientExt for Client {
	fn default() -> Self {
		reqwest::ClientBuilder::new()
			.user_agent(format!(
				"teawie-api/{}",
				option_env!("CARGO_PKG_VERSION").unwrap_or("development")
			))
			.build()
			.unwrap()
	}

	#[instrument(skip(self))]
	async fn get_request(&self, url: &str) -> Result<Response, Error> {
		trace!("Making request to {url}");
		let resp = self.get(url).send().await?;
		resp.error_for_status_ref()?;

		Ok(resp)
	}
}

impl GitHubClient for Client {
	fn token_from_env(&self) -> Result<String, VarError> {
		std::env::var("GITHUB_TOKEN")
	}

	#[instrument(skip(self, token))]
	async fn get_authenticated_request(
		&self,
		token: &str,
		url: &str,
	) -> Result<Response, Box<dyn StdError + Send + Sync>> {
		trace!("Making authenticated request to {url}");
		let mut headers = HeaderMap::new();
		headers.insert(
			"Authorization",
			HeaderValue::from_str(&format!("Bearer {token}"))?,
		);

		let resp = self.get(url).headers(headers).send().await?;
		resp.error_for_status_ref()?;

		Ok(resp)
	}
}
