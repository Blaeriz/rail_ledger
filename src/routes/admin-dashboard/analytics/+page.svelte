<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { fetchData, calculateSummaryStats } from '$lib/utils';
	import Chart from '$lib/components/Chart.svelte';
	import IndiaMap from '$lib/components/IndiaMap.svelte';

	let user = /** @type {any} */ (null);
	let vendorData = /** @type {any[]} */ ([]);
	let batchData = /** @type {any[]} */ ([]);
	let reportData = /** @type {any[]} */ ([]);
	let summaryStats = { totalVendors: 0, totalBatches: 0, pendingInspections: 0, failedBatches: 0 };
	let isLoading = true;

	// Heatmap data
	let heatmapData = /** @type {any[]} */ ([]);
	let tooltipContent = '';
	let tooltipVisible = false;
	let tooltipX = 0;
	let tooltipY = 0;

	// Chart data
	let qualityChartData = {
		labels: ['Pass', 'Fail', 'Pending'],
		datasets: [
			{
				data: [0, 0, 0],
				backgroundColor: ['#10B981', '#EF4444', '#F59E0B'],
				borderColor: ['#059669', '#DC2626', '#D97706'],
				borderWidth: 2
			}
		]
	};

	let vendorChartData = {
		labels: /** @type {string[]} */ ([]),
		datasets: [
			{
				label: 'Batches',
				data: /** @type {number[]} */ ([]),
				backgroundColor: 'rgba(139, 92, 246, 0.8)',
				borderColor: '#8B5CF6',
				borderWidth: 2,
				borderRadius: 4
			}
		]
	};

	let monthlyTrendsData = {
		labels: /** @type {string[]} */ ([]),
		datasets: [
			{
				label: 'Passed',
				data: /** @type {number[]} */ ([]),
				borderColor: '#10B981',
				backgroundColor: 'rgba(16, 185, 129, 0.1)',
				borderWidth: 3,
				fill: true,
				tension: 0.4
			},
			{
				label: 'Failed',
				data: /** @type {number[]} */ ([]),
				borderColor: '#EF4444',
				backgroundColor: 'rgba(239, 68, 68, 0.1)',
				borderWidth: 3,
				fill: true,
				tension: 0.4
			}
		]
	};

	onMount(async () => {
		const userData = sessionStorage.getItem('user');
		if (!userData) return goto('/login');

		user = JSON.parse(userData);

		try {
			const [vendors, batches, reports, heatmap] = await Promise.all([
				fetchData('/api/vendors'),
				fetchData('/api/batches'),
				fetchData('/api/reports'),
				fetchData('/api/heatmap/pending-inspections')
			]);
			vendorData = vendors || [];
			batchData = batches || [];
			reportData = reports || [];
			heatmapData = (heatmap && heatmap.data) ? heatmap.data : [];

			// Calculate summary stats with proper data handling
			summaryStats = {
				totalVendors: vendorData.length,
				totalBatches: batchData.length,
				pendingInspections: batchData.filter(
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
				).length,
				failedBatches: batchData.filter(
					(b) =>
						b.qc_status === 'Fail' ||
						b.qc_status === 'FAIL' ||
						b.status === 'Fail' ||
						b.status === 'FAIL' ||
						b.inspection_status === 'Fail' ||
						b.inspection_status === 'FAIL'
				).length
			};

			updateChartData();
			isLoading = false;
		} catch (error) {
			console.error('Error loading data:', error);
			// Set empty data if APIs fail
			vendorData = [];
			batchData = [];
			reportData = [];
			summaryStats = {
				totalVendors: 0,
				totalBatches: 0,
				pendingInspections: 0,
				failedBatches: 0
			};
			isLoading = false;
		}
	});

	function updateChartData() {
		// Quality chart - handle different status formats with enhanced colors
		const passCount = batchData.filter(
			(b) =>
				b.qc_status === 'Pass' ||
				b.qc_status === 'PASS' ||
				b.status === 'Pass' ||
				b.status === 'PASS' ||
				b.inspection_status === 'Pass' ||
				b.inspection_status === 'PASS'
		).length;
		const failCount = batchData.filter(
			(b) =>
				b.qc_status === 'Fail' ||
				b.qc_status === 'FAIL' ||
				b.status === 'Fail' ||
				b.status === 'FAIL' ||
				b.inspection_status === 'Fail' ||
				b.inspection_status === 'FAIL'
		).length;
		const pendingCount = batchData.filter(
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

		// Debug logging for quality chart data
		console.log('Quality Chart Data:', {
			passCount,
			failCount,
			pendingCount,
			totalBatches: batchData.length
		});
		console.log(
			'Sample batch data:',
			batchData.slice(0, 3).map((b) => ({
				batch_id: b.batch_id,
				qc_status: b.qc_status,
				status: b.status,
				inspection_status: b.inspection_status
			}))
		);

		// Ensure we have at least some data for the chart
		const total = passCount + failCount + pendingCount;
		if (total === 0) {
			qualityChartData = {
				labels: ['No Data'],
				datasets: [
					{
						data: [1],
						backgroundColor: ['#6B7280'],
						borderColor: ['#4B5563'],
						borderWidth: 3
					}
				]
			};
		} else {
			qualityChartData = {
				labels: ['Pass', 'Fail', 'Pending'],
				datasets: [
					{
						data: [passCount, failCount, pendingCount],
						backgroundColor: ['#10B981', '#EF4444', '#F59E0B'],
						borderColor: ['#059669', '#DC2626', '#D97706'],
						borderWidth: 3
					}
				]
			};
		}

		// Vendor chart - top 8 vendors with enhanced colors and real data
		const vendorCounts = vendorData
			.map((v) => ({
				name: v.city || v.vendor_name || 'Unknown',
				count: batchData.filter((b) => b.vendor_id === v.vendor_id).length,
				totalBatches: v.no_of_batches || 0
			}))
			.sort((a, b) => b.count - a.count)
			.slice(0, 8);

		if (vendorCounts.length === 0 || vendorCounts.every((v) => v.count === 0)) {
			vendorChartData = {
				labels: ['No Data'],
				datasets: [
					{
						label: 'Batches',
						data: [1],
						backgroundColor: 'rgba(107, 114, 128, 0.8)',
						borderColor: '#6B7280',
						borderWidth: 3,
						borderRadius: 8
					}
				]
			};
		} else {
			const barColors = vendorCounts.map((_, index) => {
				const colorPalette = [
					'rgba(139, 92, 246, 0.8)',
					'rgba(59, 130, 246, 0.8)',
					'rgba(16, 185, 129, 0.8)',
					'rgba(245, 158, 11, 0.8)',
					'rgba(239, 68, 68, 0.8)',
					'rgba(168, 85, 247, 0.8)',
					'rgba(34, 197, 94, 0.8)',
					'rgba(249, 115, 22, 0.8)'
				];
				return colorPalette[index % colorPalette.length];
			});

			const barBorderColors = vendorCounts.map((_, index) => {
				const borderPalette = [
					'#8B5CF6',
					'#3B82F6',
					'#10B981',
					'#F59E0B',
					'#EF4444',
					'#A855F7',
					'#22C55E',
					'#F97316'
				];
				return borderPalette[index % borderPalette.length];
			});

			vendorChartData = {
				labels: /** @type {string[]} */ (vendorCounts.map((v) => v.name)),
				datasets: [
					{
						label: 'Batches',
						data: /** @type {number[]} */ (vendorCounts.map((v) => v.count)),
						backgroundColor: /** @type {any} */ (barColors),
						borderColor: /** @type {any} */ (barBorderColors),
						borderWidth: 3,
						borderRadius: 8
					}
				]
			};
		}

		// Monthly trends data - only real data
		const monthlyData = calculateMonthlyTrends();
		if (
			monthlyData.passedData.every((v) => v === 0) &&
			monthlyData.failedData.every((v) => v === 0)
		) {
			monthlyTrendsData = {
				labels: ['No Data'],
				datasets: [
					{
						label: 'No Data',
						data: [1],
						borderColor: '#6B7280',
						backgroundColor: 'rgba(107, 114, 128, 0.1)',
						borderWidth: 4,
						fill: true,
						tension: 0.4
					}
				]
			};
		} else {
			monthlyTrendsData = {
				labels: monthlyData.labels,
				datasets: [
					{
						label: 'Passed',
						data: monthlyData.passedData,
						borderColor: '#10B981',
						backgroundColor: 'rgba(16, 185, 129, 0.15)',
						borderWidth: 4,
						fill: true,
						tension: 0.4
					},
					{
						label: 'Failed',
						data: monthlyData.failedData,
						borderColor: '#EF4444',
						backgroundColor: 'rgba(239, 68, 68, 0.15)',
						borderWidth: 4,
						fill: true,
						tension: 0.4
					}
				]
			};
		}
	}

	function calculateMonthlyTrends() {
		const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'];
		const passedData = [];
		const failedData = [];

		for (let i = 5; i >= 0; i--) {
			const currentDate = new Date();
			const monthStart = new Date(currentDate.getFullYear(), currentDate.getMonth() - i, 1);
			const monthEnd = new Date(currentDate.getFullYear(), currentDate.getMonth() - i + 1, 0);

			const monthBatches = batchData.filter((/** @type {any} */ batch) => {
				const batchDate = new Date(batch.date_of_production);
				return batchDate >= monthStart && batchDate <= monthEnd;
			});

			const passed = monthBatches.filter(
				(/** @type {any} */ batch) => batch.qc_status === 'Pass' || batch.qc_status === 'PASS'
			).length;
			const failed = monthBatches.filter(
				(/** @type {any} */ batch) => batch.qc_status === 'Fail' || batch.qc_status === 'FAIL'
			).length;

			passedData.push(passed);
			failedData.push(failed);
		}

		return { labels: months, passedData, failedData };
	}

	// Heatmap event handlers
	function handleMapMouseEnter(event) {
		tooltipContent = `${event.detail.state}: ${event.detail.pendingInspections} pending inspections`;
		tooltipVisible = true;
		tooltipX = event.detail.x;
		tooltipY = event.detail.y;
	}

	function handleMapMouseMove(event) {
		tooltipX = event.detail.x;
		tooltipY = event.detail.y;
	}

	function handleMapMouseLeave() {
		tooltipVisible = false;
	}
</script>

<div class="p-6">
	<div class="mb-8">
		<h1 class="mb-2 text-3xl font-bold text-white">Analytics Dashboard</h1>
		<p class="text-gray-400">Comprehensive insights and performance metrics</p>
	</div>

	<!-- Summary Cards -->
	<div class="mb-8 grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
		<div
			class="rounded-lg border border-gray-700 bg-black p-6 transition-all duration-300 hover:border-gray-500/50"
		>
			<div class="flex items-center justify-between">
				<div>
					<p class="mb-2 text-sm font-medium text-gray-400">Total Vendors</p>
					<p class="text-2xl font-bold text-white">{summaryStats.totalVendors}</p>
				</div>
				<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-blue-500/20">
					<svg class="h-6 w-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"
						></path>
					</svg>
				</div>
			</div>
		</div>

		<div
			class="rounded-lg border border-gray-700 bg-black p-6 transition-all duration-300 hover:border-gray-500/50"
		>
			<div class="flex items-center justify-between">
				<div>
					<p class="mb-2 text-sm font-medium text-gray-400">Total Batches</p>
					<p class="text-2xl font-bold text-white">{summaryStats.totalBatches}</p>
				</div>
				<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-green-500/20">
					<svg class="h-6 w-6 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
						></path>
					</svg>
				</div>
			</div>
		</div>

		<div
			class="rounded-lg border border-gray-700 bg-black p-6 transition-all duration-300 hover:border-gray-500/50"
		>
			<div class="flex items-center justify-between">
				<div>
					<p class="mb-2 text-sm font-medium text-gray-400">Pending Inspections</p>
					<p class="text-2xl font-bold text-white">{summaryStats.pendingInspections}</p>
				</div>
				<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-yellow-500/20">
					<svg
						class="h-6 w-6 text-yellow-400"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
						></path>
					</svg>
				</div>
			</div>
		</div>

		<div
			class="rounded-lg border border-gray-700 bg-black p-6 transition-all duration-300 hover:border-gray-500/50"
		>
			<div class="flex items-center justify-between">
				<div>
					<p class="mb-2 text-sm font-medium text-gray-400">Failed Batches</p>
					<p class="text-2xl font-bold text-white">{summaryStats.failedBatches}</p>
				</div>
				<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-red-500/20">
					<svg class="h-6 w-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						></path>
					</svg>
				</div>
			</div>
		</div>
	</div>

	<!-- Loading State -->
	{#if isLoading}
		<div class="flex items-center justify-center py-12">
			<div class="text-center">
				<div
					class="mx-auto mb-4 h-12 w-12 animate-spin rounded-full border-b-2 border-purple-500"
				></div>
				<p class="text-gray-400">Loading charts...</p>
			</div>
		</div>
	{:else}
		<!-- Charts Section -->
		<div class="mb-8 grid grid-cols-1 gap-6 lg:grid-cols-2">
			<!-- Pending Inspections Heatmap -->
			<div
				class="min-h-[400px] overflow-hidden rounded-lg border border-gray-700 bg-black p-6 lg:min-h-[500px]"
			>
				<h3 class="mb-4 text-lg font-bold text-white sm:mb-6 sm:text-xl">
					Pending Inspections Heatmap
				</h3>
				<div class="h-72 w-full overflow-hidden sm:h-80 lg:h-96">
					<div class="h-full w-full">
						<IndiaMap 
							data={heatmapData}
							width={600}
							height={400}
							on:mouseenter={handleMapMouseEnter}
							on:mousemove={handleMapMouseMove}
							on:mouseleave={handleMapMouseLeave}
						/>
					</div>
				</div>
				
				<!-- Color Legend -->
				<div class="mt-4 flex flex-wrap items-center justify-center gap-4">
					<span class="text-sm font-medium text-gray-300">Inspection Density:</span>
					<div class="flex items-center space-x-2">
						<div class="h-4 w-4 rounded" style="background-color: #6b7280;"></div>
						<span class="text-xs text-gray-400">Low (0-9)</span>
					</div>
					<div class="flex items-center space-x-2">
						<div class="h-4 w-4 rounded" style="background-color: #22c55e;"></div>
						<span class="text-xs text-gray-400">Medium (10-19)</span>
					</div>
					<div class="flex items-center space-x-2">
						<div class="h-4 w-4 rounded" style="background-color: #eab308;"></div>
						<span class="text-xs text-gray-400">High (20-29)</span>
					</div>
					<div class="flex items-center space-x-2">
						<div class="h-4 w-4 rounded" style="background-color: #f97316;"></div>
						<span class="text-xs text-gray-400">Very High (30-39)</span>
					</div>
					<div class="flex items-center space-x-2">
						<div class="h-4 w-4 rounded" style="background-color: #ef4444;"></div>
						<span class="text-xs text-gray-400">Critical (40+)</span>
					</div>
				</div>
			</div>

			<!-- Top Vendor Performance -->
			<div
				class="min-h-[400px] overflow-hidden rounded-lg border border-gray-700 bg-black p-6 lg:min-h-[500px]"
			>
				<h3 class="mb-4 text-lg font-bold text-white sm:mb-6 sm:text-xl">Top Vendor Performance</h3>
				<div class="h-72 w-full overflow-hidden sm:h-80 lg:h-96">
					<div class="h-full w-full">
						<Chart
							type="bar"
							data={vendorChartData}
							options={{
								responsive: true,
								maintainAspectRatio: false,
								plugins: {
									legend: { display: false },
									tooltip: {
										backgroundColor: 'rgba(0, 0, 0, 0.8)',
										titleColor: '#FFFFFF',
										bodyColor: '#D1D5DB',
										borderColor: '#374151',
										borderWidth: 1,
										cornerRadius: 8
									}
								},
								scales: {
									y: {
										beginAtZero: true,
										ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } },
										grid: { color: '#374151' },
										border: { display: false }
									},
									x: {
										ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } },
										grid: { display: false },
										border: { display: false }
									}
								},
								elements: { bar: { borderRadius: 4 } },
								animation: { duration: 2000 }
							}}
						/>
					</div>
				</div>
			</div>
		</div>

		<!-- Monthly Quality Trends - Full Width -->
		<div class="mb-8">
			<div
				class="min-h-[400px] overflow-hidden rounded-lg border border-gray-700 bg-black p-6 lg:min-h-[500px]"
			>
				<h3 class="mb-4 text-lg font-bold text-white sm:mb-6 sm:text-xl">Monthly Quality Trends</h3>
				<div class="h-72 w-full overflow-hidden sm:h-80 lg:h-96">
					<div class="h-full w-full">
						<Chart
							type="line"
							data={monthlyTrendsData}
							options={{
								responsive: true,
								maintainAspectRatio: false,
								plugins: {
									legend: {
										position: 'top',
										labels: {
											color: '#D1D5DB',
											font: { size: 12, weight: 'bold' },
											padding: 20,
											usePointStyle: true
										}
									},
									tooltip: {
										backgroundColor: 'rgba(0, 0, 0, 0.8)',
										titleColor: '#FFFFFF',
										bodyColor: '#D1D5DB',
										borderColor: '#374151',
										borderWidth: 1,
										cornerRadius: 8
									}
								},
								scales: {
									y: {
										beginAtZero: true,
										ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } },
										grid: { color: '#374151' },
										border: { display: false }
									},
									x: {
										ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } },
										grid: { display: false },
										border: { display: false }
									}
								},
								elements: { point: { radius: 4, hoverRadius: 6 } },
								animation: { duration: 2000 }
							}}
						/>
					</div>
				</div>
			</div>
		</div>

		<!-- Quality Status Distribution - Moved Below -->
		<div class="mb-8">
			<div
				class="min-h-[400px] overflow-hidden rounded-lg border border-gray-700 bg-black p-6 lg:min-h-[500px]"
			>
				<h3 class="mb-4 text-lg font-bold text-white sm:mb-6 sm:text-xl">
					Quality Status Distribution
				</h3>
				<div class="h-72 w-full overflow-hidden sm:h-80 lg:h-96">
					<div class="h-full w-full">
						<Chart
							type="doughnut"
							data={qualityChartData}
							options={{
								responsive: true,
								maintainAspectRatio: false,
								plugins: {
									legend: {
										position: 'bottom',
										labels: {
											color: '#D1D5DB',
											font: { size: 12, weight: 'bold' },
											padding: 20,
											usePointStyle: true
										}
									},
									tooltip: {
										backgroundColor: 'rgba(0, 0, 0, 0.8)',
										titleColor: '#FFFFFF',
										bodyColor: '#D1D5DB',
										borderColor: '#374151',
										borderWidth: 1,
										cornerRadius: 8
									}
								},
								cutout: '60%'
							}}
						/>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Tooltip -->
	{#if tooltipVisible && browser}
		<div
			class="fixed z-50 rounded-lg border border-gray-600 bg-gray-800 px-3 py-2 text-sm text-white shadow-lg pointer-events-none"
			style="left: {tooltipX - 60}px; top: {tooltipY - 40}px; transform: translate(-50%, -100%);"
		>
			{tooltipContent}
		</div>
	{/if}
</div>
