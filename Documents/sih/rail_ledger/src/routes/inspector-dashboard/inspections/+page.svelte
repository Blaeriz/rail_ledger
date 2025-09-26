<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let user = /** @type {any} */ (null);
	let vendorData = /** @type {any[]} */ ([]);
	let batchData = /** @type {any[]} */ ([]);
	let reportData = /** @type {any[]} */ ([]);
	let isLoading = true;

	// Inspector-specific statistics
	let inspectorStats = {
		pendingInspections: 0,
		completedToday: 0,
		failedBatches: 0,
		urgentInspections: 0
	};

	onMount(async () => {
		// Check authentication
		const userData = sessionStorage.getItem('user');
		if (!userData) {
			goto('/login');
			return;
		}

		user = JSON.parse(userData);

		// Fetch data
		await fetchAllData();
		calculateInspectorStats();
		isLoading = false;
	});

	async function fetchAllData() {
		try {
			const [vendorsRes, batchesRes, reportsRes] = await Promise.all([
				fetch('/api/vendors'),
				fetch('/api/batches'),
				fetch('/api/reports')
			]);

			vendorData = await vendorsRes.json();
			batchData = await batchesRes.json();
			reportData = await reportsRes.json();
		} catch (error) {
			console.error('Error fetching data:', error);
		}
	}

	function calculateInspectorStats() {
		inspectorStats.pendingInspections = batchData.filter(
			(b) =>
				b.qc_status === 'Pending Inspection' ||
				b.qc_status === 'Pending' ||
				b.qc_status === 'PENDING' ||
				b.qc_status === 'PENDING INSPECTION' ||
				b.status === 'Pending Inspection' ||
				b.status === 'Pending' ||
				b.status === 'PENDING' ||
				b.status === 'PENDING INSPECTION' ||
				b.inspection_status === 'Pending Inspection' ||
				b.inspection_status === 'Pending' ||
				b.inspection_status === 'PENDING' ||
				b.inspection_status === 'PENDING INSPECTION'
		).length;
		inspectorStats.failedBatches = batchData.filter((b) => b.qc_status === 'Fail').length;

		// Count reports created today by this inspector
		const today = new Date().toDateString();
		inspectorStats.completedToday = reportData.filter(
			(r) => r.inspectorName === user.username && new Date(r.createdAt).toDateString() === today
		).length;

		// Urgent inspections (batches older than 30 days without inspection)
		const thirtyDaysAgo = new Date();
		thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
		inspectorStats.urgentInspections = batchData.filter(
			(b) => !b.last_inspection_date && new Date(b.date_of_production) < thirtyDaysAgo
		).length;
	}
</script>

<svelte:head>
	<title>Inspection Overview - Rail Ledger</title>
</svelte:head>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 sm:mb-8">
		<h1 class="mb-2 text-2xl font-bold text-white sm:text-3xl lg:text-4xl">Inspection Overview</h1>
	</div>

	<!-- Inspector Stats Cards -->
	<div class="mb-6 grid grid-cols-1 gap-4 sm:mb-8 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4">
		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<div>
				<p class="mb-2 text-xs font-medium text-gray-400 sm:text-sm">PENDING INSPECTIONS</p>
				<p class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">
					{inspectorStats.pendingInspections}
				</p>
			</div>
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<div>
				<p class="mb-2 text-xs font-medium text-gray-400 sm:text-sm">COMPLETED TODAY</p>
				<p class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">
					{inspectorStats.completedToday}
				</p>
			</div>
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<div>
				<p class="mb-2 text-xs font-medium text-gray-400 sm:text-sm">FAILED BATCHES</p>
				<p class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">
					{inspectorStats.failedBatches}
				</p>
			</div>
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<div>
				<p class="mb-2 text-xs font-medium text-gray-400 sm:text-sm">URGENT INSPECTIONS</p>
				<p class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">
					{inspectorStats.urgentInspections}
				</p>
			</div>
		</div>
	</div>

	<!-- Quick Actions -->
	<div class="mb-6 sm:mb-8">
		<h2 class="mb-4 text-xl font-bold text-white sm:mb-6 sm:text-2xl">Quick Actions</h2>
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-6 lg:grid-cols-3">
			<button
				class="group rounded-lg border border-gray-700 bg-gray-800 p-4 text-left transition-all duration-300 hover:bg-gray-700 sm:p-6"
			>
				<div>
					<p class="mb-1 text-sm font-bold text-white sm:text-base">Start Inspection</p>
					<p class="text-xs text-gray-400 sm:text-sm">Begin new inspection</p>
				</div>
			</button>

			<button
				class="group rounded-lg border border-gray-700 bg-gray-800 p-4 text-left transition-all duration-300 hover:bg-gray-700 sm:p-6"
			>
				<div>
					<p class="mb-1 text-sm font-bold text-white sm:text-base">Generate Report</p>
					<p class="text-xs text-gray-400 sm:text-sm">Create inspection report</p>
				</div>
			</button>

			<button
				class="group rounded-lg border border-gray-700 bg-gray-800 p-4 text-left transition-all duration-300 hover:bg-gray-700 sm:p-6"
			>
				<div>
					<p class="mb-1 text-sm font-bold text-white sm:text-base">View Analytics</p>
					<p class="text-xs text-gray-400 sm:text-sm">Quality insights</p>
				</div>
			</button>
		</div>
	</div>
</div>
