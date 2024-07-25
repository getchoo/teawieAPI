use crate::{http::teawie_archive, state::State, ListTeawie};

use axum::{extract, http::StatusCode, response::IntoResponse, Json};
use log::{debug, error, trace};

/// Return all known Teawie download URLs
pub async fn handle(extract::State(state): extract::State<State>) -> impl IntoResponse {
	debug!("Attempting to get teawie image URLs");
	let wies = match teawie_archive::image_urls(&state.http_client, state.cache).await {
		Ok(wies) => wies,
		Err(why) => {
			let msg = format!("Couldn't fetch teawies!\n{why:#?}");
			error!("{msg}");

			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(ListTeawie {
					error: Some(msg),
					..Default::default()
				}),
			);
		}
	};
	trace!("Received teawies!");

	(
		StatusCode::OK,
		Json(ListTeawie {
			wies: Some(wies),
			..Default::default()
		}),
	)
}
