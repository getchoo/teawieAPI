use crate::router::Cache;

use super::{GitHubClient, HttpClient, HttpClientExt};

use std::{
	error::Error,
	fmt::Debug,
	sync::{Arc, RwLock},
	time::Duration,
};

use futures::future::try_join_all;
use octocrab::models::repos::Content;
use tracing::{debug, trace, warn};

const REPO_OWNER: &str = "SympathyTea";
const REPO_NAME: &str = "Teawie-Archive";

// TODO @getchoo: use /repos/{owner}/{repo}/git/trees/{tree_sha}
// after https://github.com/XAMPPRocky/octocrab/issues/536
const TEAWIE_SUBDIRS: [&str; 4] = [
	"teawie-media/Original Teawies",
	"teawie-media/Teawie Variants",
	"teawie-media/Teawie in Places",
	"teawie-media/Unfinished Teawies",
];

const IMAGE_EXTENSIONS: [&str; 6] = ["gif", "jpg", "jpeg", "png", "svg", "webp"];

#[tracing::instrument]
async fn fetch_contents<T>(
	http_github: &T,
	path: &str,
) -> Result<Vec<Content>, Box<dyn Error + Send + Sync>>
where
	T: HttpClientExt + GitHubClient + Debug,
{
	let url = format!("https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/contents/{path}");
	let content_items: Vec<Content> = if let Some(token) = http_github.token().ok() {
		http_github
			.get_authenticated_request(&token, &url)
			.await?
			.json()
			.await?
	} else {
		warn!("No GitHub token found! Rate limits should be expected");
		http_github.get_request(&url).await?.json().await?
	};

	Ok(content_items)
}

#[tracing::instrument]
fn find_image_urls<'a, T>(contents: T) -> impl Iterator<Item = String> + 'a
where
	T: Iterator<Item = &'a Content> + 'a + Debug,
{
	contents.filter_map(|content| {
		content
			.name
			.split('.')
			.last()
			.and_then(|file_ext| {
				(content.r#type == "file" && IMAGE_EXTENSIONS.contains(&file_ext))
					.then(|| content.download_url.clone())
			})
			.flatten()
	})
}

#[tracing::instrument]
pub async fn image_urls(
	http: &HttpClient,
	cache: Arc<RwLock<Cache>>,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
	{
		trace!("Checking for URLs in cache");
		let lock = cache.read().unwrap();
		if let Some((age, wies)) = lock.teawie_download_urls() {
			if age.elapsed() < Duration::from_secs(60 * 60) {
				trace!("Found!");
				return Ok(wies);
			}
			debug!("Cache is out of date! Refreshing");
		}
	}

	let futures = try_join_all(TEAWIE_SUBDIRS.iter().map(|&dir| fetch_contents(http, dir))).await?;
	let directories = futures.iter().flatten();
	debug!("Fetched Teawie subdirectories!");

	let images: Vec<String> = find_image_urls(directories).collect();
	debug!("Resolved image URLs");

	{
		trace!("Caching new URLs");
		let mut lock = cache.write().unwrap();
		lock.cache_teawie_download_urls(images.clone());
	}

	Ok(images)
}
