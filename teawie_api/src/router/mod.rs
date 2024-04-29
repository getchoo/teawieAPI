use crate::api::{HttpClient, HttpClientExt};

use std::sync::{Arc, RwLock};

use axum::Router;
use tracing::trace;

mod middleware;
mod state;

pub use state::*;

#[tracing::instrument]
pub fn router() -> Router {
	trace!("Creating HTTP client");
	let http_client = <HttpClient as HttpClientExt>::default();
	trace!("Creating cache");
	let cache = Arc::new(RwLock::new(Cache::new()));
	trace!("Setting up app state");
	let state = AppState { cache, http_client };

	trace!("Creating router");
	let router = Router::default();
	trace!("Applying middleware");
	let router = middleware::apply(router);
	trace!("Setting up routes");
	let router = crate::routes::add(router);

	router.with_state(state)
}
