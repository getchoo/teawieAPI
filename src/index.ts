import { logger } from "hono/logger";
import { prettyJSON } from "hono/pretty-json";
import { swaggerUI } from "@hono/swagger-ui";
import { OpenAPIHono, createRoute } from "@hono/zod-openapi";
import { VERSION } from "./consts";
import { Bindings, Variables } from "./env";
import {
	ListTeawiesParams,
	ListTeawiesResponse,
	RandomTeawiesResponse,
} from "./schemas";
import { imageUrls } from "./teawie";

const app = new OpenAPIHono<{ Bindings: Bindings; Variables: Variables }>();

app.use("*", logger());
app.use("*", prettyJSON());

app.get("/", (c) =>
	c.redirect(c.env.REDIRECT_ROOT ?? "https://github.com/getchoo/teawieAPI"),
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
		const urls = await imageUrls(c.env.TEAWIE_API);

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
	async (c) =>
		imageUrls(c.env.TEAWIE_API).then((urls) =>
			c.json({
				url: urls[Math.floor(Math.random() * urls.length)],
			}),
		),
);

app.get("/get_random_teawie", (c) => c.redirect("/random_teawie"));

app.onError((error, c) => {
	console.error(error);

	return c.json({ error: error.message }, 500);
});

export default app;
