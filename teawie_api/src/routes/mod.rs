use crate::router::AppState;

use axum::{routing::get, Router};

mod not_found;
mod random_teawie;
mod root;

#[tracing::instrument]
pub fn add(router: Router<AppState>) -> Router<AppState> {
	router
		.route("/", get(root::handle))
		.route("/random_teawie", get(random_teawie::handle))
}
