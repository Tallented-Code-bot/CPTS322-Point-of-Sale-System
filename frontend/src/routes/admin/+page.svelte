<svelte:head>
	<title>Admin Dashboard</title>
</svelte:head>

<script lang="ts">
	import {
		createAdminProduct,
		createAdminUser,
		deleteAdminProduct,
		getAdminProducts,
		getAdminSettings,
		getAdminUsers,
		patchAdminProduct,
		patchAdminUser,
		putAdminSettings,
		type AdminProduct,
		type AdminStaffUser
	} from '$lib/api/admin';
	import { resolve } from '$app/paths';
	import { onMount } from 'svelte';

	type SectionKey = 'products' | 'users' | 'settings';

	type Product = AdminProduct;
	type UserRow = AdminStaffUser;

	let activeSection = $state<SectionKey>('products');
	let loading = $state(false);
	let error = $state<string | null>(null);

	let productSearch = $state('');
	let products = $state<Product[]>([]);

	const filteredProducts = $derived.by(() => {
		const q = productSearch.trim().toLowerCase();
		if (!q) return products;
		return products.filter((p) => {
			return (
				p.upc.toLowerCase().includes(q) ||
				p.name.toLowerCase().includes(q) ||
				String(p.price).includes(q) ||
				String(p.quantity).includes(q)
			);
		});
	});

	let productModalOpen = $state(false);
	let productModalMode = $state<'add' | 'edit'>('add');
	let editingProductId = $state<string | null>(null);
	let productForm = $state({
		upc: '',
		name: '',
		price: '0.00',
		qty: '0'
	});

	function openAddProduct() {
		productModalMode = 'add';
		editingProductId = null;
		productForm.upc = '';
		productForm.name = '';
		productForm.price = '0.00';
		productForm.qty = '0';
		productModalOpen = true;
	}

	function openEditProduct(p: Product) {
		productModalMode = 'edit';
		editingProductId = p.id;
		productForm.upc = p.upc;
		productForm.name = p.name;
		productForm.price = p.price.toFixed(2);
		productForm.qty = String(p.quantity);
		productModalOpen = true;
	}

	function closeProductModal() {
		productModalOpen = false;
	}

	let userSearch = $state('');
	let users = $state<UserRow[]>([]);

	const filteredUsers = $derived.by(() => {
		const q = userSearch.trim().toLowerCase();
		if (!q) return users;
		return users.filter((u) => {
			return u.email.toLowerCase().includes(q) || u.role.toLowerCase().includes(q) || u.status.toLowerCase().includes(q);
		});
	});

	let userModalOpen = $state(false);
	let userModalMode = $state<'add' | 'edit'>('add');
	let editingUserId = $state<string | null>(null);
	let userForm = $state({
		email: '',
		role: 'Cashier' as UserRow['role'],
		status: 'Invited' as UserRow['status']
	});

	function openAddUser() {
		userModalMode = 'add';
		editingUserId = null;
		userForm.email = '';
		userForm.role = 'Cashier';
		userForm.status = 'Invited';
		userModalOpen = true;
	}

	function openEditUser(u: UserRow) {
		userModalMode = 'edit';
		editingUserId = u.id;
		userForm.email = u.email;
		userForm.role = u.role;
		userForm.status = u.status;
		userModalOpen = true;
	}

	function closeUserModal() {
		userModalOpen = false;
	}

	let taxRate = $state('8.7');

	function percentStringFromFraction(value: number): string {
		const pct = value * 100;
		if (!Number.isFinite(pct)) return '0';
		const rounded = Math.round(pct * 100) / 100;
		return String(rounded);
	}

	function fractionFromPercentString(value: string): number {
		const parsed = Number.parseFloat(value);
		if (!Number.isFinite(parsed)) return 0;
		const fraction = parsed / 100;
		return Math.min(1, Math.max(0, fraction));
	}

	async function refreshProducts() {
		products = await getAdminProducts();
	}

	async function refreshUsers() {
		users = await getAdminUsers();
	}

	async function refreshSettings() {
		const next = await getAdminSettings();
		taxRate = percentStringFromFraction(next.taxRate);
	}

	async function refreshAll() {
		loading = true;
		error = null;
		try {
			await Promise.all([refreshProducts(), refreshUsers(), refreshSettings()]);
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to load admin data.';
		} finally {
			loading = false;
		}
	}

	async function saveProduct() {
		loading = true;
		error = null;
		try {
			const price = Number.parseFloat(productForm.price);
			const quantity = Number.parseInt(productForm.qty, 10);
			if (!productForm.upc.trim() || !productForm.name.trim()) {
				throw new Error('UPC and name are required.');
			}
			if (!Number.isFinite(price) || !Number.isFinite(quantity)) {
				throw new Error('Price and quantity must be valid numbers.');
			}

			if (productModalMode === 'add') {
				await createAdminProduct({
					upc: productForm.upc.trim(),
					name: productForm.name.trim(),
					price,
					quantity
				});
			} else {
				if (!editingProductId) throw new Error('Missing product id.');
				await patchAdminProduct(editingProductId, {
					upc: productForm.upc.trim(),
					name: productForm.name.trim(),
					price,
					quantity
				});
			}

			await refreshProducts();
			closeProductModal();
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to save product.';
		} finally {
			loading = false;
		}
	}

	async function removeProduct(id: string) {
		loading = true;
		error = null;
		try {
			await deleteAdminProduct(id);
			await refreshProducts();
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to remove product.';
		} finally {
			loading = false;
		}
	}

	async function saveUser() {
		loading = true;
		error = null;
		try {
			if (!userForm.email.trim()) throw new Error('Email is required.');
			if (userModalMode === 'add') {
				await createAdminUser({ email: userForm.email.trim(), role: userForm.role });
			} else {
				if (!editingUserId) throw new Error('Missing user id.');
				await patchAdminUser(editingUserId, { role: userForm.role, status: userForm.status });
			}
			await refreshUsers();
			closeUserModal();
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to save user.';
		} finally {
			loading = false;
		}
	}

	async function disableUser(id: string) {
		loading = true;
		error = null;
		try {
			await patchAdminUser(id, { status: 'Suspended' });
			await refreshUsers();
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to disable user.';
		} finally {
			loading = false;
		}
	}

	async function saveTaxRate() {
		loading = true;
		error = null;
		try {
			const nextRate = fractionFromPercentString(taxRate);
			await putAdminSettings({ taxRate: nextRate });
			await refreshSettings();
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'Failed to save settings.';
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		void refreshAll();
	});

	function formatMoney(value: number) {
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(value);
	}
