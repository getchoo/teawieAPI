import { Hono } from "hono";
import { logger } from "hono/logger";
import { prettyJSON } from "hono/pretty-json";
import { zValidator } from "@hono/zod-validator";
import { list } from "./schemas";
import { Bindings, Variables } from "./env";

const app = new Hono<{ Bindings: Bindings; Variables: Variables }>();

app.use("*", logger());
app.use("*", prettyJSON());

app.get("/", (c) => {
	return c.redirect(
		c.env.REDIRECT_ROOT ?? "https://github.com/getchoo/teawieAPI",
	);
});

app.get("/static/*", async (c) => {
	return await c.env.ASSETS.fetch(c.req.raw);
});

app.get("/list_teawies", zValidator("query", list), async (c) => {
	const { limit } = c.req.query();

	return c.json(
		WIES.slice(0, parseInt(limit ?? "5")).map((wie) => {
			return {
				url: new URL(`/${WIE_DIR}/${wie}`, c.req.url).toString(),
			};
		}),
	);
});

app.get("/random_teawie", (c) => {
	const wie = WIES[Math.floor(Math.random() * WIES.length)];

	return c.json({
		url: new URL(`/${WIE_DIR}/${wie}`, c.req.url).toString(),
	});
});

app.get("/get_random_teawie", (c) => c.redirect("/random_teawie"));

export default app;
