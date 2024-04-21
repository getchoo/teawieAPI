use super::AppState;

use std::time::Duration;

use axum::{http::Method, Router};
use tower_http::{
	cors::{Any, CorsLayer},
	timeout::TimeoutLayer,
	trace::TraceLayer,
};

#[tracing::instrument]
pub fn apply(router: Router<AppState>) -> Router<AppState> {
	let timeout = (
		TraceLayer::new_for_http(),
		TimeoutLayer::new(Duration::from_secs(10)),
	);
	let cors = CorsLayer::new()
		.allow_methods([Method::GET])
		.allow_origin(Any);

	router.layer(timeout).layer(cors)
}
