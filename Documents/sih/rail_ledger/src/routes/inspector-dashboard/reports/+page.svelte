<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let user = /** @type {any} */ (null);
	let reportData = /** @type {any[]} */ ([]);
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
			const reportsRes = await fetch('/api/reports');
			reportData = await reportsRes.json();
		} catch (error) {
			console.error('Error fetching data:', error);
		}
	}

	function formatDate(/** @type {any} */ dateString) {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString();
	}
</script>

<svelte:head>
	<title>My Inspection Reports - Rail Ledger</title>
</svelte:head>

<div class="space-y-6">
	<h2 class="text-3xl font-bold text-white">My Inspection Reports</h2>

	<div class="rounded-2xl border border-gray-700/40 bg-gray-900/40 p-6 backdrop-blur-xl">
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-gray-700/40">
						<th class="px-4 py-3 text-left text-gray-300">Report ID</th>
						<th class="px-4 py-3 text-left text-gray-300">Batch ID</th>
						<th class="px-4 py-3 text-left text-gray-300">Status</th>
						<th class="px-4 py-3 text-left text-gray-300">Remark</th>
						<th class="px-4 py-3 text-left text-gray-300">Created</th>
						<th class="px-4 py-3 text-left text-gray-300">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each reportData.filter((r) => r.inspectorName === user?.username) as report}
						<tr class="border-b border-gray-700/20 hover:bg-gray-800/30">
							<td class="px-4 py-3 font-mono text-white">{report.reportId}</td>
							<td class="px-4 py-3 text-gray-300">{report.batchId}</td>
							<td class="px-4 py-3">
								<span
									class="rounded-full border-2 px-3 py-1 text-xs font-semibold {report.status === 1
										? 'border-green-600 bg-green-500/5 text-white shadow-lg shadow-green-500/10'
										: 'border-red-600 bg-red-500/5 text-white shadow-lg shadow-red-500/10'}"
								>
									{report.status === 1 ? 'Pass' : 'Fail'}
								</span>
							</td>
							<td class="px-4 py-3 text-gray-300">{report.remark || 'N/A'}</td>
							<td class="px-4 py-3 text-gray-300">{formatDate(report.createdAt)}</td>
							<td class="px-4 py-3">
								<button
									class="rounded border border-purple-500/30 bg-gray-800 px-3 py-1 text-sm text-white transition-all duration-300 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-lg hover:shadow-purple-500/20"
									>View Details</button
								>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>
