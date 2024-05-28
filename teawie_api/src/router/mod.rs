use crate::http::{Client, HttpClientExt};

use std::sync::{Arc, RwLock};

use axum::Router;
use tracing::trace;

mod app_state;
mod middleware;
mod routes;

pub use app_state::*;

#[tracing::instrument]
pub fn router() -> Router {
	trace!("Creating HTTP client");
	let http_client = <Client as HttpClientExt>::default();
	trace!("Creating cache");
	let cache = Arc::new(RwLock::new(Cache::new()));
	trace!("Setting up app state");
	let state = AppState { cache, http_client };

	trace!("Creating router");
	let router = Router::default();
	trace!("Applying middleware");
	let router = middleware::apply(router);
	trace!("Setting up routes");
	let router = routes::add(router);

	router.with_state(state)
}
