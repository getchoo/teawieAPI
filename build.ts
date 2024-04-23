import { type BuildOptions, build } from "esbuild";
import { join } from "node:path";

const distDir = "dist";

const options = {
	entryPoints: ["src/index.ts"],
	outfile: join(distDir, "_worker.js"),
	format: "esm",
	bundle: true,
	minify: true,
} satisfies BuildOptions;

await build(options);