</script>

<div class="page">
	<aside class="sidebar">
		<div class="brand">
			<div class="mark" aria-hidden="true">POS</div>
			<div class="brandText">
				<div class="appName">Point of Sale</div>
				<div class="sub">Admin Dashboard</div>
			</div>
		</div>

		<nav class="nav" aria-label="Admin sections">
			<button
				type="button"
				class="navItem"
				class:active={activeSection === 'products'}
				aria-current={activeSection === 'products' ? 'page' : undefined}
				onclick={() => (activeSection = 'products')}
			>
				Products
			</button>
			<button
				type="button"
				class="navItem"
				class:active={activeSection === 'users'}
				aria-current={activeSection === 'users' ? 'page' : undefined}
				onclick={() => (activeSection = 'users')}
			>
				Users
			</button>
			<button
				type="button"
				class="navItem"
				class:active={activeSection === 'settings'}
				aria-current={activeSection === 'settings' ? 'page' : undefined}
				onclick={() => (activeSection = 'settings')}
			>
				Settings
			</button>
		</nav>

		<div class="sidebarFooter">
			<a class="subtleLink" href={resolve('/home')}>Back to Home</a>
		</div>
	</aside>

	<main class="main">
		{#if error}
			<div class="alert" role="alert">{error}</div>
		{/if}
		{#if loading}
			<div class="alert subtle" aria-live="polite">Loading…</div>
		{/if}

		{#if activeSection === 'products'}
			<header class="sectionHeader">
				<div>
					<h1>Products</h1>
					<p>Browse and manage your catalog.</p>
				</div>
				<div class="headerActions">
					<span class="pill">Inventory</span>
					<span class="pill">Pricing</span>
				</div>
			</header>

			<section class="panel">
				<div class="toolbar">
					<label class="search">
						<span class="srOnly">Search products</span>
						<input
							type="search"
							placeholder="Search by UPC, name, price, qty…"
							bind:value={productSearch}
						/>
					</label>
					<button type="button" class="btn primary" onclick={openAddProduct}>Add Product</button>
				</div>

				<div class="tableWrap" role="region" aria-label="Products table">
					<table class="table">
						<thead>
							<tr>
								<th>UPC</th>
								<th>Name</th>
								<th class="num">Price</th>
								<th class="num">Qty</th>
								<th class="actions">Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each filteredProducts as p (p.id)}
								<tr>
									<td class="mono">{p.upc}</td>
									<td>
										<div class="cellTitle">{p.name}</div>
										<div class="cellSub">SKU: {p.id}</div>
									</td>
								<td class="num mono">{formatMoney(p.price)}</td>
								<td class="num mono">{p.quantity}</td>
								<td class="actions">
									<div class="rowActions">
										<button type="button" class="btn" onclick={() => openEditProduct(p)}>Edit</button>
										<button type="button" class="btn danger" onclick={() => removeProduct(p.id)} disabled={loading}>
											Remove
										</button>
									</div>
								</td>
							</tr>
							{:else}
								<tr>
									<td colspan="5" class="empty">No matching products.</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</section>

			{#if productModalOpen}
				<div
					class="modalOverlay"
					role="button"
					tabindex="0"
					aria-label="Close product dialog"
					onclick={closeProductModal}
					onkeydown={(e) => {
						if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') closeProductModal();
					}}
				>
					<div
						class="modal"
						role="dialog"
						tabindex="-1"
						aria-modal="true"
						aria-label="Product editor"
						onclick={(e) => e.stopPropagation()}
						onkeydown={(e) => {
							if (e.key === 'Escape') closeProductModal();
						}}
					>
						<div class="modalHeader">
							<div>
								<div class="modalTitle">
									{productModalMode === 'add' ? 'Add Product' : 'Edit Product'}
								</div>
								<div class="modalSub">
									{productModalMode === 'add'
										? 'Create a new catalog item (mock UI).'
										: `Editing ${editingProductId ?? ''} (mock UI).`}
								</div>
							</div>
							<button type="button" class="iconBtn" aria-label="Close" onclick={closeProductModal}>
								×
							</button>
						</div>

						<form class="form" onsubmit={(e) => e.preventDefault()}>
							<div class="grid2">
								<label class="field">
									<span>UPC</span>
									<input class="mono" placeholder="012345678905" bind:value={productForm.upc} />
								</label>
								<label class="field">
									<span>Name</span>
									<input placeholder="Product name" bind:value={productForm.name} />
								</label>
							</div>
							<div class="grid2">
								<label class="field">
									<span>Price</span>
									<input inputmode="decimal" class="mono" placeholder="0.00" bind:value={productForm.price} />
								</label>
								<label class="field">
									<span>Quantity</span>
									<input inputmode="numeric" class="mono" placeholder="0" bind:value={productForm.qty} />
								</label>
							</div>

							<div class="modalFooter">
								<button type="button" class="btn" onclick={closeProductModal}>Cancel</button>
								<button type="button" class="btn primary" onclick={saveProduct} disabled={loading}>Save</button>
							</div>
						</form>
					</div>
				</div>
			{/if}
		{:else if activeSection === 'users'}
			<header class="sectionHeader">
				<div>
					<h1>Users</h1>
					<p>Invite staff and manage access.</p>
				</div>
				<div class="headerActions">
					<span class="pill">Roles</span>
					<span class="pill">Status</span>
				</div>
			</header>

			<section class="panel">
				<div class="toolbar">
					<label class="search">
						<span class="srOnly">Search users</span>
						<input type="search" placeholder="Search by email, role, status…" bind:value={userSearch} />
					</label>
					<button type="button" class="btn primary" onclick={openAddUser}>Add User</button>
				</div>

				<div class="tableWrap" role="region" aria-label="Users table">
					<table class="table">
						<thead>
							<tr>
								<th>Email</th>
								<th>Role</th>
								<th>Status</th>
								<th class="actions">Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each filteredUsers as u (u.id)}
								<tr>
									<td class="mono">{u.email}</td>
									<td>
										<span class="tag">{u.role}</span>
									</td>
									<td>
										<span class="status {u.status.toLowerCase()}">{u.status}</span>
									</td>
								<td class="actions">
									<div class="rowActions">
										<button type="button" class="btn" onclick={() => openEditUser(u)}>Edit</button>
										<button
											type="button"
											class="btn danger"
											onclick={() => disableUser(u.id)}
											disabled={loading || u.status === 'Suspended'}
										>
											Disable
										</button>
									</div>
								</td>
							</tr>
							{:else}
								<tr>
									<td colspan="4" class="empty">No matching users.</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</section>

			{#if userModalOpen}
				<div
					class="modalOverlay"
					role="button"
					tabindex="0"
					aria-label="Close user dialog"
					onclick={closeUserModal}
					onkeydown={(e) => {
						if (e.key === 'Escape' || e.key === 'Enter' || e.key === ' ') closeUserModal();
					}}
				>
					<div
						class="modal"
						role="dialog"
						tabindex="-1"
						aria-modal="true"
						aria-label="Add user"
						onclick={(e) => e.stopPropagation()}
						onkeydown={(e) => {
							if (e.key === 'Escape') closeUserModal();
						}}
					>
						<div class="modalHeader">
							<div>
								<div class="modalTitle">{userModalMode === 'add' ? 'Add User' : 'Edit User'}</div>
								<div class="modalSub">
									{userModalMode === 'add' ? 'Create an invite.' : `Editing ${editingUserId ?? ''}.`}
								</div>
							</div>
							<button type="button" class="iconBtn" aria-label="Close" onclick={closeUserModal}>
								×
							</button>
						</div>

						<form class="form" onsubmit={(e) => e.preventDefault()}>
							<label class="field">
								<span>Email</span>
								<input
									class="mono"
									placeholder="name@company.com"
									bind:value={userForm.email}
									disabled={userModalMode === 'edit'}
								/>
							</label>
							<div class="grid2">
								<label class="field">
									<span>Role</span>
									<select bind:value={userForm.role}>
										<option value="Admin">Admin</option>
										<option value="Manager">Manager</option>
										<option value="Cashier">Cashier</option>
									</select>
								</label>
								<label class="field">
									<span>Status</span>
									<select bind:value={userForm.status}>
										<option value="Invited">Invited</option>
										<option value="Active">Active</option>
										<option value="Suspended">Suspended</option>
									</select>
								</label>
							</div>
							<div class="modalFooter">
								<button type="button" class="btn" onclick={closeUserModal}>Cancel</button>
								<button type="button" class="btn primary" onclick={saveUser} disabled={loading}>Save</button>
							</div>
						</form>
					</div>
				</div>
			{/if}
		{:else}
			<header class="sectionHeader">
				<div>
					<h1>Settings</h1>
					<p>Configure system defaults.</p>
				</div>
				<div class="headerActions">
					<span class="pill">Store</span>
					<span class="pill">Taxes</span>
				</div>
			</header>

			<div class="cards">
				<section class="panel card">
					<div class="cardHeader">
						<div>
							<h2>Tax Rate</h2>
							<p>Applied at checkout. Enter a percent.</p>
						</div>
					</div>
					<div class="formRow">
						<label class="field inline">
							<span>Rate</span>
							<div class="inputAffix">
								<input class="mono" inputmode="decimal" bind:value={taxRate} />
								<span class="affix">%</span>
							</div>
						</label>
						<button type="button" class="btn primary" onclick={saveTaxRate} disabled={loading}>Save</button>
					</div>
				</section>

				<section class="panel card">
					<div class="cardHeader">
						<div>
							<h2>Etc.</h2>
							<p>Placeholder settings to flesh out the layout.</p>
						</div>
					</div>
					<div class="settingsList">
						<div class="settingRow">
							<div>
								<div class="settingKey">Receipt Footer</div>
								<div class="settingSub">Printed message at bottom of receipts.</div>
							</div>
							<button type="button" class="btn" onclick={() => {}}>Edit</button>
						</div>
						<div class="settingRow">
							<div>
								<div class="settingKey">Low Stock Threshold</div>
								<div class="settingSub">Warn when inventory drops below this.</div>
							</div>
							<button type="button" class="btn" onclick={() => {}}>Edit</button>
						</div>
						<div class="settingRow">
							<div>
								<div class="settingKey">Default Cash Drawer</div>
								<div class="settingSub">Connected drawer identifier (mock).</div>
							</div>
							<button type="button" class="btn" onclick={() => {}}>Edit</button>
						</div>
					</div>
				</section>
			</div>
		{/if}
	</main>
</div>

<style>
	:root {
		/* Match POS pages (home/cashier) */
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

		/* Admin-only helpers */
		--panel: rgba(22, 27, 34, 0.82);
		--panel2: rgba(22, 27, 34, 0.6);
		--faint: rgba(230, 237, 243, 0.6);
		--shadow: rgba(0, 0, 0, 0.6);
		--accent2: #63b3ed;
		--warning: #ffd166;
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

	.page {
		position: fixed;
		inset: 0;
		z-index: 100;
		display: grid;
		grid-template-columns: 280px 1fr;
		background: var(--bg);
		color: var(--text);
		border-radius: 25px;
		overflow: hidden;
	}

	.sidebar {
		position: relative;
		height: 100%;
		padding: 1.1rem 1rem;
		box-sizing: border-box;
		border-right: 1px solid var(--border);
		background: linear-gradient(180deg, rgba(22, 27, 34, 0.95), rgba(22, 27, 34, 0.75));
		backdrop-filter: blur(10px);
		display: grid;
		grid-template-rows: auto 1fr auto;
		gap: 1.25rem;
	}

	.brand {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.mark {
		width: 42px;
		height: 42px;
		border-radius: 12px;
		display: grid;
		place-items: center;
		font-weight: 800;
		letter-spacing: 0.04em;
		color: rgba(13, 17, 23, 0.92);
		background: linear-gradient(135deg, var(--accent), rgba(99, 179, 237, 0.95));
		box-shadow: 0 10px 24px rgba(0, 0, 0, 0.35);
	}

	.appName {
		font-weight: 800;
		letter-spacing: 0.01em;
		line-height: 1.15;
	}

	.sub {
		margin-top: 0.1rem;
		font-size: 0.9rem;
		color: var(--muted);
	}

	.nav {
		display: grid;
		gap: 0.4rem;
		align-content: start;
	}

	.navItem {
		width: 100%;
		text-align: left;
		padding: 0.65rem 0.8rem;
		border-radius: 12px;
		border: 1px solid transparent;
		background: transparent;
		color: var(--muted);
		font-weight: 650;
		letter-spacing: 0.01em;
		cursor: pointer;
		transition:
			background 120ms ease,
			border-color 120ms ease,
			color 120ms ease,
			transform 120ms ease;
	}

	.navItem:hover {
		background: rgba(255, 255, 255, 0.04);
		border-color: rgba(255, 255, 255, 0.08);
		color: rgba(255, 255, 255, 0.92);
	}

	.navItem.active {
		background: linear-gradient(180deg, rgba(240, 180, 41, 0.18), rgba(240, 180, 41, 0.06));
		border-color: rgba(240, 180, 41, 0.22);
		color: rgba(255, 255, 255, 0.95);
	}

	.sidebarFooter {
		padding-top: 0.25rem;
		border-top: 1px solid rgba(255, 255, 255, 0.06);
	}

	.subtleLink {
		display: inline-block;
		color: rgba(255, 255, 255, 0.7);
		text-decoration: none;
		font-weight: 600;
		padding: 0.5rem 0;
	}

	.subtleLink:hover {
		color: rgba(255, 255, 255, 0.92);
		text-decoration: underline;
	}

	.main {
		padding: 1.5rem 1.5rem 2rem;
		box-sizing: border-box;
		overflow: auto;
	}

	.sectionHeader {
		display: flex;
		justify-content: space-between;
		align-items: flex-end;
		gap: 1rem;
		margin: 0 0 1rem;
	}

	.sectionHeader h1 {
		margin: 0;
		font-size: 1.75rem;
		letter-spacing: -0.02em;
	}

	.sectionHeader p {
		margin: 0.35rem 0 0;
		color: var(--muted);
		line-height: 1.5;
	}

	.headerActions {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
		justify-content: flex-end;
	}

	.pill {
		font-size: 0.85rem;
		color: rgba(255, 255, 255, 0.82);
		border: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.04);
		border-radius: 999px;
		padding: 0.3rem 0.55rem;
		white-space: nowrap;
	}

	.panel {
		border-radius: 16px;
		background: var(--panel);
		border: 1px solid var(--border);
		box-shadow: 0 18px 60px var(--shadow);
		backdrop-filter: blur(10px);
	}

	.toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		padding: 0.95rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.06);
	}

	.search {
		flex: 1;
		max-width: 520px;
	}

	.search input {
		width: 100%;
		padding: 0.62rem 0.75rem;
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(10, 13, 20, 0.55);
		color: rgba(255, 255, 255, 0.92);
		outline: none;
		box-sizing: border-box;
	}

	.search input:focus {
		border-color: rgba(240, 180, 41, 0.55);
		box-shadow: 0 0 0 4px rgba(240, 180, 41, 0.15);
	}

	.btn {
		border-radius: 12px;
		padding: 0.6rem 0.8rem;
		border: 1px solid rgba(255, 255, 255, 0.12);
		background: rgba(255, 255, 255, 0.04);
		color: rgba(255, 255, 255, 0.88);
		font-weight: 650;
		cursor: pointer;
		transition:
			transform 120ms ease,
			border-color 120ms ease,
			background 120ms ease;
	}

	.btn:hover {
		transform: translateY(-1px);
		border-color: rgba(255, 255, 255, 0.18);
		background: rgba(255, 255, 255, 0.06);
	}

	.btn:active {
		transform: translateY(0);
	}

	.btn.primary {
		border-color: rgba(240, 180, 41, 0.35);
		background: linear-gradient(135deg, rgba(240, 180, 41, 0.22), rgba(240, 180, 41, 0.12));
		color: rgba(255, 255, 255, 0.94);
	}

	.btn.primary:hover {
		border-color: rgba(240, 180, 41, 0.5);
	}

	.btn.danger {
		border-color: rgba(248, 81, 73, 0.35);
		color: rgba(255, 255, 255, 0.9);
	}

	.tableWrap {
		overflow-x: auto;
	}

	.table {
		width: 100%;
		border-collapse: separate;
		border-spacing: 0;
		min-width: 760px;
	}

	.table thead th {
		text-align: left;
		font-size: 0.85rem;
		letter-spacing: 0.02em;
		text-transform: uppercase;
		color: var(--faint);
		padding: 0.85rem 0.95rem;
		background: rgba(255, 255, 255, 0.02);
		border-bottom: 1px solid rgba(255, 255, 255, 0.06);
	}

	.table tbody td {
		padding: 0.85rem 0.95rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
		color: rgba(255, 255, 255, 0.86);
		vertical-align: middle;
	}

	.table tbody tr:hover td {
		background: rgba(255, 255, 255, 0.03);
	}

	.table .num {
		text-align: right;
	}

	.table th.actions,
	.table td.actions {
		text-align: right;
		width: 1%;
		white-space: nowrap;
	}

	.rowActions {
		display: inline-flex;
		gap: 0.5rem;
	}

	.cellTitle {
		font-weight: 700;
		line-height: 1.2;
	}

	.cellSub {
		margin-top: 0.2rem;
		font-size: 0.85rem;
		color: var(--faint);
	}

	.empty {
		text-align: center;
		color: var(--muted);
		padding: 1.2rem 0.95rem;
	}

	.alert {
		margin: 0 0 1rem;
		padding: 0.75rem 0.85rem;
		border-radius: 14px;
		border: 1px solid rgba(248, 81, 73, 0.28);
		background: rgba(248, 81, 73, 0.08);
		color: rgba(255, 255, 255, 0.9);
		font-weight: 650;
	}

	.alert.subtle {
		border-color: rgba(255, 255, 255, 0.12);
		background: rgba(255, 255, 255, 0.04);
		color: rgba(255, 255, 255, 0.82);
	}

	.mono {
		font-variant-numeric: tabular-nums;
		font-feature-settings: 'tnum' 1;
		font-family:
			ui-monospace,
			SFMono-Regular,
			Menlo,
			Monaco,
			Consolas,
			Liberation Mono,
			Courier New,
			monospace;
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.22rem 0.5rem;
		border-radius: 999px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.04);
		color: rgba(255, 255, 255, 0.85);
		font-weight: 650;
		font-size: 0.9rem;
	}

	.status {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 650;
		color: rgba(255, 255, 255, 0.86);
	}

	.status::before {
		content: '';
		width: 8px;
		height: 8px;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.35);
	}

	.status.active::before {
		background: var(--success);
	}

	.status.invited::before {
		background: var(--accent);
	}

	.status.suspended::before {
		background: var(--warning);
	}

	.modalOverlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.55);
		display: grid;
		place-items: center;
		padding: 1rem;
		z-index: 50;
	}

	.modal {
		width: min(720px, 92vw);
		border-radius: 18px;
		border: 1px solid var(--border2);
		background: linear-gradient(180deg, rgba(28, 34, 47, 0.92), rgba(17, 21, 31, 0.86));
		box-shadow: 0 26px 90px rgba(0, 0, 0, 0.65);
		backdrop-filter: blur(12px);
		overflow: hidden;
	}

	.modalHeader {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		padding: 1rem 1rem 0.85rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.06);
	}

	.modalTitle {
		font-weight: 800;
		letter-spacing: -0.01em;
		font-size: 1.1rem;
	}

	.modalSub {
		margin-top: 0.25rem;
		color: var(--muted);
		font-size: 0.92rem;
		line-height: 1.4;
	}

	.iconBtn {
		width: 40px;
		height: 40px;
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.12);
		background: rgba(255, 255, 255, 0.05);
		color: rgba(255, 255, 255, 0.85);
		font-size: 1.4rem;
		line-height: 1;
		cursor: pointer;
	}

	.iconBtn:hover {
		background: rgba(255, 255, 255, 0.08);
	}

	.form {
		padding: 1rem;
	}

	.field {
		display: grid;
		gap: 0.35rem;
		color: rgba(255, 255, 255, 0.88);
		font-weight: 650;
	}

	.field span {
		font-size: 0.9rem;
		color: rgba(255, 255, 255, 0.8);
		font-weight: 650;
	}

	.field input,
	.field select {
		width: 100%;
		padding: 0.65rem 0.75rem;
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.12);
		background: rgba(10, 13, 20, 0.55);
		color: rgba(255, 255, 255, 0.92);
		outline: none;
		box-sizing: border-box;
	}

	.field input:focus,
	.field select:focus {
		border-color: rgba(240, 180, 41, 0.55);
		box-shadow: 0 0 0 4px rgba(240, 180, 41, 0.15);
	}

	.grid2 {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
		margin-bottom: 0.85rem;
	}

	.modalFooter {
		display: flex;
		justify-content: flex-end;
		gap: 0.6rem;
		padding-top: 0.6rem;
	}

	.cards {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
	}

	.card {
		padding: 1rem;
	}

	.cardHeader h2 {
		margin: 0;
		font-size: 1.1rem;
		letter-spacing: -0.01em;
	}

	.cardHeader p {
		margin: 0.35rem 0 0;
		color: var(--muted);
		line-height: 1.5;
	}

	.formRow {
		margin-top: 0.9rem;
		display: flex;
		align-items: flex-end;
		justify-content: space-between;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.field.inline {
		min-width: 260px;
		flex: 1;
	}

	.inputAffix {
		display: grid;
		grid-template-columns: 1fr auto;
		align-items: center;
		gap: 0.5rem;
	}

	.affix {
		color: var(--muted);
		font-weight: 750;
	}

	.settingsList {
		margin-top: 0.9rem;
		display: grid;
		gap: 0.6rem;
	}

	.settingRow {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		border-radius: 14px;
		border: 1px solid rgba(255, 255, 255, 0.08);
		background: rgba(255, 255, 255, 0.03);
	}

	.settingKey {
		font-weight: 750;
		color: rgba(255, 255, 255, 0.9);
	}

	.settingSub {
		margin-top: 0.2rem;
		color: var(--muted);
		font-size: 0.9rem;
	}

	.srOnly {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}

	@media (max-width: 720px) {
		.page {
			grid-template-columns: 1fr;
			grid-template-rows: auto 1fr;
		}

		.sidebar {
			position: relative;
			height: auto;
			border-right: none;
			border-bottom: 1px solid var(--border);
			grid-template-rows: auto auto;
			gap: 0.75rem;
		}

		.nav {
			display: flex;
			gap: 0.5rem;
			overflow-x: auto;
			padding-bottom: 0.2rem;
		}

		.navItem {
			white-space: nowrap;
			text-align: center;
		}

		.sidebarFooter {
			display: none;
		}

		.main {
			padding: 1rem;
		}

		.sectionHeader {
			flex-direction: column;
			align-items: flex-start;
		}

		.cards {
			grid-template-columns: 1fr;
		}
	}
</style>
