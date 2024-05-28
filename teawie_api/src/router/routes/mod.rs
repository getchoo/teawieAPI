use crate::router::AppState;

use axum::{routing::get, Router};

mod list_teawies;
mod not_found;
mod random_teawie;
mod root;

#[tracing::instrument(skip_all)]
pub fn add(router: Router<AppState>) -> Router<AppState> {
	router
		.route("/", get(root::handle))
		.route("/list_teawies", get(list_teawies::handle))
		.route("/random_teawie", get(random_teawie::handle))
		.route("/get_random_teawie", get(random_teawie::handle))
		.fallback(not_found::handle)
}
