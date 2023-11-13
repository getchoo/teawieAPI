import { BuildOptions, build } from "esbuild";
import { existsSync } from "node:fs";
import { constants, copyFile, mkdir, readdir, rm } from "node:fs/promises";
import { basename, join } from "node:path";

const distDir = "dist";
const contentDir = "static";
const teawieArchiveDir = "Teawie-Archive/teawie-media";
const teawieContentDir = join(contentDir, "teawie");

const checkAndCreate = async (dir: string) => {
	if (!existsSync(dir)) {
		await mkdir(dir, { recursive: true });
	} else {
		for (const f of await readdir(dir, { withFileTypes: true })) {
			await rm(f.path, { recursive: true, force: true });
		}
	}
};

const findWies = async () => {
	// grab directories from archive
	const wieDirs = (await readdir(teawieArchiveDir, { withFileTypes: true }))
		.filter((file) => file.isDirectory())
		.map((file) => join(file.path, file.name));

	// map over directories until we (probably) only have the images we want and their path
	const collected = wieDirs.map(async (dir) =>
		(await readdir(dir, { withFileTypes: true }))
			.filter(async (file) => {
				const fileExt = file.name.split(",").pop() ?? "";
				return file.isFile() && !["ini", "txt"].includes(fileExt);
			})
			.map((file) => join(file.path, file.name)),
	);

	const res = await Promise.all(collected);

	return res.flat();
};

const wies = await findWies();

const define = {
	WIES: JSON.stringify(wies.map((wie) => basename(wie).replaceAll(" ", "%20"))),
	WIE_DIR: JSON.stringify(teawieContentDir),
};

await checkAndCreate(distDir);
await checkAndCreate(join(distDir, contentDir));
await checkAndCreate(join(distDir, teawieContentDir));

for (const f of wies) {
	await copyFile(
		f,
		join(distDir, teawieContentDir, basename(f)),
		constants.COPYFILE_FICLONE,
	);
}

const options = {
	entryPoints: ["src/index.ts"],
	outfile: join(distDir, "_worker.js"),
	define,
	format: "esm",
	bundle: true,
	minify: true,
} satisfies BuildOptions;

await build(options);
