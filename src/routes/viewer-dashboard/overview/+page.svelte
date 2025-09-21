<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let user = /** @type {any} */ (null);
	let vendorData = /** @type {any[]} */ ([]);
	let batchData = /** @type {any[]} */ ([]);
	let reportData = /** @type {any[]} */ ([]);
	let isLoading = true;

	// Viewer-specific statistics
	let viewerStats = /** @type {any} */ ({
		totalVendors: 0,
		totalBatches: 0,
		passedBatches: 0,
		failedBatches: 0,
		pendingInspections: 0,
		recentActivity: /** @type {any[]} */ ([])
	});

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
		calculateViewerStats();
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

	function calculateViewerStats() {
		viewerStats.totalVendors = vendorData.length;
		viewerStats.totalBatches = batchData.length;
		viewerStats.passedBatches = batchData.filter((b) => b.qc_status === 'Pass').length;
		viewerStats.failedBatches = batchData.filter((b) => b.qc_status === 'Fail').length;
		viewerStats.pendingInspections = batchData.filter(
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

		// Recent activity (last 10 reports)
		viewerStats.recentActivity = reportData
			.sort((/** @type {any} */ a, /** @type {any} */ b) => {
				const dateA = new Date(a.createdAt).getTime();
				const dateB = new Date(b.createdAt).getTime();
				return dateB - dateA;
			})
			.slice(0, 10);
	}

	function formatDate(/** @type {any} */ dateString) {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString();
	}

	function getPassRate() {
		if (viewerStats.totalBatches === 0) return 0;
		return Math.round((viewerStats.passedBatches / viewerStats.totalBatches) * 100);
	}
</script>

<svelte:head>
	<title>System Overview - Rail Ledger</title>
</svelte:head>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 sm:mb-8">
		<h1 class="mb-2 text-2xl font-bold text-white sm:text-3xl lg:text-4xl">System Overview</h1>
	</div>

	<!-- Summary Cards -->
	<div class="mb-6 grid grid-cols-1 gap-4 sm:mb-8 sm:grid-cols-2 sm:gap-6 lg:grid-cols-4">
		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<div>
				<p class="mb-2 text-xs font-medium text-gray-400 sm:text-sm">TOTAL VENDORS</p>
				<p class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">
					{viewerStats.totalVendors}
				</p>
			</div>
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<div>
				<p class="mb-2 text-xs font-medium text-gray-400 sm:text-sm">TOTAL BATCHES</p>
				<p class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">
					{viewerStats.totalBatches}
				</p>
			</div>
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<div>
				<p class="mb-2 text-xs font-medium text-gray-400 sm:text-sm">PASSED BATCHES</p>
				<p class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">
					{viewerStats.passedBatches}
				</p>
			</div>
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<div>
				<p class="mb-2 text-xs font-medium text-gray-400 sm:text-sm">FAILED BATCHES</p>
				<p class="text-2xl font-bold text-white sm:text-3xl lg:text-4xl">
					{viewerStats.failedBatches}
				</p>
			</div>
		</div>
	</div>

	<!-- Quality Metrics -->
	<div class="mb-6 grid grid-cols-1 gap-4 sm:mb-8 sm:gap-6 lg:grid-cols-2">
		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<h3 class="mb-4 text-lg font-bold text-white sm:text-xl">Quality Overview</h3>
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<span class="text-sm text-gray-300">Overall Pass Rate</span>
					<span class="text-2xl font-bold text-green-400">{getPassRate()}%</span>
				</div>
				<div class="h-3 w-full rounded-full bg-gray-700">
					<div
						class="h-3 rounded-full bg-gradient-to-r from-green-500 to-emerald-500"
						style="width: {getPassRate()}%"
					></div>
				</div>
				<div class="mt-4 grid grid-cols-2 gap-4">
					<div class="text-center">
						<p class="text-2xl font-bold text-blue-400">{viewerStats.pendingInspections}</p>
						<p class="text-sm text-gray-400">Pending</p>
					</div>
					<div class="text-center">
						<p class="text-2xl font-bold text-yellow-400">{viewerStats.totalVendors}</p>
						<p class="text-sm text-gray-400">Active Vendors</p>
					</div>
				</div>
			</div>
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4 sm:p-6">
			<h3 class="mb-4 text-lg font-bold text-white sm:text-xl">Recent Activity</h3>
			<div class="max-h-64 space-y-3 overflow-y-auto">
				{#each viewerStats.recentActivity as activity}
					<div class="flex items-center space-x-3 rounded-lg bg-gray-700 p-3">
						<div class="flex h-8 w-8 items-center justify-center rounded-full bg-blue-600/20">
							<svg
								class="h-4 w-4 text-blue-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"
								></path>
							</svg>
						</div>
						<div class="flex-1">
							<p class="text-sm text-white">Report #{activity.reportId}</p>
							<p class="text-xs text-gray-400">
								Batch {activity.batchId} • {formatDate(activity.createdAt)}
							</p>
						</div>
						<span
							class="rounded-full border-2 px-3 py-1 text-xs font-semibold {activity.status === 1
								? 'border-green-600 bg-green-500/5 text-white shadow-lg shadow-green-500/10'
								: 'border-red-600 bg-red-500/5 text-white shadow-lg shadow-red-500/10'}"
						>
							{activity.status === 1 ? 'Pass' : 'Fail'}
						</span>
					</div>
				{/each}
			</div>
		</div>
	</div>
</div>
