use axum::{http::StatusCode, response::IntoResponse};
use tracing::trace;

#[tracing::instrument]
pub async fn handle() -> impl IntoResponse {
	trace!("Returning 404");
	(StatusCode::NOT_FOUND, "womp womp")
}
