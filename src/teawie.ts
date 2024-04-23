import { version } from "../package.json";
import type { Bindings } from "./env";
import { Octokit } from "@octokit/rest";

const repoOwner = "SympathyTea";
const repoName = "Teawie-Archive";
const repoSha = "main";

const imageTypes = ['.jpg', '.jpeg', '.png', '.gif', '.webp'];

const octokit = (env: Bindings) => new Octokit({
	auth: env.GITHUB_TOKEN,
	userAgent: `teawie-api/${version}`
});

export const getFileExtension = (path: string) => new URL(path, "https://example.com").pathname.split('.').at(-1)

export const findWies = async (env: Bindings) => {
	const client = octokit(env);

	return await client.git.getTree({
		owner: repoOwner,
		repo: repoName,
		tree_sha: repoSha,
		recursive: "true",
	}).then(({ data }) =>
		data.tree.filter((file) => {
			const filePath = file.path ?? "";
			const fileExt = getFileExtension(filePath) ?? "";
			file.type?.toString() === "blob" && imageTypes.includes(fileExt)
		}
		).map((file) => file.path ?? ""));
}

export const getWie = async (env: Bindings, path: string) => {
	const client = octokit(env);

	return await client.repos.getContent({
		owner: repoOwner,
		repo: repoName,
		ref: repoSha,
		path: path
	}).then((resp) => {
		console.log(resp);
		return resp.data.download_url
	});
}
