import { BuildOptions, build } from "esbuild";
import { existsSync } from "node:fs";
import { constants, copyFile, mkdir, readdir, rm } from "node:fs/promises";
import { join } from "node:path";

const distDir = "dist";
const contentDir = join(distDir, "static");
const teawieArchiveDir = "Teawie-Archive/teawie-media/Original Teawies";

const checkAndCreate = async (dir: string) => {
	if (!existsSync(dir)) {
		await mkdir(dir, { recursive: true });
	} else {
		for (const f of await readdir(dir)) {
			await rm(join(dir, f), { recursive: true, force: true });
		}
	}
};

await checkAndCreate(distDir);
await checkAndCreate(contentDir);
await checkAndCreate(join(contentDir, "teawie"));

const wies = (await readdir(teawieArchiveDir)).filter((wie) => {
	const fileExt = wie.split(".").pop();
	return !["ini", "txt"].includes(fileExt ?? "");
});

for (const f of wies) {
	await copyFile(join(teawieArchiveDir, f), join(contentDir, "teawie", f), constants.COPYFILE_FICLONE);
}

const define = {
	WIES: JSON.stringify(wies.map((wie) => wie.replace(" ", "%20"))),
	WIE_DIR: JSON.stringify("static/teawie"),
};

const options = {
	entryPoints: ["src/index.ts"],
	outfile: join(distDir, "_worker.js"),
	define,
	format: "esm",
	bundle: true,
	minify: true,
} satisfies BuildOptions;

await build(options);
