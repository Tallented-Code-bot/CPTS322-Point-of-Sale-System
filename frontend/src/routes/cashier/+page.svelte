<script lang="ts">
	import {
		addToCart,
		cartItems,
		subtotal,
		tax,
		total,
		setQty,
		removeItem,
		clearCart,
		buildCheckoutPayload
	} from '$lib/stores/cart';
	import { fetchProductByUPC, checkout } from '$lib/api/pos';
	import { isLoading } from '$lib/stores/auth';

	let upc = '';
	let error = '';
	let paid = 0;
	let receiptMsg = '';
	let isSubmitting = false;
	let isScanning = false;

	function getErrMsg(e: unknown) {
		if (e instanceof Error) return e.message;
		if (typeof e === 'string') return e;
		try {
			return JSON.stringify(e);
		} catch {
			return 'Unknown error.';
		}
	}

	async function scanAdd() {
		error = '';
		receiptMsg = '';

		const trimmed = upc.trim();
		if (!trimmed) return;

		isScanning = true;

		try {
			// Debug: confirm the handler fired
			console.log('[cashier] scanAdd UPC:', trimmed);

			const product = await fetchProductByUPC(trimmed);

			// Guard: ensure product looks valid for your cart store
			if (!product || !product.upc || !product.name || typeof product.price !== 'number') {
				console.error('[cashier] Invalid product returned:', product);
				throw new Error('Product data from API is missing fields (upc/name/price).');
			}

			addToCart(product, 1);
			upc = '';
		} catch (e) {
			console.error('[cashier] scanAdd failed:', e);
			error = getErrMsg(e) || 'Could not find product.';
		} finally {
			isScanning = false;
		}
	}

	async function completeSale() {
		error = '';
		receiptMsg = '';
		isSubmitting = true;

		try {
			const payload = buildCheckoutPayload(paid);
			if (payload.items.length === 0) throw new Error('Cart is empty.');
			if (payload.change < 0) throw new Error('Not enough payment.');

			const res = await checkout(payload);
			receiptMsg = `Sale complete. Receipt: ${res.receiptId}`;
			clearCart();
			paid = 0;
		} catch (e) {
			console.error('[cashier] checkout failed:', e);
			error = getErrMsg(e) || 'Checkout failed.';
		} finally {
			isSubmitting = false;
		}
	}
</script>

{#if $isLoading}
	<p class="muted">Loading...</p>
{:else}
	<div class="page">
		<h1>Cashier Checkout</h1>

		<!-- Use a form so Enter ALWAYS submits -->
		<form class="scanRow" on:submit|preventDefault={scanAdd}>
			<input
				placeholder="Scan or type UPC and press Enter"
				bind:value={upc}
				autocomplete="off"
				autocapitalize="off"
				spellcheck="false"
			/>
			<button type="submit" disabled={isScanning}>
				{isScanning ? 'Adding...' : 'Add'}
			</button>
		</form>

		{#if error}
			<div class="error">{error}</div>
		{/if}
		{#if receiptMsg}
			<div class="success">{receiptMsg}</div>
		{/if}

		<div class="layout">
			<div class="cart">
				<h2>Cart</h2>

				{#if $cartItems.length === 0}
					<p class="muted">No items yet.</p>
				{:else}
					<table>
						<thead>
							<tr>
								<th>Item</th>
								<th>UPC</th>
								<th>Price</th>
								<th>Qty</th>
								<th>Line</th>
								<th></th>
							</tr>
						</thead>
						<tbody>
							{#each $cartItems as line (line.product.upc)}
								<tr>
									<td>{line.product.name}</td>
									<td class="mono">{line.product.upc}</td>
									<td>${line.product.price.toFixed(2)}</td>
									<td>
										<input
											type="number"
											min="0"
											value={line.qty}
											on:input={(e) =>
												setQty(
													line.product.upc,
													Number((e.target as HTMLInputElement).value)
												)}
										/>
									</td>
									<td>${(line.product.price * line.qty).toFixed(2)}</td>
									<td>
										<button class="danger" type="button" on:click={() => removeItem(line.product.upc)}>
											Remove
										</button>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>

					<button class="ghost" type="button" on:click={clearCart}>Clear Cart</button>
				{/if}
			</div>

			<div class="summary">
				<h2>Summary</h2>

				<div class="row">
					<span>Subtotal</span><strong>${$subtotal.toFixed(2)}</strong>
				</div>
				<div class="row">
					<span>Tax</span><strong>${$tax.toFixed(2)}</strong>
				</div>
				<div class="row total">
					<span>Total</span><strong>${$total.toFixed(2)}</strong>
				</div>

				<label class="pay">
					<span>Paid</span>
					<input type="number" min="0" step="0.01" bind:value={paid} />
				</label>

				<div class="row">
					<span>Change</span>
					<strong>${Math.max(0, paid - $total).toFixed(2)}</strong>
				</div>

				<button disabled={isSubmitting} on:click={completeSale}>
					{isSubmitting ? 'Processing...' : 'Complete Sale'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.page {
		max-width: 1100px;
		margin: 2rem auto;
		padding: 1rem;
	}
	.muted {
		color: #a0aec0;
	}
	.mono {
		font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono',
			'Courier New', monospace;
	}

	.scanRow {
		display: flex;
		gap: 0.75rem;
		margin: 1rem 0 1.25rem;
	}
	input {
		padding: 0.8rem;
		border-radius: 10px;
		border: 1px solid #2d3748;
		background: #111827;
		color: #f7fafc;
		width: 100%;
	}
	button {
		padding: 0.8rem 1.1rem;
		border-radius: 10px;
		border: none;
		cursor: pointer;
		font-weight: 600;
	}
	button:disabled {
		opacity: 0.65;
		cursor: not-allowed;
	}
	.ghost {
		background: transparent;
		border: 1px solid #2d3748;
		color: #f7fafc;
		margin-top: 0.75rem;
	}
	.danger {
		background: #e53e3e;
		color: #111;
	}

	.error {
		background: rgba(229, 62, 62, 0.15);
		border: 1px solid rgba(229, 62, 62, 0.35);
		padding: 0.75rem;
		border-radius: 10px;
		margin-bottom: 1rem;
	}
	.success {
		background: rgba(72, 187, 120, 0.15);
		border: 1px solid rgba(72, 187, 120, 0.35);
		padding: 0.75rem;
		border-radius: 10px;
		margin-bottom: 1rem;
	}

	.layout {
		display: grid;
		grid-template-columns: 2fr 1fr;
		gap: 1.25rem;
		align-items: start;
	}
	.cart,
	.summary {
		background: #0b1220;
		border: 1px solid #1f2937;
		border-radius: 16px;
		padding: 1rem;
	}
	table {
		width: 100%;
		border-collapse: collapse;
	}
	th,
	td {
		padding: 0.6rem;
		border-bottom: 1px solid #1f2937;
		text-align: left;
	}
	td input {
		width: 80px;
	}

	.row {
		display: flex;
		justify-content: space-between;
		margin: 0.5rem 0;
	}
	.total {
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid #1f2937;
	}
	.pay {
		display: grid;
		gap: 0.25rem;
		margin: 1rem 0;
	}
</style>