import { Hono } from "hono";
import { logger } from "hono/logger";
import { HTTPException } from "hono/http-exception";
import { prettyJSON } from "hono/pretty-json";
import { zValidator } from "@hono/zod-validator";
import { Bindings, Variables } from "./env";
import { list } from "./schemas";
import { imageUrls } from "./teawie";

const app = new Hono<{ Bindings: Bindings; Variables: Variables }>();

app.use("*", logger());
app.use("*", prettyJSON());

app.get("/", (c) =>
	c.redirect(c.env.REDIRECT_ROOT ?? "https://github.com/getchoo/teawieAPI"),
);

app.get("/list_teawies", zValidator("query", list), async (c) => {
	const { limit } = c.req.query();

	return imageUrls()
		.then((urls) =>
			c.json(
				urls.slice(0, parseInt(limit ?? "5")).map((url) => {
					url;
				}),
			),
		)
		.catch((error) => console.log(error));
});

app.get("/random_teawie", (c) =>
	imageUrls().then((urls) =>
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
