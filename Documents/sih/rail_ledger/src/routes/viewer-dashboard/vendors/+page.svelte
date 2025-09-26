<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	let user = null;
	let vendorData = [];
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
			const vendorsRes = await fetch('/api/vendors');
			vendorData = await vendorsRes.json();
		} catch (error) {
			console.error('Error fetching data:', error);
		}
	}
</script>

<svelte:head>
	<title>Vendor Information - Rail Ledger</title>
</svelte:head>

<div class="space-y-6">
	<h2 class="text-3xl font-bold text-white">Vendor Information</h2>

	<div class="rounded-2xl border border-gray-700/40 bg-gray-900/40 p-6 backdrop-blur-xl">
		<div class="overflow-x-auto">
			<table class="w-full">
				<thead>
					<tr class="border-b border-gray-700/40">
						<th class="px-4 py-3 text-left text-gray-300">Vendor ID</th>
						<th class="px-4 py-3 text-left text-gray-300">City</th>
						<th class="px-4 py-3 text-left text-gray-300">State</th>
						<th class="px-4 py-3 text-left text-gray-300">Batches</th>
						<th class="px-4 py-3 text-left text-gray-300">Email</th>
						<th class="px-4 py-3 text-left text-gray-300">Phone</th>
					</tr>
				</thead>
				<tbody>
					{#each vendorData as vendor}
						<tr class="border-b border-gray-700/20 hover:bg-gray-800/30">
							<td class="px-4 py-3 font-mono text-white">{vendor.vendor_id}</td>
							<td class="px-4 py-3 text-gray-300">{vendor.city}</td>
							<td class="px-4 py-3 text-gray-300">{vendor.state}</td>
							<td class="px-4 py-3 text-gray-300">{vendor.no_of_batches}</td>
							<td class="px-4 py-3 text-gray-300">{vendor.email}</td>
							<td class="px-4 py-3 text-gray-300">{vendor.phone_number}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>
