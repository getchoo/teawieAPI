use crate::api::teawie_archive;

use super::AppState;

use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use getrandom::getrandom;
use serde::{Deserialize, Serialize};
use tracing::{error, info, trace};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct RandomTeawie {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
}

#[tracing::instrument]
#[debug_handler]
pub async fn handle(State(state): State<AppState>) -> impl IntoResponse {
	trace!("Attempting to get teawie image URLs");
	let wies = match teawie_archive::image_urls(&state.http_client).await {
		Ok(wies) => wies,
		Err(why) => {
			error!("Couldn't get teawie image urls! {why:#?}");
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(RandomTeawie {
					error: Some("Couldn't fetch wies from GitHub!".to_string()),
					..Default::default()
				}),
			);
		}
	};
	info!("Received teawies!");

	trace!("Choosing a random wie");
	let mut index = [0u8; 1];
	if getrandom(&mut index).is_err() {
		return (
			StatusCode::INTERNAL_SERVER_ERROR,
			Json(RandomTeawie {
				error: Some("Couldn't choose a random teawie!".to_string()),
				..Default::default()
			}),
		);
	}

	let url = wies.get(index[0] as usize % wies.len()).cloned();
	info!("Finished choosing a random teawie");
	(
		StatusCode::OK,
		Json(RandomTeawie {
			url,
			..Default::default()
		}),
	)
}
