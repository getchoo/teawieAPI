use axum::{http::StatusCode, response::IntoResponse};
use tracing::debug;

#[tracing::instrument(skip_all)]
pub async fn handle() -> impl IntoResponse {
	debug!("Returning 404");
	(StatusCode::NOT_FOUND, "womp womp")
}
