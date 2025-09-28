<!-- @ts-nocheck -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import IndiaMap from '$lib/components/IndiaMap.svelte';

	let heatmapData: { id: string; state: string; pendingInspections: number }[] = [];
	let isLoading = true;
	let error: string | null = null;
	let tooltipContent = '';
	let tooltipVisible = false;
	let tooltipX = 0;
	let tooltipY = 0;

	// Sample data for Indian states with pending inspections
	const sampleData = [];

	onMount(async () => {
		try {
			const response = await fetch('/api/heatmap/pending-inspections', { cache: 'no-store' });
			if (!response.ok) throw new Error('Failed to fetch');
			const result = await response.json();
			heatmapData = result.data || [];
		} catch (err) {
			console.error('Error fetching data:', err);
			heatmapData = [];
			error = 'Failed to load heatmap data from API';
		} finally {
			isLoading = false;
		}
	});

	// Color scale for heatmap intensity
	function getColor(intensity: number) {
		if (intensity >= 40) return '#ef4444'; // Red for high
		if (intensity >= 30) return '#f97316'; // Orange
		if (intensity >= 20) return '#eab308'; // Yellow
		if (intensity >= 10) return '#22c55e'; // Green
		return '#6b7280'; // Gray for low
	}

	// Event handlers for map interactions
	function handleMapMouseEnter(event: any) {
		tooltipContent = `${event.detail.state}: ${event.detail.pendingInspections} pending inspections`;
		tooltipVisible = true;
		tooltipX = event.detail.x;
		tooltipY = event.detail.y;
	}

	function handleMapMouseMove(event: any) {
		tooltipX = event.detail.x;
		tooltipY = event.detail.y;
	}

	function handleMapMouseLeave() {
		tooltipVisible = false;
	}
</script>

<svelte:head>
	<title>Heatmap - Pending Inspections | Rail Ledger</title>
</svelte:head>

<div class="min-h-screen bg-black text-white p-4">
	<div class="container mx-auto py-8">
		<h1 class="text-4xl font-bold mb-6 text-center">Pending Inspections Heatmap</h1>

		{#if isLoading}
			<div class="flex h-96 items-center justify-center">
				<div class="text-center">
					<div class="mx-auto h-12 w-12 animate-spin rounded-full border-b-2 border-purple-500"></div>
					<p class="mt-4 text-gray-400">Loading heatmap data...</p>
				</div>
			</div>
		{:else if error}
			<div class="mb-4 rounded-lg border border-yellow-500/30 bg-yellow-500/10 px-4 py-3">
				<p class="text-yellow-400">{error}</p>
			</div>
		{/if}

		<!-- Heatmap Container -->
		<div class="relative">
			<div class="rounded-lg border border-gray-800 bg-gray-900/50 p-4">
				<!-- India Map Component -->
				<div class="flex justify-center p-6">
					<div class="w-full max-w-4xl h-[500px]">
						<IndiaMap 
							data={heatmapData}
							width={700}
							height={450}
							on:mouseenter={handleMapMouseEnter}
							on:mousemove={handleMapMouseMove}
							on:mouseleave={handleMapMouseLeave}
						/>
					</div>
				</div>
				
				<!-- Map title -->
				<div class="mt-4 text-center">
					<h2 class="text-2xl font-bold text-white">
						India - Pending Inspections Heatmap
					</h2>
				</div>
			</div>
		</div>

		<!-- Data Table -->
		<div class="mt-6">
			<h3 class="mb-4 text-xl font-semibold text-white">State-wise Pending Inspections</h3>
			<div class="overflow-x-auto">
				<table class="w-full border-collapse border border-gray-700">
					<thead>
						<tr class="bg-gray-800">
							<th class="px-4 py-3 text-left text-sm font-semibold text-gray-300">State</th>
							<th class="px-4 py-3 text-left text-sm font-semibold text-gray-300">Pending Inspections</th>
							<th class="px-4 py-3 text-left text-sm font-semibold text-gray-300">Priority Level</th>
							<th class="px-4 py-3 text-left text-sm font-semibold text-gray-300">Status</th>
						</tr>
					</thead>
					<tbody>
						{#each heatmapData.sort((a, b) => b.pendingInspections - a.pendingInspections) as state}
							<tr class="border-t border-gray-700 hover:bg-gray-800/50">
								<td class="px-4 py-3 text-sm text-white">{state.state}</td>
								<td class="px-4 py-3 text-sm text-white">{state.pendingInspections}</td>
								<td class="px-4 py-3 text-sm">
									<span
										class="rounded-full px-2 py-1 text-xs font-medium"
										style="background-color: {getColor(state.pendingInspections)}20; color: {getColor(state.pendingInspections)};"
									>
										{#if state.pendingInspections >= 40}
											Critical
										{:else if state.pendingInspections >= 30}
											Very High
										{:else if state.pendingInspections >= 20}
											High
										{:else if state.pendingInspections >= 10}
											Medium
										{:else}
											Low
										{/if}
									</span>
								</td>
								<td class="px-4 py-3 text-sm">
									<span class="text-yellow-400">Pending</span>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</div>

	<!-- Tooltip -->
	{#if tooltipVisible && browser}
		<div
			class="fixed z-50 rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white shadow-lg"
			style="left: {tooltipX + 10}px; top: {tooltipY - 10}px; pointer-events: none;"
		>
			{tooltipContent}
		</div>
	{/if}
</div>
