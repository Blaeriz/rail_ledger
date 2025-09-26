<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { fetchData, createItem, updateItem, deleteItem } from '$lib/utils';
	import { formatDate } from '$lib/types';
	import DataTable from '$lib/components/DataTable.svelte';
	import Modal from '$lib/components/Modal.svelte';

	let user = null;
	let batchData = /** @type {any[]} */ ([]);
	let reportData = /** @type {any[]} */ ([]);
	let isLoading = true;
	let showReportModal = false;
	let editingItem = /** @type {any} */ (null);

	const reportForm = {
		reportId: '',
		batchId: '',
		inspectorName: '',
		status: 1,
		createdAt: ''
	};

	const tableColumns = [
		{ key: 'reportId', label: 'REPORT ID' },
		{ key: 'batchId', label: 'BATCH ID' },
		{ key: 'inspectorName', label: 'INSPECTOR' },
		{
			key: 'status',
			label: 'STATUS',
			render: (/** @type {any} */ item) => {
				const status = item.status === 1 ? 'PASS' : 'FAIL';
				const color =
					item.status === 1
						? 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10'
						: 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10';
				return `<span class="px-3 py-1 rounded-full text-xs font-semibold border-2 ${color}">${status}</span>`;
			}
		},
		{
			key: 'createdAt',
			label: 'CREATED',
			render: (/** @type {any} */ item) => formatDate(item.createdAt)
		},
		{
			key: 'actions',
			label: 'ACTIONS',
			render: (/** @type {any} */ item) => `
        <div class="flex space-x-2">
          <button onclick="editReport('${item.reportId}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20">Edit</button>
          <button onclick="deleteReport('${item.reportId}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-red-500/30 hover:border-red-400/50 hover:shadow-lg hover:shadow-red-500/20">Delete</button>
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
			const [batches, reports] = await Promise.all([
				fetchData('/api/batches'),
				fetchData('/api/reports')
			]);

			batchData = batches;
			reportData = reports;
		} catch (error) {
			console.error('Error loading data:', error);
		} finally {
			isLoading = false;
		}
	});

	function openReportModal(report = null) {
		editingItem = report;
		if (report) {
			Object.assign(reportForm, report);
		} else {
			Object.assign(reportForm, {
				reportId: '',
				batchId: '',
				inspectorName: '',
				status: 1,
				createdAt: new Date().toISOString().split('T')[0]
			});
		}
		showReportModal = true;
	}

	function closeModals() {
		showReportModal = false;
		editingItem = null;
	}

	async function saveReport() {
		try {
			const success = editingItem
				? await updateItem('/api/reports', reportForm.reportId, reportForm)
				: await createItem('/api/reports', reportForm);

			if (success) {
				reportData = await fetchData('/api/reports');
				closeModals();
			}
		} catch (error) {
			console.error('Error saving report:', error);
		}
	}

	async function deleteReport(/** @type {string} */ reportId) {
		if (confirm('Are you sure you want to delete this report?')) {
			try {
				const success = await deleteItem('/api/reports', reportId);
				if (success) {
					reportData = await fetchData('/api/reports');
				}
			} catch (error) {
				console.error('Error deleting report:', error);
			}
		}
	}

	// Make functions globally available for inline onclick handlers
	window.editReport = (reportId) => {
		const report = reportData.find((r) => r.reportId === reportId);
		openReportModal(report);
	};
	window.deleteReport = deleteReport;
</script>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-center">
		<h1 class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">Inspection Reports</h1>
		<button
			on:click={() => openReportModal()}
			class="rounded-lg bg-purple-600 px-4 py-2 text-sm font-medium text-white transition-all duration-300 hover:bg-purple-700 sm:px-6 sm:py-3 sm:text-base"
		>
			+ Add Report
		</button>
	</div>

	<!-- Reports Table -->
	<DataTable
		data={reportData}
		columns={tableColumns}
		searchable={true}
		searchPlaceholder="Search reports..."
	/>
</div>

<!-- Report Modal -->
<Modal
	isOpen={showReportModal}
	title={editingItem ? 'Edit Report' : 'Add New Report'}
	onClose={closeModals}
>
	<form on:submit|preventDefault={saveReport} class="space-y-4">
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
			<div>
				<label for="reportId" class="mb-2 block text-sm font-medium text-gray-300">Report ID</label>
				<input
					id="reportId"
					type="text"
					bind:value={reportForm.reportId}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter report ID"
				/>
			</div>
			<div>
				<label for="batchId" class="mb-2 block text-sm font-medium text-gray-300">Batch ID</label>
				<select
					id="batchId"
					bind:value={reportForm.batchId}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
				>
					<option value="">Select batch</option>
					{#each batchData as batch}
						<option value={batch.batch_id}>{batch.batch_id} - {batch.vendor_id}</option>
					{/each}
				</select>
			</div>
			<div>
				<label for="inspectorName" class="mb-2 block text-sm font-medium text-gray-300"
					>Inspector Name</label
				>
				<input
					id="inspectorName"
					type="text"
					bind:value={reportForm.inspectorName}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-400 focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
					placeholder="Enter inspector name"
				/>
			</div>
			<div>
				<label for="status" class="mb-2 block text-sm font-medium text-gray-300">Status</label>
				<select
					id="status"
					bind:value={reportForm.status}
					required
					class="w-full rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white focus:border-purple-500 focus:ring-1 focus:ring-purple-500 focus:outline-none"
				>
					<option value={1}>Pass</option>
					<option value={0}>Fail</option>
				</select>
			</div>
			<div>
				<label for="createdAt" class="mb-2 block text-sm font-medium text-gray-300"
					>Created Date</label
				>
				<input
					id="createdAt"
					type="date"
					bind:value={reportForm.createdAt}
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
				{editingItem ? 'Update Report' : 'Add Report'}
			</button>
		</div>
	</form>
</Modal>
