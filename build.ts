import { BuildOptions, build } from "esbuild";
import { existsSync } from "node:fs";
import { constants, copyFile, mkdir, readdir, rm } from "node:fs/promises";
import { join } from "node:path";

const distDir = "dist";
const teawieArchiveDir = "Teawie-Archive/teawie-media/Original Teawies";

const checkAndCreate = async (dir: string) => {
	if (!existsSync(dir)) {
		await mkdir(dir, { recursive: true });
	}
};

await checkAndCreate(distDir);
for (const f of await readdir(distDir)) {
	await rm(join(distDir, f), { recursive: true, force: true });
}

await checkAndCreate(join(distDir, "static/teawie"));

const wies = (await readdir(teawieArchiveDir)).filter((wie) => {
	const fileExt = wie.split(".").pop();
	return !["ini", "txt"].includes(fileExt ?? "");
});

for (const f of wies) {
	await copyFile(join(teawieArchiveDir, f), join(distDir, "static/teawie", f), constants.COPYFILE_FICLONE);
}

const define = {
	WIES: JSON.stringify(wies),
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
