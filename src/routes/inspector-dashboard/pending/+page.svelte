<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let user = null;
	let batchData = /** @type {any[]} */ ([]);
	let isLoading = true;

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
		isLoading = false;
	});

	async function fetchAllData() {
		try {
			const batchesRes = await fetch('/api/batches');
			batchData = await batchesRes.json();
		} catch (error) {
			console.error('Error fetching data:', error);
		}
	}

	function getDaysSinceProduction(/** @type {any} */ productionDate) {
		if (!productionDate) return 'N/A';
		const days = Math.floor(
			(new Date().getTime() - new Date(productionDate).getTime()) / (1000 * 60 * 60 * 24)
		);
		return days;
	}

	function getPriority(/** @type {any} */ daysSinceProduction) {
		if (daysSinceProduction > 30) return { level: 'HIGH', color: 'text-red-400 bg-red-900/30' };
		if (daysSinceProduction > 15)
			return { level: 'MEDIUM', color: 'text-yellow-400 bg-yellow-900/30' };
		return { level: 'LOW', color: 'text-green-400 bg-green-900/30' };
	}
</script>

<svelte:head>
	<title>Pending Inspections - Rail Ledger</title>
</svelte:head>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 sm:mb-8">
		<h1 class="mb-2 text-2xl font-bold text-white sm:text-3xl lg:text-4xl">Pending Inspections</h1>
	</div>

	<!-- Pending Table -->
	<div class="overflow-hidden rounded-lg border border-gray-700 bg-gray-800">
		<div class="overflow-x-auto">
			<table class="w-full min-w-[600px]">
				<thead>
					<tr class="border-b border-gray-700">
						<th class="px-4 py-3 text-left text-xs font-bold text-white sm:px-6 sm:py-4 sm:text-sm"
							>BATCH ID</th
						>
						<th class="px-4 py-3 text-left text-xs font-bold text-white sm:px-6 sm:py-4 sm:text-sm"
							>VENDOR ID</th
						>
						<th class="px-4 py-3 text-left text-xs font-bold text-white sm:px-6 sm:py-4 sm:text-sm"
							>DAYS SINCE PRODUCTION</th
						>
						<th class="px-4 py-3 text-left text-xs font-bold text-white sm:px-6 sm:py-4 sm:text-sm"
							>PRIORITY</th
						>
						<th class="px-4 py-3 text-left text-xs font-bold text-white sm:px-6 sm:py-4 sm:text-sm"
							>LOCATION</th
						>
						<th class="px-4 py-3 text-left text-xs font-bold text-white sm:px-6 sm:py-4 sm:text-sm"
							>ACTIONS</th
						>
					</tr>
				</thead>
				<tbody>
					{#each batchData.filter((b) => !b.last_inspection_date) as batch}
						{@const daysSince = getDaysSinceProduction(batch.date_of_production)}
						{@const priority = getPriority(daysSince)}
						<tr class="border-b border-gray-700 hover:bg-gray-700/50">
							<td class="px-4 py-3 font-mono text-xs text-gray-300 sm:px-6 sm:py-4 sm:text-sm"
								>{batch.batch_id}</td
							>
							<td class="px-4 py-3 text-xs text-gray-300 sm:px-6 sm:py-4 sm:text-sm"
								>{batch.vendor_id}</td
							>
							<td class="px-4 py-3 text-xs text-gray-300 sm:px-6 sm:py-4 sm:text-sm"
								>{daysSince} days</td
							>
							<td class="px-4 py-3 sm:px-6 sm:py-4">
								<span
									class="rounded-full border-2 px-3 py-1 text-xs font-semibold {priority.level ===
									'HIGH'
										? 'border-red-600 bg-red-500/5 text-white shadow-lg shadow-red-500/10'
										: priority.level === 'MEDIUM'
											? 'border-yellow-600 bg-yellow-500/5 text-white shadow-lg shadow-yellow-500/10'
											: 'border-green-600 bg-green-500/5 text-white shadow-lg shadow-green-500/10'}"
								>
									{priority.level}
								</span>
							</td>
							<td class="px-4 py-3 text-xs text-gray-300 sm:px-6 sm:py-4 sm:text-sm"
								>{batch.fitment_location || 'N/A'}</td
							>
							<td class="px-4 py-3 sm:px-6 sm:py-4">
								<button
									class="rounded-lg border border-purple-500/30 bg-gray-800 px-4 py-2 text-xs text-white transition-all duration-300 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/30 sm:text-sm"
								>
									Inspect
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>
