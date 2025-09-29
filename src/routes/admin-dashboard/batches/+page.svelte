<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { fetchData, createItem, updateItem, deleteItem } from '$lib/utils';
	import { formatDate, getStatusColor } from '$lib/types';
	import DataTable from '$lib/components/DataTable.svelte';
	import Modal from '$lib/components/Modal.svelte';

	let user = null;
	let vendorData = /** @type {any[]} */ ([]);
	let batchData = /** @type {any[]} */ ([]);
	let isLoading = true;
	let showBatchModal = false;
	let editingItem = /** @type {any} */ (null);

	const batchForm = {
		batch_id: '',
		vendor_id: '',
		batch_size: '',
		qc_status: 'Pending',
		date_of_production: '',
		expiry_date: ''
	};

	const tableColumns = [
		{ key: 'batch_id', label: 'BATCH ID' },
		{ key: 'vendor_id', label: 'VENDOR ID' },
		{ key: 'batch_size', label: 'SIZE' },
		{
			key: 'qc_status',
			label: 'STATUS',
			render: (/** @type {any} */ item) => {
				const status =
					item.qc_status === 'Pass'
						? 'PASS'
						: item.qc_status === 'Fail'
							? 'FAIL'
							: 'PENDING INSPECTION';
				const color = getStatusColor(item.qc_status);
				return `<span class="px-2 py-1 rounded-full text-xs font-medium ${color} text-white">${status}</span>`;
			}
		},
		{
			key: 'date_of_production',
			label: 'PRODUCTION DATE',
			render: (/** @type {any} */ item) => formatDate(item.date_of_production)
		},
		{
			key: 'actions',
			label: 'ACTIONS',
			render: (/** @type {any} */ item) => `
        <div class="flex space-x-2">
          <button onclick="editBatch('${item.batch_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20">Edit</button>
          <button onclick="deleteBatch('${item.batch_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-red-500/30 hover:border-red-400/50 hover:shadow-lg hover:shadow-red-500/20">Delete</button>
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
			const [vendors, batches] = await Promise.all([
				fetchData('/api/vendors'),
				fetchData('/api/batches')
			]);

			vendorData = vendors;
			batchData = batches;
		} catch (error) {
			console.error('Error loading data:', error);
		} finally {
			isLoading = false;
		}
	});

	function openBatchModal(batch = null) {
		editingItem = batch;
		if (batch) {
			Object.assign(batchForm, batch);
		} else {
			Object.assign(batchForm, {
				batch_id: '',
				vendor_id: '',
				batch_size: '',
				qc_status: 'Pending',
				date_of_production: '',
				expiry_date: ''
			});
		}
		showBatchModal = true;
	}

	function closeModals() {
		showBatchModal = false;
		editingItem = null;
	}

	async function saveBatch() {
		try {
			const success = editingItem
				? await updateItem('/api/batches', batchForm.batch_id, batchForm)
				: await createItem('/api/batches', batchForm);

			if (success) {
				// Refresh data
				batchData = await fetchData('/api/batches');
				closeModals();
			}
		} catch (error) {
			console.error('Error saving batch:', error);
		}
	}

	async function deleteBatch(/** @type {string} */ batchId) {
		if (confirm('Are you sure you want to delete this batch?')) {
			try {
				const success = await deleteItem('/api/batches', batchId);
				if (success) {
					batchData = await fetchData('/api/batches');
				}
			} catch (error) {
				console.error('Error deleting batch:', error);
			}
		}
	}

	// Make functions globally available for inline onclick handlers
	window.editBatch = (batchId) => {
		const batch = batchData.find((b) => b.batch_id === batchId);
		openBatchModal(batch);
	};
	window.deleteBatch = deleteBatch;
</script>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-center">
		<h1 class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">Batch Management</h1>
	</div>

	<!-- Batch Table -->
	<DataTable
		data={batchData}
		columns={tableColumns}
		searchable={true}
		searchPlaceholder="Search batches..."
		showPagination={true}
		pageSize={20}
	/>
</div>

<!-- Batch Modal -->
<Modal
	isOpen={showBatchModal}
	title={editingItem ? 'Edit Batch' : 'Add New Batch'}
	onClose={closeModals}
>
	<form on:submit|preventDefault={saveBatch} class="space-y-4">
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
			<div>
				<label for="batch_id" class="mb-2 block text-sm font-medium text-gray-300">Batch ID</label>
				<input
					id="batch_id"
					type="text"
					bind:value={batchForm.batch_id}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter batch ID"
				/>
			</div>
			<div>
				<label for="vendor_id" class="mb-2 block text-sm font-medium text-gray-300">Vendor ID</label
				>
				<select
					id="vendor_id"
					bind:value={batchForm.vendor_id}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
				>
					<option value="">Select vendor</option>
					{#each vendorData as vendor}
						<option value={vendor.vendor_id}>{vendor.vendor_id} - {vendor.city}</option>
					{/each}
				</select>
			</div>
			<div>
				<label for="batch_size" class="mb-2 block text-sm font-medium text-gray-300"
					>Batch Size</label
				>
				<input
					id="batch_size"
					type="number"
					bind:value={batchForm.batch_size}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter batch size"
				/>
			</div>
			<div>
				<label for="qc_status" class="mb-2 block text-sm font-medium text-gray-300">QC Status</label
				>
				<select
					id="qc_status"
					bind:value={batchForm.qc_status}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
				>
					<option value="Pending">Pending</option>
					<option value="Pass">Pass</option>
					<option value="Fail">Fail</option>
				</select>
			</div>
			<div>
				<label for="date_of_production" class="mb-2 block text-sm font-medium text-gray-300"
					>Production Date</label
				>
				<input
					id="date_of_production"
					type="date"
					bind:value={batchForm.date_of_production}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
				/>
			</div>
			<div>
				<label for="expiry_date" class="mb-2 block text-sm font-medium text-gray-300"
					>Expiry Date</label
				>
				<input
					id="expiry_date"
					type="date"
					bind:value={batchForm.expiry_date}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
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
				{editingItem ? 'Update Batch' : 'Add Batch'}
			</button>
		</div>
	</form>
</Modal>
