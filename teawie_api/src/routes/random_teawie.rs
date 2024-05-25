use crate::api::teawie_archive;

use super::AppState;

use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tracing::{error, info, trace};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct RandomTeawie {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[tracing::instrument(skip_all)]
#[debug_handler]
pub async fn handle(State(state): State<AppState>) -> impl IntoResponse {
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
	info!("Received teawies!");

	trace!("Choosing a random wie");
	let mut rng = rand::thread_rng();
	let url = wies.choose(&mut rng).cloned();
	info!("Finished choosing a random teawie");

	(
		StatusCode::OK,
		Json(RandomTeawie {
			url,
			..Default::default()
		}),
	)
}
