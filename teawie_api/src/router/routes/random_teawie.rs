use crate::{http::teawie_archive, RandomTeawie};

use super::AppState;

use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use rand::seq::SliceRandom;
use tracing::{debug, error, trace};

#[tracing::instrument(skip_all)]
#[debug_handler]
pub async fn handle(State(state): State<AppState>) -> impl IntoResponse {
	debug!("Getting a random teawie");

	trace!("Attempting to get teawie image URLs");
	let wies = match teawie_archive::image_urls(&state.http_client, state.cache).await {
		Ok(wies) => wies,
		Err(why) => {
			let msg = format!("Couldn't fetch teawies!\n{why:#?}");
			error!(msg);
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(RandomTeawie {
					error: Some(msg),
					..Default::default()
				}),
			);
		}
	};
	trace!("Received teawies!");

	trace!("Choosing a random wie");
	let mut rng = rand::thread_rng();
	let url = wies.choose(&mut rng).cloned();
	trace!("Found a random teawie!");

	(
		StatusCode::OK,
		Json(RandomTeawie {
			url,
			..Default::default()
		}),
	)
}
