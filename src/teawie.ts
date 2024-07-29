import { USER_AGENT } from "./consts";
import { Endpoints } from "@octokit/types";

type repositoryPathContentsResponse =
	Endpoints["GET /repos/{owner}/{repo}/contents/{path}"]["response"];

const GITHUB_API = "https://api.github.com";

// Teawie repository owner and name
const REPO_OWNER = "SympathyTea";
const REPO_NAME = "Teawie-Archive";

// Subdirectories of the above repository containing files we want
const SUBDIRS = [
	"teawie-media/Original Teawies",
	"teawie-media/Teawie Variants",
	"teawie-media/Teawie in Places",
	"teawie-media/Unfinished Teawies",
];

// File extensions we consider to be images
const IMAGE_EXTENSIONS = ["gif", "jpg", "jpeg", "png", "svg", "webp"];

const contentsOf = (
	path: string,
): Promise<repositoryPathContentsResponse["data"]> =>
	fetch(`${GITHUB_API}/repos/${REPO_OWNER}/${REPO_NAME}/contents/${path}`, {
		headers: {
			accept: "application/vnd.github+json",
			"user-agent": USER_AGENT,
		},
	})
		.then((response) => {
			if (!response.ok) {
				throw new Error(
					`HTTP Error ${response.status}: ${response.statusText}`,
				);
			}

			return response.json();
		})
		.then((json) => {
			return json as repositoryPathContentsResponse["data"];
		});

const imageUrlsIn = (
	files: repositoryPathContentsResponse["data"],
): string[] => {
	// NOTE: This is done because the response may only contain data
	// for a single file's path
	const filesArray = Array.isArray(files) ? files : [files];

	return (
		filesArray
			// Find *files* that are (probably) images and have a download URL
			.filter(
				(file) =>
					!Array.isArray(file) &&
					file.download_url &&
					file.type == "file" &&
					IMAGE_EXTENSIONS.includes(file.name.split(".").at(-1) ?? ""),
			)
			.map((file) => {
				// Should this happen? No
				// Could it? I don't know
				// But let's be safe :steamhappy:
				if (!file.download_url) {
					throw new Error(
						`Could not find download URL for file "${file.name}"`,
					);
				}

				return file.download_url;
			})
	);
};

export const imageUrls = async (kv: KVNamespace): Promise<string[]> => {
	const cached = await kv.get("urls");
	if (cached) {
		console.trace("Found Teawie URLs in cache!");
		return JSON.parse(cached);
	}

	console.warn("Couldn't find Teawie URLs in cache! Fetching fresh ones");
	const fresh = await Promise.all(SUBDIRS.map(contentsOf)).then((responses) => {
		// See the note above
		const flatResponses = responses.flatMap((response) =>
			Array.isArray(response) ? response : [response],
		);

		return imageUrlsIn(flatResponses);
	});

	await kv.put("urls", JSON.stringify(fresh), { expirationTtl: 60 * 60 });

	return fresh;
};
