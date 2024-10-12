import { z } from "@hono/zod-openapi";

const ErrorResponse = z.string().optional().openapi({
	description: "Error message reported by server",
});

const TeawieURLResponse = z.object({
	url: z.string().url().optional().openapi({
		description: "URL to Teawie",
	}),

	error: ErrorResponse,
});

export const ListTeawiesParams = z.object({
	limit: z
		.string()
		.optional()
		.default("5")
		.refine((data) => {
			const parsed = parseInt(data);
			return !isNaN(parsed);
		})
		.openapi({
			description: "Maximum number of Teawie URLs to be returned",
		}),
});

export const ListTeawiesResponse = z.object({
	urls: z.array(z.string().url()).optional(),
	error: ErrorResponse,
});

export const RandomTeawiesResponse = TeawieURLResponse;
