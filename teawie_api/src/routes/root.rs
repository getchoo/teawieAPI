use std::sync::OnceLock;

use axum::response::{IntoResponse, Redirect};
use tracing::trace;

#[tracing::instrument(skip_all)]
pub async fn handle() -> impl IntoResponse {
	static URL: OnceLock<&str> = OnceLock::new();
	let url = URL.get_or_init(|| {
		option_env!("REDIRECT_URL").unwrap_or("https://github.com/getchoo/teawieAPI")
	});

	trace!("Redirecting to {url}");
	Redirect::to(url)
}
