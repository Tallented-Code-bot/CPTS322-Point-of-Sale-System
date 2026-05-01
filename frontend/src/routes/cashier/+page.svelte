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
	import type { CheckoutResponse, CheckoutItemPayload, CheckoutPayload } from '$lib/api/pos';
	import BackButton from '$lib/components/BackButton.svelte';

	let upc = '';
	let error = '';
	let paid = 0;
	let receiptMsg = '';
	let isSubmitting = false;
	let isScanning = false;
	let showClearConfirm = false;

	function getErrMsg(e: unknown): string {
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
			const product = await fetchProductByUPC(trimmed);
			if (!product || !product.upc || !product.name || typeof product.price !== 'number') {
				throw new Error('Product data is missing required fields.');
			}
			addToCart(product, 1); //I think this ", 1)" might be unnessessary,
			//since 1 is hard coded in the addToCart fct anyway.
			upc = '';
		} catch (e) {
			error = getErrMsg(e) || 'Could not find product.';
		} finally {
			isScanning = false;
		}
	}

	function handleQtyChange(upc: string, value: string) {
		const num = Number(value);
		if (!isNaN(num) && num >= 0) setQty(upc, num);
	}

	function confirmClear() {
		showClearConfirm = true;
	}

	function doClear() {
		clearCart();
		showClearConfirm = false;
	}

	async function formatReceipt(payload: CheckoutPayload) {
		//ITEM NAMES SHOULD NOT BE LONGER THAN 32 CHARS
		let receipt = '';
		for (const item of payload.items) {
			var curItem = await fetchProductByUPC(item.upc.trim());
			receipt +=
				curItem.name +
				' '.repeat(32 - curItem.name.length) +
				item.qty.toString() +
				' $' +
				curItem.price.toFixed(2) +
				'\n';
		}
		var blob = new Blob([receipt], { type: 'text/plain' });
		var link = document.createElement('a');
		link.href = URL.createObjectURL(blob);
		link.download = 'receipt.txt';
		link.click();
	}

	async function completeSale() {
		error = '';
		receiptMsg = '';
		isSubmitting = true;
		try {
			const payload = buildCheckoutPayload(paid);
			if (payload.items.length === 0) throw new Error('Cart is empty.');
			if (paid < $total) throw new Error('Payment amount is insufficient.');
			const res = await checkout(payload);
			receiptMsg = `Sale complete — Receipt #${res.receiptId}`;
			var temp = await fetchProductByUPC(payload.items[0].upc.trim());
			console.log(temp);
			formatReceipt(payload);
			clearCart();
			paid = 0;
		} catch (e) {
			error = getErrMsg(e) || 'Checkout failed.';
		} finally {
			isSubmitting = false;
		}
	}

	$: change = Math.max(0, paid - $total);
	$: shortfall = paid > 0 && paid < $total ? $total - paid : 0;
</script>

