use std::{env::VarError, error::Error};

use axum::http::{HeaderMap, HeaderValue};
use reqwest::Response;
use tracing::{instrument, trace};

pub mod teawie_archive;

pub type HttpClient = reqwest::Client;

pub trait HttpClientExt {
	async fn get_request(&self, url: &str) -> Result<Response, reqwest::Error>;
	fn default() -> Self;
}

pub trait GitHubClient {
	fn token(&self) -> Result<String, VarError>;
	async fn get_authenticated_request(
		&self,
		token: &str,
		url: &str,
	) -> Result<Response, Box<dyn Error + Send + Sync>>;
}

impl HttpClientExt for HttpClient {
	#[instrument]
	fn default() -> Self {
		reqwest::ClientBuilder::new()
			.user_agent(&format!(
				"teawie-api/{}",
				option_env!("CARGO_PKG_VERSION").unwrap_or("development")
			))
			.build()
			.unwrap()
	}

	#[instrument]
	async fn get_request(&self, url: &str) -> Result<Response, reqwest::Error> {
		trace!("Making request to {url}");
		let resp = self.get(url).send().await?;
		resp.error_for_status_ref()?;

		Ok(resp)
	}
}

impl GitHubClient for HttpClient {
	#[instrument]
	fn token(&self) -> Result<String, VarError> {
		std::env::var("GH_TOKEN")
	}

	async fn get_authenticated_request(
		&self,
		token: &str,
		url: &str,
	) -> Result<Response, Box<dyn Error + Send + Sync>> {
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
