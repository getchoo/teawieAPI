use axum::{http::StatusCode, response::IntoResponse};
use log::debug;

/// Return a nice 404 to lost travelers
pub async fn handle() -> impl IntoResponse {
	debug!("Returning 404");
	(StatusCode::NOT_FOUND, "womp womp")
}
