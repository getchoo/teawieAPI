import { env } from "@hono/hono/adapter";
import { logger } from "@hono/hono/logger";
import { prettyJSON } from "@hono/hono/pretty-json";
import { swaggerUI } from "@hono/swagger-ui";
import { createRoute, OpenAPIHono } from "@hono/zod-openapi";
import { VERSION } from "./consts.ts";
import {
	ListTeawiesParams,
	ListTeawiesResponse,
	RandomTeawiesResponse,
} from "./schemas.ts";
import { imageUrls } from "./teawie.ts";

const app = new OpenAPIHono();

app.use("*", logger());
app.use("*", prettyJSON());

app.get(
	"/",
	(c) => {
		const { REDIRECT_ROOT } = env<{ REDIRECT_ROOT: string | undefined }>(c);

		return c.redirect(
			REDIRECT_ROOT ?? "https://github.com/getchoo/teawieAPI",
		);
	},
);

app.get("/swagger", swaggerUI({ url: "/doc" }));

app.doc("/doc", {
	openapi: "3.0.0",
	info: {
		version: VERSION,
		title: "teawieAPI",
	},
});

app.openapi(
	createRoute({
		method: "get",
		path: "/list_teawies",
		request: {
			params: ListTeawiesParams,
		},
		responses: {
			200: {
				content: {
					"application/json": {
						schema: ListTeawiesResponse,
					},
				},
				description: "List known Teawie URLS",
			},
		},
	}),
	async (c) => {
		const { limit } = c.req.query();
		const kv = await Deno.openKv();
		const urls = await imageUrls(kv);

		return c.json(
			{
				urls: urls.splice(0, parseInt(limit ?? "5")),
			},
			200,
		);
	},
);

app.openapi(
	createRoute({
		method: "get",
		path: "/random_teawie",
		responses: {
			200: {
				content: {
					"application/json": {
						schema: RandomTeawiesResponse,
					},
				},
				description: "A random URL to a picture of Teawie",
			},
		},
	}),
	async (c) => {
		const kv = await Deno.openKv();

		return imageUrls(kv).then((urls) =>
			c.json({
				url: urls[Math.floor(Math.random() * urls.length)],
			})
		);
	},
);

app.onError((error, c) => {
	console.error(error);

	return c.json({ error: error.message }, 500);
});

export default app;
