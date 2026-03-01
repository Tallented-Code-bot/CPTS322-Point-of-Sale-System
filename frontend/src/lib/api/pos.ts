import type { Product } from '$lib/stores/cart';

const BASE_URL = 'http://127.0.0.1:8000';

async function handle<T>(res: Response): Promise<T> {
	if (!res.ok) {
		const text = await res.text().catch(() => '');
		throw new Error(text || `Request failed (${res.status})`);
	}
	return (await res.json()) as T;
}

export async function fetchProductByUPC(upc: string): Promise<Product> {
	const res = await fetch(`${BASE_URL}/api/products/${encodeURIComponent(upc)}`, {
		method: 'GET',
		credentials: 'include'
	});
	return handle<Product>(res);
}

export async function checkout(payload: unknown): Promise<{ receiptId: string }> {
	const res = await fetch(`${BASE_URL}/api/checkout`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify(payload)
	});
	return handle(res);
}
