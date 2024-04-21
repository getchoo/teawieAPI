use crate::http;

use std::{
	sync::{Arc, RwLock},
	time::Instant,
};

use axum::Router;

/// Basic in-memory cache
#[derive(Clone, Debug, Default)]
pub struct Cache {
	teawie_download_urls: Option<(Instant, Vec<String>)>,
}

impl Cache {
	/// Recover Teawie download URLs from the cache if they exist
	pub fn teawie_download_urls(&self) -> Option<(Instant, Vec<String>)> {
		self.teawie_download_urls.clone()
	}

	/// Update the Teawie download URLs in the cache
	pub fn cache_teawie_download_urls(&mut self, urls: Vec<String>) {
		let now = Instant::now();
		self.teawie_download_urls = Some((now, urls));
	}
}

/// State for our router
#[derive(Clone, Debug)]
pub struct State {
	pub cache: Arc<RwLock<Cache>>,

	pub http_client: http::Client,
}

impl Default for State {
	fn default() -> Self {
		Self {
			cache: Arc::new(RwLock::new(Cache::default())),
			http_client: <http::Client as http::Ext>::default(),
		}
	}
}

/// Add our [`AppState`] to the [`Router`]
pub fn apply(router: Router<State>) -> Router {
	let state = State::default();
	router.with_state(state)
}
