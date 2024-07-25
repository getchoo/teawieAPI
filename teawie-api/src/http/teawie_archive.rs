use super::{github::API_URL as GITHUB_API, Client, GitHubExt};
use crate::state::Cache;

use std::{
	error::Error,
	fmt::Debug,
	sync::{Arc, RwLock},
	time::Duration,
};

use futures::future::try_join_all;
use log::{trace, warn};
use serde::Deserialize;

/// Bad version of GitHub's
/// `GET /repos/{owner}/{repo}/content/{path}`
#[derive(Clone, Debug, Deserialize)]
struct Content {
	name: String,
	#[serde(rename = "type")]
	r#type: String,
	download_url: Option<String>,
}

/// How long image URLs are cached for
const CACHE_TTL_SECS: u64 = 60 * 60; // 1 hour

// Teawie repository info
const REPO_OWNER: &str = "SympathyTea";
const REPO_NAME: &str = "Teawie-Archive";

// TODO @getchoo: maybe use GET /repos/{owner}/{repo}/git/trees/{tree_sha}?
const TEAWIE_SUBDIRS: [&str; 4] = [
	"teawie-media/Original Teawies",
	"teawie-media/Teawie Variants",
	"teawie-media/Teawie in Places",
	"teawie-media/Unfinished Teawies",
];

/// File extensions we consider to be images
// TODO @getchoo: maybe make this less naive?
const IMAGE_EXTENSIONS: [&str; 6] = ["gif", "jpg", "jpeg", "png", "svg", "webp"];

/// Get the [`Content`]s of [`path`] in the Teawie Archive
async fn fetch_contents<T>(
	http_github: &T,
	path: &str,
) -> Result<Vec<Content>, Box<dyn Error + Send + Sync>>
where
	T: super::Ext + GitHubExt + Debug,
{
	let url = format!("{GITHUB_API}/repos/{REPO_OWNER}/{REPO_NAME}/contents/{path}");
	let content_items: Vec<Content> = if let Ok(token) = http_github.token_from_env() {
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

/// Find images through the [`Content`]s of directories and map them to their download URL
fn find_image_urls<'a, T>(directories: T) -> impl Iterator<Item = String> + 'a
where
	T: Iterator<Item = &'a Content> + 'a + Debug,
{
	directories.filter_map(|file| {
		// Only find *files* with image extensions
		if file.r#type == "file"
			&& IMAGE_EXTENSIONS.contains(&file.name.split('.').last().unwrap_or(""))
		{
			file.download_url.clone()
		} else {
			None
		}
	})
}

/// Get all image URLs from the Teawie Archive
pub async fn image_urls(
	http: &Client,
	cache: Arc<RwLock<Cache>>,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
	{
		trace!("Checking for URLs in cache");
		let lock = cache.read().unwrap();
		if let Some((age, wies)) = lock.teawie_download_urls() {
			if age.elapsed() < Duration::from_secs(CACHE_TTL_SECS) {
				trace!("Found!");
				return Ok(wies);
			}
		}
	}
	warn!("Teawie image URL cache is out of date! Refreshing");

	// Fetch contents for all subdirectories in parallel
	let futures = try_join_all(TEAWIE_SUBDIRS.iter().map(|&dir| fetch_contents(http, dir))).await?;
	let directories = futures.iter().flatten();
	trace!("Fetched Teawie subdirectories!");

	// Find the `download_url`'s of images in each directory
	let images: Vec<String> = find_image_urls(directories).collect();
	trace!("Resolved image URLs");

	{
		trace!("Caching new URLs");
		let mut lock = cache.write().unwrap();
		lock.cache_teawie_download_urls(images.clone());
		trace!("Cached!");
	}

	Ok(images)
}
