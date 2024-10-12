import app from "./lib/mod.ts";

Deno.serve(app.fetch);
