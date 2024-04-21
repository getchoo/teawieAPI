mod http;
pub mod model;
mod routes;
mod state;

pub use model::*;
use state::State;

use std::time::Duration;

use axum::{http::Method, Router};
use tower_http::{
	cors::{Any, CorsLayer},
	timeout::TimeoutLayer,
	trace::TraceLayer,
};

/// Apply our middleware to the [`Router`]
fn apply_middleware(router: Router<State>) -> Router<State> {
	let timeout = (
		TraceLayer::new_for_http(),
		TimeoutLayer::new(Duration::from_secs(10)),
	);

	let cors = CorsLayer::new()
		.allow_methods([Method::GET])
		.allow_origin(Any);

	router.layer(timeout).layer(cors)
}

/// Wire up our middleware, routes, and state to the [`Router`]
pub fn router() -> Router {
	let router = Router::default();
	let router = apply_middleware(router);
	let router = routes::add(router);
	state::apply(router)
}
