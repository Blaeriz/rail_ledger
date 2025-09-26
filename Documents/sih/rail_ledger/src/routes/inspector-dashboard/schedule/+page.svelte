<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let user = /** @type {any} */ (null);
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
			batchData = (await batchesRes.json()) || [];
		} catch (error) {
			console.error('Error fetching data:', error);
			batchData = [];
		}
	}

	// Get today's inspections (batches without inspection date)
	$: todaysInspections = batchData.filter((b) => !b.last_inspection_date).slice(0, 5);

	// Get upcoming inspections (remaining batches)
	$: upcomingInspections = batchData.filter((b) => !b.last_inspection_date).slice(5, 10);
</script>

<svelte:head>
	<title>Inspection Schedule - Rail Ledger</title>
</svelte:head>

<div class="p-6">
	<div class="mb-8">
		<h1 class="mb-2 text-3xl font-bold text-white">Inspection Schedule</h1>
		<p class="text-gray-400">Manage your inspection tasks and schedule</p>
	</div>

	{#if isLoading}
		<div class="flex items-center justify-center py-12">
			<div class="text-center">
				<div
					class="mx-auto mb-4 h-12 w-12 animate-spin rounded-full border-b-2 border-purple-500"
				></div>
				<p class="text-gray-400">Loading schedule...</p>
			</div>
		</div>
	{:else}
		<div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
			<!-- Today's Inspections -->
			<div
				class="rounded-lg border border-gray-700 bg-black p-6 transition-all duration-300 hover:border-gray-500/50"
			>
				<h3 class="mb-6 text-xl font-bold text-white">Today's Inspections</h3>
				<div class="space-y-4">
					{#if todaysInspections.length > 0}
						{#each todaysInspections as batch}
							<div
								class="flex items-center justify-between rounded-lg border border-gray-600/30 bg-gray-800/30 p-4 transition-all duration-200 hover:border-gray-500/50"
							>
								<div class="flex-1">
									<p class="text-lg font-semibold text-white">
										{batch.batch_id ||
											'BATCH' + Math.random().toString(36).substr(2, 6).toUpperCase()}
									</p>
									<p class="mt-1 text-sm text-gray-400">
										{batch.vendor_id ||
											'VEND' + Math.random().toString(36).substr(2, 6).toUpperCase()}
									</p>
								</div>
								<button
									class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-all duration-200 hover:bg-blue-700 hover:shadow-lg hover:shadow-blue-500/25"
								>
									Start
								</button>
							</div>
						{/each}
					{:else}
						<div class="py-8 text-center">
							<div
								class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-700/30"
							>
								<svg
									class="h-8 w-8 text-gray-500"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
									></path>
								</svg>
							</div>
							<p class="text-gray-400">No inspections scheduled for today</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Upcoming Inspections -->
			<div
				class="rounded-lg border border-gray-700 bg-black p-6 transition-all duration-300 hover:border-gray-500/50"
			>
				<h3 class="mb-6 text-xl font-bold text-white">Upcoming Inspections</h3>
				<div class="space-y-4">
					{#if upcomingInspections.length > 0}
						{#each upcomingInspections as batch}
							<div
								class="flex items-center justify-between rounded-lg border border-gray-600/30 bg-gray-800/30 p-4 transition-all duration-200 hover:border-gray-500/50"
							>
								<div class="flex-1">
									<p class="text-lg font-semibold text-white">
										{batch.batch_id ||
											'BATCH' + Math.random().toString(36).substr(2, 6).toUpperCase()}
									</p>
									<p class="mt-1 text-sm text-gray-400">
										{batch.vendor_id ||
											'VEND' + Math.random().toString(36).substr(2, 6).toUpperCase()}
									</p>
								</div>
								<span class="rounded-lg bg-gray-700/30 px-3 py-1 text-sm font-medium text-gray-400">
									Scheduled
								</span>
							</div>
						{/each}
					{:else}
						<div class="py-8 text-center">
							<div
								class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gray-700/30"
							>
								<svg
									class="h-8 w-8 text-gray-500"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
									></path>
								</svg>
							</div>
							<p class="text-gray-400">No upcoming inspections</p>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</div>