<div class="pos">
	<header class="topbar">
		<div class="topbar-left">
			<div class="back-wrap">
				<BackButton />
			</div>
		</div>
		<div class="topbar-center">
			<!-- <span class="terminal-label">CASHIER TERMINAL</span> -->
		</div>
		<div class="topbar-right">
			<a class="btn btn-ghost" href="/admin">ADMIN</a>
			<!-- <span class="badge-operator">
				<span class="badge-dot"></span>
				Cashier
			</span> -->
		</div>
	</header>

	<main class="main">
		<section class="panel cart-panel">
			<div class="panel-title">
				<!-- <span>CART</span> -->
				{#if $cartItems.length > 0}
					<span class="item-count"
						>{$cartItems.length} item{$cartItems.length !== 1 ? 's' : ''}</span
					>
				{/if}
			</div>

			<div class="scan-row">
				<!-- <div class="scan-input-wrap">
					<span class="scan-icon">⊡</span>
					<input
						class="scan-input"
						placeholder="Scan or enter UPC…"
						bind:value={upc}
						autocomplete="off"
						autocapitalize="off"
						spellcheck="false"
						on:keydown={(e) => e.key === 'Enter' && scanAdd()}
					/>
				</div> -->
				<!-- <button class="btn btn-add" on:click={scanAdd} disabled={isScanning}>
					{isScanning ? '…' : 'ADD'}
				</button> -->
			</div>

			{#if error}
				<div class="alert alert-error" role="alert">
					<span class="alert-icon">✕</span>
					{error}
				</div>
			{/if}

			{#if receiptMsg}
				<div class="alert alert-success" role="status">
					<div class="alert-copy">
						<span class="alert-icon">✓</span>
						<span>{receiptMsg}</span>
					</div>
					<a class="alert-link" href="/home">Go Home</a>
				</div>
			{/if}

			<div class="table-wrap">
				{#if $cartItems.length === 0}
					<div class="empty-state">
						<div class="empty-icon">▭</div>
						<p>Cart is empty. Scan a barcode to begin.</p>
					</div>
				{:else}
					<table>
						<thead>
							<tr>
								<th>Item</th>
								<th>UPC</th>
								<th class="num">Unit</th>
								<th class="num">Qty</th>
								<th class="num">Line</th>
								<th></th>
							</tr>
						</thead>
						<tbody>
							{#each $cartItems as line (line.product.upc)}
								<tr class="cart-row">
									<td class="name-cell">{line.product.name}</td>
									<td class="mono upc-cell">{line.product.upc}</td>
									<td class="num">${line.product.price.toFixed(2)}</td>
									<td class="num qty-cell">
										<input class="qty-input" type="number" min="0" value={line.qty} disabled />
									</td>
									<td class="num line-total">
										${(line.product.price * line.qty).toFixed(2)}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				{/if}
			</div>
		</section>

		<aside class="panel summary-panel">
			<div class="panel-title">SUMMARY</div>

			<div class="summary-rows">
				<div class="sum-row">
					<span>Subtotal</span>
					<span>${$subtotal.toFixed(2)}</span>
				</div>
				<div class="sum-row">
					<span>Tax</span>
					<span>${$tax.toFixed(2)}</span>
				</div>
				<div class="sum-row sum-total">
					<span>TOTAL</span>
					<strong>${$total.toFixed(2)}</strong>
				</div>
			</div>

			<div class="divider"></div>

			<div class="pay-section">
				<label class="pay-label" for="paid-input">TENDERED</label>
				<div class="pay-input-wrap">
					<span class="currency-sign">$</span>
					<input
						id="paid-input"
						class="pay-input"
						type="number"
						min="0"
						step="0.01"
						bind:value={paid}
					/>
				</div>

				{#if shortfall > 0}
					<p class="shortfall">Still need ${shortfall.toFixed(2)}</p>
				{/if}

				<div class="sum-row change-row" class:highlight={change > 0}>
					<span>CHANGE</span>
					<strong>${change.toFixed(2)}</strong>
				</div>
			</div>

			{#if showClearConfirm}
				<div class="confirm-box">
					<p>Clear all items?</p>
					<div class="confirm-actions">
						<button class="btn btn-ghost" on:click={() => (showClearConfirm = false)}>Cancel</button
						>
						<button class="btn btn-danger" on:click={doClear}>Clear</button>
					</div>
				</div>
			{/if}
		</aside>
	</main>

	<footer class="bottombar">
		<div class="bottombar-left">
			<button class="btn btn-ghost" type="button" on:click={confirmClear}>Clear Cart</button>
		</div>
		<div class="bottombar-right">
			<button
				class="btn btn-checkout"
				disabled={isSubmitting || $cartItems.length === 0 || paid < $total}
				on:click={completeSale}
			>
				{isSubmitting ? 'Processing…' : 'Complete Sale'}
			</button>
		</div>
	</footer>
</div>

<style>
	:root {
		--bg: #0d1117;
		--surface: #161b22;
		--border: #21262d;
		--border2: #30363d;
		--text: #e6edf3;
		--muted: #8b949e;
		--accent: #f0b429;
		--success: #3fb950;
		--danger: #f85149;
		--mono: 'JetBrains Mono', 'Fira Mono', 'Courier New', monospace;
		--sans: 'DM Sans', 'Helvetica Neue', sans-serif;
		--label: 'Barlow Condensed', 'Arial Narrow', sans-serif;
	}

	:global(body) {
		margin: 0;
		padding: 0;
		overflow: hidden;
		background: var(--bg);
		color: var(--text);
		font-family: var(--sans);
		-webkit-font-smoothing: antialiased;
	}

	/* Take over the full viewport, sit above any layout wrappers */
	.pos {
		position: fixed;
		inset: 0;
		z-index: 100;
		display: grid;
		grid-template-rows: clamp(48px, 8vh, 72px) 1fr clamp(50px, 8vh, 70px);
		/* background: var(--bg); */
		border-radius: 25px;
	}

	/* ── TOPBAR ── */
	.topbar {
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		align-items: center;
		padding: 0 1.25rem;
		border-bottom: 1px solid var(--border);
		background: var(--surface);
	}

	.topbar-left {
		display: flex;
		align-items: center;
	}
	.topbar-center {
		display: flex;
		justify-content: center;
	}
	.topbar-right {
		display: flex;
		justify-content: flex-end;
	}
	.topbar-left {
		display: flex;
		align-items: center;
	}
	.topbar-center {
		display: flex;
		justify-content: center;
	}
	.topbar-right {
		display: flex;
		justify-content: flex-end;
		align-items: center;
		gap: 0.5rem;
	}

	/* Scale down whatever BackButton renders */
	.back-wrap {
		display: flex;
		align-items: center;
		transform: scale(0.72);
		transform-origin: left center;
	}

	.terminal-label {
		font-family: var(--label);
		font-size: 0.75rem;
		letter-spacing: 0.18em;
		color: var(--muted);
		font-weight: 600;
	}

	.badge-operator {
		display: flex;
		align-items: center;
		gap: 0.45rem;
		font-family: var(--label);
		font-size: 0.78rem;
		letter-spacing: 0.08em;
		color: var(--muted);
	}

	.badge-dot {
		width: 7px;
		height: 7px;
		background: var(--success);
		border-radius: 50%;
		box-shadow: 0 0 6px var(--success);
	}

	/* ── MAIN ── */
	.main {
		display: grid;
		grid-template-columns: 1fr 300px;
		overflow: hidden;
		gap: 1rem;
		padding: 1rem;
	}

	/* ── PANELS ── */
	.panel {
		display: flex;
		flex-direction: column;
		overflow: hidden;
		padding: 1.1rem 1.25rem;
		border-radius: 25px;
		background: var(--bg);
	}

	.cart-panel {
		border-right: 1px solid var(--border);
	}
	.summary-panel {
		background: var(--surface);
	}

	.panel-title {
		font-family: var(--label);
		font-size: 0.7rem;
		letter-spacing: 0.2em;
		color: var(--muted);
		font-weight: 700;
		margin-bottom: 0.9rem;
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.item-count {
		background: var(--border2);
		color: var(--text);
		font-size: 0.65rem;
		padding: 0.1rem 0.45rem;
		border-radius: 99px;
		letter-spacing: 0.05em;
	}

	/* ── SCAN ROW ── */
	.scan-row {
		display: flex;
		gap: 0.6rem;
		margin-bottom: 0.85rem;
	}

	.scan-input-wrap {
		flex: 1;
		display: flex;
		align-items: center;
		background: var(--bg);
		border: 1px solid var(--border2);
		border-radius: 8px;
		padding: 0 0.75rem;
		gap: 0.5rem;
		transition: border-color 0.15s;
	}

	.scan-input-wrap:focus-within {
		border-color: var(--accent);
	}
	.scan-icon {
		color: var(--muted);
		font-size: 1rem;
	}

	.scan-input {
		flex: 1;
		background: transparent;
		border: none;
		outline: none;
		color: var(--text);
		font-family: var(--mono);
		font-size: 0.9rem;
		padding: 0.65rem 0;
	}

	/* ── ALERTS ── */
	.alert {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.6rem 0.85rem;
		border-radius: 7px;
		font-size: 0.85rem;
		margin-bottom: 0.75rem;
		animation: slideIn 0.2s ease;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.alert-error {
		background: rgba(248, 81, 73, 0.1);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: #ffa198;
	}
	.alert-success {
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.3);
		color: #56d364;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 0.75rem;
	}
	.alert-copy {
		display: flex;
		align-items: center;
		gap: 0.45rem;
		flex: 1 1 auto;
	}
	.alert-link {
		font-family: var(--label);
		font-size: 0.72rem;
		letter-spacing: 0.18em;
		text-transform: uppercase;
		text-decoration: none;
		color: var(--accent);
		border: 1px solid rgba(240, 180, 41, 0.5);
		padding: 0.35rem 0.85rem;
		border-radius: 999px;
		transition:
			color 0.15s ease,
			border-color 0.15s ease,
			background 0.15s ease;
		flex: 0 0 auto;
	}
	.alert-link:hover,
	.alert-link:focus-visible {
		color: #0d1117;
		background: var(--accent);
		border-color: var(--accent);
	}
	.alert-icon {
		font-size: 0.8rem;
		flex-shrink: 0;
	}

	/* ── TABLE ── */
	.table-wrap {
		flex: 1;
		overflow: auto;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.88rem;
	}

	thead th {
		font-family: var(--label);
		font-size: 0.65rem;
		letter-spacing: 0.15em;
		color: var(--muted);
		font-weight: 700;
		padding: 0.5rem 0.6rem;
		border-bottom: 1px solid var(--border);
		text-align: left;
	}

	th.num,
	td.num {
		text-align: right;
	}

	.cart-row td {
		padding: 0.65rem 0.6rem;
		border-bottom: 1px solid var(--border);
		vertical-align: middle;
	}

	.cart-row:last-child td {
		border-bottom: none;
	}
	.cart-row:hover td {
		background: rgba(255, 255, 255, 0.02);
	}

	.name-cell {
		font-weight: 500;
	}
	.upc-cell {
		font-family: var(--mono);
		font-size: 0.78rem;
		color: var(--muted);
	}
	.line-total {
		font-weight: 600;
	}

	.qty-input {
		width: 58px;
		background: var(--bg);
		border: 1px solid var(--border2);
		border-radius: 6px;
		color: var(--text);
		font-family: var(--mono);
		font-size: 0.88rem;
		padding: 0.3rem 0.45rem;
		text-align: center;
		outline: none;
		transition: border-color 0.15s;
	}

	.qty-input:focus {
		border-color: var(--accent);
	}
	.remove-cell {
		text-align: right;
	}

	/* ── EMPTY STATE ── */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		color: var(--muted);
		gap: 0.75rem;
		text-align: center;
		font-size: 0.88rem;
	}

	.empty-icon {
		font-size: 2.5rem;
		opacity: 0.3;
	}

	/* ── BUTTONS ── */
	.btn {
		font-family: var(--label);
		font-size: 0.78rem;
		letter-spacing: 0.12em;
		font-weight: 700;
		padding: 0.6rem 1rem;
		border-radius: 7px;
		border: none;
		cursor: pointer;
		transition:
			opacity 0.15s,
			transform 0.1s;
	}

	.btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
	.btn:not(:disabled):active {
		transform: scale(0.97);
	}

	.btn-add {
		background: var(--accent);
		color: #0d1117;
		min-width: 60px;
	}
	.btn-ghost {
		background: transparent;
		border: 1px solid var(--border2);
		color: var(--muted);
	}
	.btn-ghost:hover:not(:disabled) {
		border-color: var(--text);
		color: var(--text);
	}
	.btn-danger {
		background: var(--danger);
		color: #fff;
	}

	.btn-checkout {
		background: var(--text);
		color: var(--bg);
		padding: 0.65rem 1.6rem;
		font-size: 0.82rem;
	}

	.btn-checkout:not(:disabled):hover {
		background: var(--accent);
	}

	.btn-remove {
		background: transparent;
		border: none;
		color: var(--muted);
		cursor: pointer;
		font-size: 0.85rem;
		padding: 0.25rem 0.4rem;
		border-radius: 4px;
		line-height: 1;
		transition:
			color 0.15s,
			background 0.15s;
	}

	.btn-remove:hover {
		color: var(--danger);
		background: rgba(248, 81, 73, 0.1);
	}

	/* ── SUMMARY ── */
	.summary-rows {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.sum-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-size: 0.88rem;
	}

	.sum-row span:first-child {
		color: var(--muted);
	}

	.sum-total {
		font-family: var(--label);
		font-size: 1.05rem;
		letter-spacing: 0.08em;
		font-weight: 700;
		padding-top: 0.6rem;
		border-top: 1px solid var(--border);
	}

	.divider {
		height: 1px;
		background: var(--border);
		margin: 0.85rem 0;
	}

	/* ── PAY SECTION ── */
	.pay-section {
		display: flex;
		flex-direction: column;
		gap: 0.65rem;
	}

	.pay-label {
		font-family: var(--label);
		font-size: 0.65rem;
		letter-spacing: 0.2em;
		color: var(--muted);
		font-weight: 700;
	}

	.pay-input-wrap {
		display: flex;
		align-items: center;
		background: var(--bg);
		border: 1px solid var(--border2);
		border-radius: 8px;
		padding: 0 0.75rem;
		gap: 0.35rem;
		transition: border-color 0.15s;
	}

	.pay-input-wrap:focus-within {
		border-color: var(--accent);
	}

	.currency-sign {
		color: var(--muted);
		font-family: var(--mono);
		font-size: 0.9rem;
	}

	.pay-input {
		flex: 1;
		background: transparent;
		border: none;
		outline: none;
		color: var(--text);
		font-family: var(--mono);
		font-size: 1rem;
		padding: 0.65rem 0;
	}

	.shortfall {
		font-size: 0.78rem;
		color: var(--danger);
		margin: 0;
		font-family: var(--label);
		letter-spacing: 0.05em;
	}

	.change-row {
		font-family: var(--label);
		font-size: 0.95rem;
		letter-spacing: 0.08em;
		font-weight: 700;
		padding: 0.6rem 0.75rem;
		border-radius: 7px;
		background: var(--border);
		transition:
			background 0.2s,
			color 0.2s;
	}

	.change-row.highlight {
		background: rgba(63, 185, 80, 0.12);
		color: var(--success);
	}

	/* ── CONFIRM BOX ── */
	.confirm-box {
		margin-top: auto;
		padding: 0.85rem;
		background: var(--bg);
		border: 1px solid var(--border2);
		border-radius: 8px;
		animation: slideIn 0.2s ease;
	}

	.confirm-box p {
		margin: 0 0 0.65rem;
		font-size: 0.85rem;
		color: var(--muted);
	}
	.confirm-actions {
		display: flex;
		gap: 0.5rem;
	}

	/* ── BOTTOM BAR ── */
	.bottombar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0 1.25rem;
		border-top: 1px solid var(--border);
		background: var(--surface);
	}

	.bottombar-left,
	.bottombar-right {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.mono {
		font-family: var(--mono);
	}

	/* ── RESPONSIVE ── */
	@media (max-width: 860px) {
		.main {
			grid-template-columns: 1fr;
			grid-template-rows: 1fr auto;
			overflow: auto;
		}

		.cart-panel {
			border-right: none;
			border-bottom: 1px solid var(--border);
		}
		.bottombar {
			flex-direction: column;
			gap: 0.6rem;
			padding: 0.75rem 1.25rem;
		}
		.bottombar-left,
		.bottombar-right {
			width: 100%;
			justify-content: center;
		}
	}
</style>
