import { redirect } from '@sveltejs/kit';
import { configureAuth } from '$lib/auth';
import * as Auth from '@aws-amplify/auth';

export const prerender = false; // if this layout depends on per-user data
export const ssr = false; // if this layout depends on per-user data

export async function load({ url }) {
	configureAuth();
	try {
		return { user: await Auth.getCurrentUser() };
	} catch {
		redirect(303, `/?redirectTo=${url.pathname}`);
	}
}
