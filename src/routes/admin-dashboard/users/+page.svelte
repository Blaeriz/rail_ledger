<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { fetchData, createItem, updateItem, deleteItem } from '$lib/utils';
	import DataTable from '$lib/components/DataTable.svelte';
	import Modal from '$lib/components/Modal.svelte';

	let user = null;
	let usersData = /** @type {any[]} */ ([]);
	let isLoading = true;
	let showUserModal = false;
	let editingItem = /** @type {any} */ (null);

	const userForm = {
		user_id: '',
		name: '',
		aadhar: '',
		phone_number: '',
		user_role: 'viewer'
	};

	const tableColumns = [
		{ key: 'user_id', label: 'USER ID' },
		{ key: 'name', label: 'NAME' },
		{ key: 'aadhar', label: 'AADHAR' },
		{ key: 'phone_number', label: 'PHONE' },
		{
			key: 'user_role',
			label: 'ROLE',
			render: (/** @type {any} */ item) => {
				return `<span class="text-gray-300 text-sm">${(item.user_role || 'viewer').toLowerCase()}</span>`;
			}
		},
		{
			key: 'actions',
			label: 'ACTIONS',
			render: (/** @type {any} */ item) => `
        <div class="flex space-x-2">
          <button onclick="editUser('${item.user_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20">Edit</button>
          <button onclick="deleteUser('${item.user_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-red-500/30 hover:border-red-400/50 hover:shadow-lg hover:shadow-red-500/20">Delete</button>
        </div>
      `
		}
	];

	onMount(async () => {
		const userData = sessionStorage.getItem('user');
		if (!userData) {
			goto('/login');
			return;
		}

		user = JSON.parse(userData);

		try {
			usersData = await fetchData('/api/users');
		} catch (error) {
			console.error('Error loading data:', error);
		} finally {
			isLoading = false;
		}
	});

	function openUserModal(user = null) {
		editingItem = user;
		if (user) {
			Object.assign(userForm, user);
		} else {
			Object.assign(userForm, {
				user_id: '',
				name: '',
				aadhar: '',
				phone_number: '',
				user_role: 'viewer'
			});
		}
		showUserModal = true;
	}

	function closeModals() {
		showUserModal = false;
		editingItem = null;
	}

	async function saveUser() {
		try {
			const success = editingItem
				? await updateItem('/api/users', userForm.user_id, userForm)
				: await createItem('/api/users', userForm);

			if (success) {
				usersData = await fetchData('/api/users');
				closeModals();
			}
		} catch (error) {
			console.error('Error saving user:', error);
		}
	}

	async function deleteUser(/** @type {string} */ userId) {
		if (confirm('Are you sure you want to delete this user?')) {
			try {
				const success = await deleteItem('/api/users', userId);
				if (success) {
					usersData = await fetchData('/api/users');
				}
			} catch (error) {
				console.error('Error deleting user:', error);
			}
		}
	}

	// Make functions globally available for inline onclick handlers
	window.editUser = (userId) => {
		const user = usersData.find((u) => u.user_id === userId);
		openUserModal(user);
	};
	window.deleteUser = deleteUser;
</script>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-center">
		<h1 class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">User Management</h1>
		<button
			on:click={() => openUserModal()}
			class="flex items-center justify-center space-x-2 rounded-lg border border-purple-500/30 bg-gray-800 px-6 py-3 text-sm font-medium text-white transition-all duration-300 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/30"
		>
			<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 6v6m0 0v6m0-6h6m-6 0H6"
				></path>
			</svg>
			<span>Add User</span>
		</button>
	</div>

	<!-- Users Table -->
	<DataTable
		data={usersData}
		columns={tableColumns}
		searchable={true}
		searchPlaceholder="Search users..."
		showPagination={true}
		pageSize={20}
	/>
</div>

<!-- User Modal -->
<Modal
	isOpen={showUserModal}
	title={editingItem ? 'Edit User' : 'Add New User'}
	onClose={closeModals}
>
	<form on:submit|preventDefault={saveUser} class="space-y-4">
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
			<div>
				<label for="user_id" class="mb-2 block text-sm font-medium text-gray-300">User ID</label>
				<input
					id="user_id"
					type="text"
					bind:value={userForm.user_id}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter user ID"
				/>
			</div>
			<div>
				<label for="name" class="mb-2 block text-sm font-medium text-gray-300">Name</label>
				<input
					id="name"
					type="text"
					bind:value={userForm.name}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter name"
				/>
			</div>
			<div>
				<label for="aadhar" class="mb-2 block text-sm font-medium text-gray-300"
					>Aadhar Number</label
				>
				<input
					id="aadhar"
					type="text"
					bind:value={userForm.aadhar}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter Aadhar number"
				/>
			</div>
			<div>
				<label for="phone_number" class="mb-2 block text-sm font-medium text-gray-300"
					>Phone Number</label
				>
				<input
					id="phone_number"
					type="text"
					bind:value={userForm.phone_number}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter phone number"
				/>
			</div>
			<div>
				<label for="user_role" class="mb-2 block text-sm font-medium text-gray-300">User Role</label
				>
				<select
					id="user_role"
					bind:value={userForm.user_role}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
				>
					<option value="viewer">Viewer</option>
					<option value="inspector">Inspector</option>
					<option value="admin">Admin</option>
				</select>
			</div>
		</div>

		<div class="flex flex-col justify-end space-y-2 pt-6 sm:flex-row sm:space-y-0 sm:space-x-4">
			<button
				type="button"
				on:click={closeModals}
				class="w-full rounded-lg border border-gray-600 bg-gray-800 px-6 py-3 text-sm text-white transition-all duration-300 hover:border-gray-500 hover:bg-gray-700 hover:shadow-lg hover:shadow-gray-500/10 sm:w-auto"
			>
				Cancel
			</button>
			<button
				type="submit"
				class="w-full rounded-lg border border-purple-500/30 bg-gray-800 px-6 py-3 text-sm text-white transition-all duration-300 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/30 sm:w-auto"
			>
				{editingItem ? 'Update User' : 'Add User'}
			</button>
		</div>
	</form>
</Modal>
