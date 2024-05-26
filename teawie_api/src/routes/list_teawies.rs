use crate::{api::teawie_archive, ListTeawie};

use super::AppState;

use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::{debug, error};

#[tracing::instrument(skip_all)]
#[debug_handler]
pub async fn handle(State(state): State<AppState>) -> impl IntoResponse {
	debug!("Attempting to get teawie image URLs");
	let wies = match teawie_archive::image_urls(&state.http_client, state.cache).await {
		Ok(wies) => wies,
		Err(why) => {
			let msg = format!("Couldn't fetch teawies!\n{why:#?}");
			error!(msg);
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(ListTeawie {
					error: Some(msg),
					..Default::default()
				}),
			);
		}
	};
	debug!("Received teawies!");

	(
		StatusCode::OK,
		Json(ListTeawie {
			wies: Some(wies),
			..Default::default()
		}),
	)
}
