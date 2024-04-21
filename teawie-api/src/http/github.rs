use super::{Client, Response};

use std::{env::VarError, error::Error as StdError};

use axum::http::{HeaderMap, HeaderValue};
use log::trace;

pub const API_URL: &str = "https://api.github.com";

pub trait Ext {
	fn token_from_env(&self) -> Result<String, VarError>;
	/// Make a GET request using a token in the environment
	async fn get_authenticated_request(
		&self,
		token: &str,
		url: &str,
	) -> Result<Response, Box<dyn StdError + Send + Sync>>;
}

impl Ext for Client {
	fn token_from_env(&self) -> Result<String, VarError> {
		std::env::var("GITHUB_TOKEN")
	}

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
