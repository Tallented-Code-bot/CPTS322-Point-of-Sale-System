import { writable, derived, get, type Readable } from 'svelte/store';
import { createAuth0Client, type Auth0Client, type User } from '@auth0/auth0-spa-js';
import { browser } from '$app/environment';

export const auth0Client = writable<Auth0Client | null>(null);
export const user = writable<User | null>(null);
export const isAuthenticated = writable<boolean>(false);
export const isLoading = writable<boolean>(true);
export const error = writable<string | null>(null);

// Derived stores
export const isLoggedIn: Readable<boolean> = derived(
	[isAuthenticated, isLoading],
	([$isAuthenticated, $isLoading]) => $isAuthenticated && !$isLoading
);

export async function initializeAuth() {
	if (!browser) return;
	try {
		isAuthenticated.set(false);
		user.set(null);
		error.set(null);
		isLoading.set(true);

		const client = await createAuth0Client({
			domain: import.meta.env.VITE_AUTH0_DOMAIN,
			clientId: import.meta.env.VITE_AUTH0_CLIENT_ID,
			authorizationParams: {
				redirect_uri: window.location.origin,
				audience: import.meta.env.VITE_AUTH0_AUDIENCE
			},
			useRefreshTokens: true,
			cacheLocation: 'memory'
		});
		auth0Client.set(client);

		// Handle callback
		if (window.location.search.includes('code=')) {
			await client.handleRedirectCallback();
			window.history.replaceState({}, document.title, window.location.pathname);
		}
		// Check authentication status
		const authenticated = await client.isAuthenticated();
		isAuthenticated.set(authenticated);

		if (authenticated) {
			const userData = await client.getUser();
			user.set(userData || null);
		}

		error.set(null);
	} catch (err) {
		console.error('Auth initialization error:', err);
		error.set(err instanceof Error ? err.message : 'Authentication initialization failed');
	} finally {
		isLoading.set(false);
	}
}

export async function login() {
	const client = get(auth0Client);
	if (client) {
		await client.loginWithRedirect({
			authorizationParams: {
				audience: import.meta.env.VITE_AUTH0_AUDIENCE
			}
		});
	}
}

export async function logout() {
	const client = get(auth0Client);
	if (client) {
		isAuthenticated.set(false);
		user.set(null);
		client.logout({
			logoutParams: {
				returnTo: window.location.origin
			}
		});
	}
}

export async function getToken(): Promise<string | null> {
	const client = get(auth0Client);
	if (!client) return null;

	try {
		return await client.getTokenSilently({
			authorizationParams: {
				audience: import.meta.env.VITE_AUTH0_AUDIENCE
			}
		});
	} catch (err: unknown) {
		if (err && typeof err === 'object' && 'error' in err && err.error === 'login_required') {
			await login();
		}
		return null;
	}
}
