import { writable, derived, get } from 'svelte/store';

export type Product = {
	_id?: string; //_id? means that it doesn't need an id (?), but if it does have one, it doesn't need it right away (_)
	upc: string;
	name: string;
	price: number;
	quantity?: number;
};

export type CartItem = {
	product: Product;
	qty: number;
};

const cart = writable<CartItem[]>([]); //creates cart: a (currently empty) array of CartItem[]'s

function round2(n: number) {
	//simple function that rounds to 2 decimal places.
	return Math.round(n * 100) / 100;
}

export const cartItems = { subscribe: cart.subscribe };

export const subtotal = derived(cart, ($cart) =>
	round2($cart.reduce((sum, line) => sum + line.product.price * line.qty, 0))
);

export const taxRate = writable(0.0825);
export const tax = derived([subtotal, taxRate], ([$subtotal, $taxRate]) =>
	round2($subtotal * $taxRate)
);
export const total = derived([subtotal, tax], ([$subtotal, $tax]) => round2($subtotal + $tax));

export function clearCart() {
	cart.set([]);
}

export function addToCart(product: Product, qty = 1) {
	cart.update((items) => {
		const idx = items.findIndex((x) => x.product.upc === product.upc);
		if (idx !== -1) {
			const copy = [...items];
			copy[idx] = { ...copy[idx], qty: copy[idx].qty + qty };
			return copy;
		}
		return [...items, { product, qty }];
		//"spread" syntax. ...items is able to return every item in items
		//items is a cart item.
		//If I understnad it correct, to add an item to our cart array, we
		//have to delete it and copy it again. not sure why.
		//This is all in the .update method which I believe is from Svelte.
	});
}

export function setQty(upc: string, qty: number) {
	cart.update((items) =>
		items.map((x) => (x.product.upc === upc ? { ...x, qty } : x)).filter((x) => x.qty > 0)
	);
}

export function removeItem(upc: string) {
	cart.update((items) => items.filter((x) => x.product.upc !== upc));
}

export function buildCheckoutPayload(paid: number) {
	const items = get(cart);
	const tot = get(total);
	const change = round2(paid - tot);

	return {
		items: items.map((x) => ({
			upc: String(x.product.upc),
			qty: x.qty,
			unitPrice: x.product.price
		})),
		totalPrice: tot,
		paid: round2(paid),
		change
	};
}
