import { getToken } from '$lib/stores/auth';

const BASE_URL = import.meta.env.VITE_POS_API_BASE ?? '';

async function authedFetch(path: string, init: RequestInit = {}): Promise<Response> {
	const token = await getToken();
	if (!token) {
		throw new Error('Not authenticated.');
	}

	const headers = new Headers(init.headers);
	headers.set('Authorization', `Bearer ${token}`);
	if (init.body && !headers.has('Content-Type')) {
		headers.set('Content-Type', 'application/json');
	}

	return fetch(`${BASE_URL}${path}`, {
		...init,
		headers
	});
}

async function handle<T>(res: Response): Promise<T> {
	if (!res.ok) {
		const contentType = res.headers.get('content-type') ?? '';
		if (contentType.includes('application/json')) {
			const data = (await res.json().catch(() => null)) as unknown;
			if (data && typeof data === 'object' && 'message' in data && typeof data.message === 'string') {
				throw new Error(data.message);
			}
		}
		const text = await res.text().catch(() => '');
		throw new Error(text || `Request failed (${res.status})`);
	}
	return (await res.json()) as T;
}

export type AdminProduct = {
	id: string;
	upc: string;
	name: string;
	price: number;
	quantity: number;
};

export async function getAdminProducts(query?: string): Promise<AdminProduct[]> {
	const q = query?.trim();
	const qs = q ? `?query=${encodeURIComponent(q)}` : '';
	const res = await authedFetch(`/api/admin/products${qs}`, { method: 'GET' });
	return handle<AdminProduct[]>(res);
}

export type CreateAdminProductPayload = {
	upc: string;
	name: string;
	price: number;
	quantity: number;
};

export async function createAdminProduct(payload: CreateAdminProductPayload): Promise<AdminProduct> {
	const res = await authedFetch('/api/admin/products', {
		method: 'POST',
		body: JSON.stringify(payload)
	});
	return handle<AdminProduct>(res);
}

export type PatchAdminProductPayload = Partial<CreateAdminProductPayload>;

export async function patchAdminProduct(id: string, payload: PatchAdminProductPayload): Promise<void> {
	const res = await authedFetch(`/api/admin/products/${encodeURIComponent(id)}`, {
		method: 'PATCH',
		body: JSON.stringify(payload)
	});
	if (!res.ok) {
		const text = await res.text().catch(() => '');
		throw new Error(text || `Request failed (${res.status})`);
	}
}

export async function deleteAdminProduct(id: string): Promise<void> {
	const res = await authedFetch(`/api/admin/products/${encodeURIComponent(id)}`, { method: 'DELETE' });
	if (!res.ok) {
		const text = await res.text().catch(() => '');
		throw new Error(text || `Request failed (${res.status})`);
	}
}

export type StaffRole = 'Admin' | 'Manager' | 'Cashier';
export type StaffStatus = 'Active' | 'Invited' | 'Suspended';

export type AdminStaffUser = {
	id: string;
	email: string;
	role: StaffRole;
	status: StaffStatus;
};

export async function getAdminUsers(query?: string): Promise<AdminStaffUser[]> {
	const q = query?.trim();
	const qs = q ? `?query=${encodeURIComponent(q)}` : '';
	const res = await authedFetch(`/api/admin/users${qs}`, { method: 'GET' });
	return handle<AdminStaffUser[]>(res);
}

export type CreateAdminUserPayload = {
	email: string;
	role: StaffRole;
};

export async function createAdminUser(payload: CreateAdminUserPayload): Promise<AdminStaffUser> {
	const res = await authedFetch('/api/admin/users', {
		method: 'POST',
		body: JSON.stringify(payload)
	});
	return handle<AdminStaffUser>(res);
}

export type PatchAdminUserPayload = {
	role?: StaffRole;
	status?: StaffStatus;
};

export async function patchAdminUser(id: string, payload: PatchAdminUserPayload): Promise<void> {
	const res = await authedFetch(`/api/admin/users/${encodeURIComponent(id)}`, {
		method: 'PATCH',
		body: JSON.stringify(payload)
	});
	if (!res.ok) {
		const text = await res.text().catch(() => '');
		throw new Error(text || `Request failed (${res.status})`);
	}
}

export type AdminSettings = {
	taxRate: number;
};

export async function getAdminSettings(): Promise<AdminSettings> {
	const res = await authedFetch('/api/admin/settings', { method: 'GET' });
	return handle<AdminSettings>(res);
}

export async function putAdminSettings(payload: AdminSettings): Promise<void> {
	const res = await authedFetch('/api/admin/settings', {
		method: 'PUT',
		body: JSON.stringify(payload)
	});
	if (!res.ok) {
		const text = await res.text().catch(() => '');
		throw new Error(text || `Request failed (${res.status})`);
	}
}
