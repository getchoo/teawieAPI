use crate::http::Client;

use std::{
	sync::{Arc, RwLock},
	time::Instant,
};

#[derive(Clone, Debug, Default)]
pub struct Cache {
	teawie_download_urls: Option<(Instant, Vec<String>)>,
}

impl Cache {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn teawie_download_urls(&self) -> Option<(Instant, Vec<String>)> {
		self.teawie_download_urls.clone()
	}

	pub fn cache_teawie_download_urls(&mut self, urls: Vec<String>) {
		let now = Instant::now();
		self.teawie_download_urls = Some((now, urls));
	}
}

#[derive(Clone, Debug)]
pub struct AppState {
	pub cache: Arc<RwLock<Cache>>,
	pub http_client: Client,
}
