use log::trace;

pub mod github;
pub mod teawie_archive;
pub use github::Ext as GitHubExt;

pub type Client = reqwest::Client;
pub type Response = reqwest::Response;
pub type Error = reqwest::Error;

/// Primary extension for HTTP client
pub trait Ext {
	/// Make a GET request
	async fn get_request(&self, url: &str) -> Result<Response, Error>;
	/// Pre-configured HTTP clientk
	fn default() -> Self;
}

impl Ext for Client {
	fn default() -> Self {
		reqwest::ClientBuilder::new()
			.user_agent(format!(
				"teawie-api/{}",
				option_env!("CARGO_PKG_VERSION").unwrap_or("development")
			))
			.build()
			.unwrap()
	}

	async fn get_request(&self, url: &str) -> Result<Response, Error> {
		trace!("Making request to {url}");
		let resp = self.get(url).send().await?;
		resp.error_for_status_ref()?;

		Ok(resp)
	}
}
