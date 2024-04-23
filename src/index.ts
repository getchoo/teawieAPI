import { Hono } from "hono";
import { HTTPException } from "hono/http-exception";
import { logger } from "hono/logger";
import { prettyJSON } from "hono/pretty-json";
import { findWies } from "./teawie";
import type { Bindings, Variables } from "./env";

const app = new Hono<{ Bindings: Bindings; Variables: Variables }>();

app.use("*", logger());
app.use("*", prettyJSON());

app.get("/", (c) => {
	return c.redirect(
		c.env.REDIRECT_ROOT ?? "https://github.com/getchoo/teawieAPI",
	);
});

app.get("/list_teawies", async (c) => {
	const wiePaths = await findWies(c.env);

	return c.json({
		wies: wiePaths
	})
})

app.get("/random_teawie", async (c) => {
	const wiePaths = await findWies(c.env);
	const wie = wiePaths.at(Date.now() % wiePaths.length);
	if (!wie) {
		throw new HTTPException(500, { message: "Couldn't choose a wie!" })
	}

	return c.json({
		url: wie,
	});
});

app.get("/get_random_teawie", (c) => c.redirect("/random_teawie"));

export default app;
