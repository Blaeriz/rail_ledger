<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { fetchData, createItem, updateItem, deleteItem } from '$lib/utils';
	import DataTable from '$lib/components/DataTable.svelte';
	import Modal from '$lib/components/Modal.svelte';

	let user = null;
	let vendorData = /** @type {any[]} */ ([]);
	let isLoading = true;
	let showVendorModal = false;
	let editingItem = /** @type {any} */ (null);

	const vendorForm = {
		vendor_id: '',
		city: '',
		state: '',
		no_of_batches: '',
		email: '',
		gst_no: '',
		pan_number: '',
		phone_number: '',
		audit_date: ''
	};

	const tableColumns = [
		{ key: 'vendor_id', label: 'VENDOR ID' },
		{ key: 'city', label: 'CITY' },
		{ key: 'state', label: 'STATE' },
		{ key: 'email', label: 'EMAIL' },
		{ key: 'phone_number', label: 'PHONE' },
		{ key: 'no_of_batches', label: 'BATCHES' },
		{
			key: 'actions',
			label: 'ACTIONS',
			render: (/** @type {any} */ item) => `
        <div class="flex space-x-2">
          <button onclick="editVendor('${item.vendor_id}')" class="group relative flex items-center justify-center space-x-1 overflow-hidden rounded-lg border border-purple-500/30 bg-gray-800 px-3 py-1 text-xs font-medium text-white transition-all duration-500 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/30 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/40 sm:text-sm">
            <div class="absolute inset-0 bg-gradient-to-r from-purple-500/10 to-blue-500/10 opacity-0 transition-opacity duration-500 group-hover:opacity-100"></div>
            <span class="relative z-10">Edit</span>
          </button>
          <button onclick="deleteVendor('${item.vendor_id}')" class="group relative flex items-center justify-center space-x-1 overflow-hidden rounded-lg border border-red-500/30 bg-gray-800 px-3 py-1 text-xs font-medium text-white transition-all duration-500 hover:border-red-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-red-500/30 active:border-red-400/60 active:bg-gray-600 active:shadow-xl active:shadow-red-500/40 sm:text-sm">
            <div class="absolute inset-0 bg-gradient-to-r from-red-500/10 to-pink-500/10 opacity-0 transition-opacity duration-500 group-hover:opacity-100"></div>
            <span class="relative z-10">Delete</span>
          </button>
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
			vendorData = await fetchData('/api/vendors');
		} catch (error) {
			console.error('Error loading data:', error);
		} finally {
			isLoading = false;
		}
	});

	function openVendorModal(vendor = null) {
		editingItem = vendor;
		if (vendor) {
			Object.assign(vendorForm, vendor);
		} else {
			Object.assign(vendorForm, {
				vendor_id: '',
				city: '',
				state: '',
				no_of_batches: '',
				email: '',
				gst_no: '',
				pan_number: '',
				phone_number: '',
				audit_date: ''
			});
		}
		showVendorModal = true;
	}

	function closeModals() {
		showVendorModal = false;
		editingItem = null;
	}

	async function saveVendor() {
		try {
			const success = editingItem
				? await updateItem('/api/vendors', vendorForm.vendor_id, vendorForm)
				: await createItem('/api/vendors', vendorForm);

			if (success) {
				vendorData = await fetchData('/api/vendors');
				closeModals();
			}
		} catch (error) {
			console.error('Error saving vendor:', error);
		}
	}

	async function deleteVendor(/** @type {string} */ vendorId) {
		if (confirm('Are you sure you want to delete this vendor?')) {
			try {
				const success = await deleteItem('/api/vendors', vendorId);
				if (success) {
					vendorData = await fetchData('/api/vendors');
				}
			} catch (error) {
				console.error('Error deleting vendor:', error);
			}
		}
	}

	// Make functions globally available for inline onclick handlers
	window.editVendor = (vendorId) => {
		const vendor = vendorData.find((v) => v.vendor_id === vendorId);
		openVendorModal(vendor);
	};
	window.deleteVendor = deleteVendor;
</script>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-center">
		<h1 class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">Vendor Management</h1>
		<button
			on:click={() => openVendorModal()}
			class="group animate-slide-in-left interactive relative flex items-center justify-center space-x-2 overflow-hidden rounded-lg border border-purple-500/30 bg-gray-800 px-6 py-3 text-sm font-medium text-white transition-all duration-500 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-2xl hover:shadow-purple-500/30 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/40"
		>
			<div
				class="absolute inset-0 bg-gradient-to-r from-purple-500/10 to-blue-500/10 opacity-0 transition-opacity duration-500 group-hover:opacity-100"
			></div>
			<div class="relative z-10 flex items-center space-x-2">
				<svg
					class="h-4 w-4 transition-transform duration-300 group-hover:scale-110"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 6v6m0 0v6m0-6h6m-6 0H6"
					></path>
				</svg>
				<span class="transition-transform duration-300 group-hover:translate-x-1">+ Add Vendor</span>
			</div>
		</button>
	</div>

	<!-- Vendor Table -->
	<DataTable
		data={vendorData}
		columns={tableColumns}
		searchable={true}
		searchPlaceholder="Search vendors..."
	/>
</div>

<!-- Vendor Modal -->
<Modal
	isOpen={showVendorModal}
	title={editingItem ? 'Edit Vendor' : 'Add New Vendor'}
	onClose={closeModals}
>
	<form on:submit|preventDefault={saveVendor} class="space-y-4">
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
			<div>
				<label for="vendor_id" class="mb-2 block text-sm font-medium text-gray-300">Vendor ID</label
				>
				<input
					id="vendor_id"
					type="text"
					bind:value={vendorForm.vendor_id}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter vendor ID"
				/>
			</div>
			<div>
				<label for="city" class="mb-2 block text-sm font-medium text-gray-300">City</label>
				<input
					id="city"
					type="text"
					bind:value={vendorForm.city}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter city"
				/>
			</div>
			<div>
				<label for="state" class="mb-2 block text-sm font-medium text-gray-300">State</label>
				<input
					id="state"
					type="text"
					bind:value={vendorForm.state}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter state"
				/>
			</div>
			<div>
				<label for="no_of_batches" class="mb-2 block text-sm font-medium text-gray-300"
					>Number of Batches</label
				>
				<input
					id="no_of_batches"
					type="number"
					bind:value={vendorForm.no_of_batches}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter number of batches"
				/>
			</div>
			<div>
				<label for="gst_no" class="mb-2 block text-sm font-medium text-gray-300">GST Number</label>
				<input
					id="gst_no"
					type="text"
					bind:value={vendorForm.gst_no}
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter GST number"
				/>
			</div>
			<div>
				<label for="pan_number" class="mb-2 block text-sm font-medium text-gray-300">PAN Number</label>
				<input
					id="pan_number"
					type="text"
					bind:value={vendorForm.pan_number}
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter PAN number"
				/>
			</div>
			<div>
				<label for="phone_number" class="mb-2 block text-sm font-medium text-gray-300">Phone Number</label>
				<input
					id="phone_number"
					type="tel"
					bind:value={vendorForm.phone_number}
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter phone number"
				/>
			</div>
			<div>
				<label for="audit_date" class="mb-2 block text-sm font-medium text-gray-300">Audit Date</label>
				<input
					id="audit_date"
					type="date"
					bind:value={vendorForm.audit_date}
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
				/>
			</div>
			<div class="sm:col-span-2">
				<label for="email" class="mb-2 block text-sm font-medium text-gray-300">Email</label>
				<input
					id="email"
					type="email"
					bind:value={vendorForm.email}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter email address"
				/>
			</div>
		</div>

		<div class="flex flex-col justify-end space-y-2 pt-6 sm:flex-row sm:space-y-0 sm:space-x-4">
			<button
				type="button"
				on:click={closeModals}
				class="w-full rounded-lg bg-gray-800 px-4 py-2 text-sm text-white transition-all duration-300 hover:bg-gray-700 sm:w-auto"
			>
				Cancel
			</button>
			<button
				type="submit"
				class="w-full rounded-lg bg-purple-600 px-4 py-2 text-sm text-white transition-all duration-300 hover:bg-purple-700 sm:w-auto"
			>
				{editingItem ? 'Update Vendor' : 'Add Vendor'}
			</button>
		</div>
	</form>
</Modal>
