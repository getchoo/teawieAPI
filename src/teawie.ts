import { Octokit } from "@octokit/core";
import { Endpoints } from "@octokit/types";
import { restEndpointMethods } from "@octokit/plugin-rest-endpoint-methods";

const MyOctokit = Octokit.plugin(restEndpointMethods);
const octokit = new MyOctokit();

type repositoryPathContentsResponse =
	Endpoints["GET /repos/{owner}/{repo}/contents/{path}"]["response"];

// Teawie repository owner and name
const REPO_OWNER = "SympathyTea";
const REPO_NAME = "Teawie-Archive";

// Subdirectories of the above repository containing files we want
const SUBDIRS = [
	"teawie-media/Original Teawies",
	/*
	"teawie-media/Teawie Variants",
	"teawie-media/Teawie in Places",
	"teawie-media/Unfinished Teawies",
	*/
];

// File extensions we consider to be images
const IMAGE_EXTENSIONS = ["gif", "jpg", "jpeg", "png", "svg", "webp"];

const contentsOf = (
	path: string,
): Promise<repositoryPathContentsResponse["data"]> =>
	octokit.rest.repos
		.getContent({
			owner: REPO_OWNER,
			repo: REPO_NAME,
			path,
		})
		.then(({ data }) => data);

const imageUrlsIn = (
	files: repositoryPathContentsResponse["data"][],
): string[] =>
	files
		// Find *files* that are (probably) images and have a download URL
		.filter(
			(file) =>
				!Array.isArray(file) &&
				file.download_url &&
				file.type == "file" &&
				IMAGE_EXTENSIONS.includes(file.name.split(".").at(-1) || ""),
		)
		.map((file) => {
			if (Array.isArray(file)) {
				throw new Error(
					"We go an array when we weren't supported to. Womp womp",
				);
			}

			// Should this happen? No
			// Could it? I don't know
			// But let's be safe :steamhappy:
			if (!file.download_url) {
				throw new Error(`Could not find download URL for file "${file.name}"`);
			}

			return file.download_url;
		});

export const imageUrls = (): Promise<string[]> =>
	Promise.all(SUBDIRS.map(contentsOf)).then(imageUrlsIn);
