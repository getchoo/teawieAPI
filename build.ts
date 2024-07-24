import { BuildOptions, build } from "esbuild";
import { join } from "path";

const options = {
	entryPoints: ["src/index.ts"],
	outfile: join("dist", "_worker.js"),
	format: "esm",
	bundle: true,
	minify: true,
} satisfies BuildOptions;

await build(options);
