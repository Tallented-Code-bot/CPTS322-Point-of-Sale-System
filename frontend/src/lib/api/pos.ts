import type { Product } from '$lib/stores/cart';

export type CheckoutItemPayload = {
	//A type that contains information for each item in a recipt
	upc: string;
	qty: number;
	unitPrice: number;
};

export type CheckoutPayload = {
	//A type that holds the data for a single recipt (excluding tax and all that)
	items: CheckoutItemPayload[];
	totalPrice: number;
};

export type CheckoutResponse = {
	//A type that holds a lot of misc data to put on recipts  
	receiptId: string;
	timestamp: string;
	itemCount: number;
	total: number;
};

const BASE_URL = import.meta.env.VITE_POS_API_BASE ?? '';
//VITE api? not sure what this does

async function handle<T>(res: Response): Promise<T> {
	// handler function test to see if "Response" is ok
	if (!res.ok) {
		const text = await res.text().catch(() => '');
		throw new Error(text || `Request failed (${res.status})`);
	}
	return (await res.json()) as T;
}


export async function fetchProductByUPC(upc: string): Promise<Product> {
	const res = await fetch(`${BASE_URL}/api/products/${encodeURIComponent(upc)}`, {
		method: 'GET',
		credentials: 'include',
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
