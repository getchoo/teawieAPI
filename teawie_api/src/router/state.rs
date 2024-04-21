use crate::api::HttpClient;

#[derive(Clone, Debug)]
pub struct AppState {
	pub http_client: HttpClient,
}
