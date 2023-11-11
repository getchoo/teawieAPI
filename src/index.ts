import { Hono } from "hono";
import { logger } from "hono/logger";
import { prettyJSON } from "hono/pretty-json";
import { secureHeaders } from "hono/secure-headers";
import { zValidator } from "@hono/zod-validator";
import { list } from "./schemas";
import { Bindings, Variables } from "./env";

const app = new Hono<{ Bindings: Bindings; Variables: Variables }>();

app.use("*", secureHeaders());
app.use("*", logger());
app.use("*", prettyJSON());

app.get("/", (c) => {
	return c.redirect(c.env.REDIRECT_ROOT ?? "https://github.com/getchoo/teawieAPI");
});

app.get("/static/teawie/*", async (c) => {
	return await c.env.ASSETS.fetch(c.req.raw);
});

app.get("/list_teawies", zValidator("query", list), async (c) => {
	const { limit } = c.req.query();

	return c.json(
		WIES.slice(0, parseInt(limit ?? "5")).map((wie) => {
			return {
				url: `${c.env.URL ?? "https://api.mydadleft.me"}/static/teawie/${wie}`,
			};
		}),
	);
});

app.get("/random_teawie", async (c) => {
	const wies = WIES;
	const wie = wies[Math.floor(Math.random() * wies.length)];

	return c.json({
		url: `${c.env.URL ?? "https://api.mydadleft.me"}/static/${wie}`,
	});
});

export default app;
