import { z } from "zod";

export const list = z.object({
	limit: z
		.string()
		.optional()
		.default("5")
		.refine((data) => {
			const parsed = parseInt(data);
			return !isNaN(parsed);
		}),
});
