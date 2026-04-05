import type { Product } from '$lib/stores/cart';

export type CheckoutItemPayload = {
	upc: string;
	qty: number;
	unitPrice: number;
};

export type CheckoutPayload = {
	items: CheckoutItemPayload[];
	totalPrice: number;
};

export type CheckoutResponse = {
	receiptId: string;
	timestamp: string;
	itemCount: number;
	total: number;
};

const BASE_URL = import.meta.env.VITE_POS_API_BASE ?? '';

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

export async function checkout(payload: CheckoutPayload): Promise<CheckoutResponse> {
	const res = await fetch(`${BASE_URL}/api/transactions`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		credentials: 'include',
		body: JSON.stringify(payload)
	});
	return handle(res);
}
