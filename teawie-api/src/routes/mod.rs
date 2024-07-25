use crate::state::State;

use axum::{routing::get, Router};

mod list_teawies;
mod not_found;
mod random_teawie;
mod root;

pub fn add(router: Router<State>) -> Router<State> {
	router
		.route("/", get(root::handle))
		.route("/list_teawies", get(list_teawies::handle))
		.route("/random_teawie", get(random_teawie::handle))
		.route("/get_random_teawie", get(random_teawie::handle))
		.fallback(not_found::handle)
}
