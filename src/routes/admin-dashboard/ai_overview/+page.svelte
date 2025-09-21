<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { fetchData } from '$lib/utils';
	import Chart from '$lib/components/Chart.svelte';

	let user = null;
	/** @type {any[]} */
	let reportData = [];
	let isLoading = true;
	let isSummarizing = false;
	let aiSummary = '';
	let aiSummaryHtml = '';
	let errorMsg = '';
	let extraContext = '';

	// Charts data
	let qualityChartData = {
		labels: ['Pass', 'Fail'],
		datasets: [
			{
				data: [0, 0],
				backgroundColor: ['#10B981', '#EF4444'],
				borderColor: ['#059669', '#DC2626'],
				borderWidth: 3
			}
		]
	};
	let issueChartData = {
		labels: /** @type {string[]} */ ([]),
		datasets: [
			{
				label: 'Issues',
				data: /** @type {number[]} */ ([]),
				backgroundColor: 'rgba(245, 158, 11, 0.8)',
				borderColor: '#F59E0B',
				borderWidth: 3,
				borderRadius: 8
			}
		]
	};

	// Minimal, safe markdown to HTML converter (headings, bold, italics, lists)
	/** @param {string} str */
	function escapeHtml(str) {
		return String(str)
			.replaceAll('&', '&amp;')
			.replaceAll('<', '&lt;')
			.replaceAll('>', '&gt;')
			.replaceAll('"', '&quot;')
			.replaceAll("'", '&#39;');
	}

	/** @param {string} text */
	function inlineMd(text) {
		// bold **text**
		text = text.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
		// italics _text_ or *text*
		text = text.replace(
			/(?:^|\W)_(.+?)_(?=\W|$)/g,
			function (/** @type {string} */ m, /** @type {string} */ p1) {
				return m.replace(`_${p1}_`, `<em>${p1}</em>`);
			}
		);
		text = text.replace(
			/(?:^|\W)\*(.+?)\*(?=\W|$)/g,
			function (/** @type {string} */ m, /** @type {string} */ p1) {
				return m.replace(`*${p1}*`, `<em>${p1}</em>`);
			}
		);
		return text;
	}

	/** @param {string} md */
	function mdToHtml(md) {
		const lines = escapeHtml(md || '').split(/\r?\n/);
		let html = '';
		let inList = false;
		for (const raw of lines) {
			const line = raw.trim();
			// headings
			const h = line.match(/^(#{1,6})\s+(.*)$/);
			if (h) {
				if (inList) {
					html += '</ul>';
					inList = false;
				}
				const level = h[1].length;
				html += `<h${level}>${inlineMd(h[2])}</h${level}>`;
				continue;
			}
			// list items
			const li = line.match(/^[-*]\s+(.*)$/);
			if (li) {
				if (!inList) {
					html += '<ul>';
					inList = true;
				}
				html += `<li>${inlineMd(li[1])}</li>`;
				continue;
			}
			// paragraph or blank
			if (inList) {
				html += '</ul>';
				inList = false;
			}
			if (line.length) {
				html += `<p>${inlineMd(line)}</p>`;
			}
		}
		if (inList) html += '</ul>';
		return html;
	}

	function updateCharts() {
		// Pass/Fail from reports (status: 1 pass, 0 fail)
		const passCount = reportData.filter(
			(r) => r.status === 1 || r.status === '1' || r.status === 'PASS' || r.status === 'Pass'
		).length;
		const failCount = reportData.filter(
			(r) => r.status === 0 || r.status === '0' || r.status === 'FAIL' || r.status === 'Fail'
		).length;
		const total = passCount + failCount;
		if (total === 0) {
			qualityChartData = {
				labels: ['No Data'],
				datasets: [
					{ data: [1], backgroundColor: ['#6B7280'], borderColor: ['#4B5563'], borderWidth: 3 }
				]
			};
		} else {
			qualityChartData = {
				labels: ['Pass', 'Fail'],
				datasets: [
					{
						data: [passCount, failCount],
						backgroundColor: ['#10B981', '#EF4444'],
						borderColor: ['#059669', '#DC2626'],
						borderWidth: 3
					}
				]
			};
		}

		// Issue categories from remark text (simple keyword binning)
		const bins = {
			Packaging: [/packag/i, /carton/i, /seal/i, /strap/i],
			Labeling: [/label/i, /serial/i, /misalign/i],
			Rust: [/rust/i, /corrosion/i],
			Cracks: [/crack/i, /fracture/i],
			Moisture: [/moist/i, /wet/i, /damp/i]
		};
		const counts = /** @type {Record<string, number>} */ ({});
		Object.keys(bins).forEach((k) => (counts[k] = 0));
		for (const r of reportData) {
			const text = `${r.remark ?? ''}`;
			for (const [k, patterns] of Object.entries(bins)) {
				if (patterns.some((re) => re.test(text))) counts[k]++;
			}
		}
		const labels = Object.keys(counts);
		const values = labels.map((l) => counts[l]);
		if (values.every((v) => v === 0)) {
			issueChartData = {
				labels: ['No Data'],
				datasets: [
					{
						label: 'Issues',
						data: [1],
						backgroundColor: 'rgba(107,114,128,0.8)',
						borderColor: '#6B7280',
						borderWidth: 3,
						borderRadius: 8
					}
				]
			};
		} else {
			issueChartData = {
				labels,
				datasets: [
					{
						label: 'Issues',
						data: values,
						backgroundColor: 'rgba(245, 158, 11, 0.8)',
						borderColor: '#F59E0B',
						borderWidth: 3,
						borderRadius: 8
					}
				]
			};
		}
	}

	async function summarizeReports() {
		if (!reportData?.length) return;
		isSummarizing = true;
		errorMsg = '';
		aiSummary = '';
		try {
			const res = await fetch('/api/ai/summary', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ reports: reportData, context: extraContext })
			});
			if (!res.ok) throw new Error(`AI summary failed: ${res.status}`);
			const data = await res.json();
			aiSummary = data.summary || 'No summary returned.';
			aiSummaryHtml = mdToHtml(aiSummary);
		} catch (e) {
			console.error(e);
			errorMsg = 'Failed to generate AI summary.';
		} finally {
			isSummarizing = false;
		}
	}

	onMount(async () => {
		const userData = sessionStorage.getItem('user');
		if (!userData) {
			goto('/login');
			return;
		}

		user = JSON.parse(userData);

		try {
			const reports = await fetchData('/api/reports');
			reportData = reports;
			// Auto-generate on load
			updateCharts();
			summarizeReports();
		} catch (error) {
			console.error('Error loading data:', error);
			errorMsg = 'Failed to load reports.';
		} finally {
			isLoading = false;
		}
	});
</script>

<div class="p-4 sm:p-6 lg:p-8">
	<div class="mb-6 sm:mb-8">
		<h1 class="mb-2 text-2xl font-bold text-white sm:text-3xl lg:text-4xl">AI Overview</h1>
		<p class="text-sm text-gray-400">Automated summary of inspection reports</p>
	</div>

	{#if isLoading}
		<div class="text-gray-300">Loading reports…</div>
	{:else}
		{#if errorMsg}
			<div class="mb-4 rounded border border-red-600/40 bg-red-500/5 p-3 text-red-400">
				{errorMsg}
			</div>
		{/if}

		<div class="mb-6 rounded-lg border border-gray-700 bg-black p-4 sm:p-6">
			<div class="mb-4 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
				<div class="text-sm text-gray-300">Reports loaded: {reportData.length}</div>
				<div class="flex gap-2">
					<button
						class="rounded border border-blue-500/40 bg-gray-800 px-4 py-2 text-white transition-all hover:border-blue-400/60 hover:bg-gray-700 disabled:opacity-60"
						on:click={summarizeReports}
						disabled={isSummarizing || !reportData.length}
					>
						{isSummarizing ? 'Summarizing…' : 'Regenerate Summary'}
					</button>
				</div>
			</div>

			<label for="ai-context" class="mb-2 block text-xs text-gray-400"
				>Optional context for AI</label
			>
			<textarea
				id="ai-context"
				class="mb-4 w-full rounded border border-gray-700 bg-transparent p-2 text-gray-200 focus:border-gray-500 focus:outline-none"
				rows="3"
				bind:value={extraContext}
				placeholder="e.g., Focus on failure clusters for the last month"
			></textarea>

			<div class="prose max-w-none prose-invert">
				{#if aiSummaryHtml}
					{@html aiSummaryHtml}
				{:else}
					<p class="text-gray-400">No summary generated yet.</p>
				{/if}
			</div>
		</div>
	{/if}
</div>

<!-- Charts Section -->
{#if !isLoading}
	<div class="p-4 pt-0 sm:p-6 lg:p-8">
		<div class="mb-8 grid grid-cols-1 gap-6 lg:grid-cols-2">
			<div class="min-h-[320px] overflow-hidden rounded-lg border border-gray-700 bg-black p-6">
				<h3 class="mb-4 text-lg font-bold text-white sm:text-xl">Pass vs Fail</h3>
				<div class="h-64 w-full">
					<Chart
						type="doughnut"
						data={qualityChartData}
						options={{
							plugins: { legend: { position: 'bottom', labels: { color: '#D1D5DB' } } },
							cutout: '60%'
						}}
					/>
				</div>
			</div>
			<div class="min-h-[320px] overflow-hidden rounded-lg border border-gray-700 bg-black p-6">
				<h3 class="mb-4 text-lg font-bold text-white sm:text-xl">Issue Categories</h3>
				<div class="h-64 w-full">
					<Chart
						type="bar"
						data={issueChartData}
						options={{
							plugins: { legend: { display: false } },
							scales: {
								y: { beginAtZero: true, ticks: { color: '#9ca3af' }, grid: { color: '#374151' } },
								x: { ticks: { color: '#9ca3af' }, grid: { display: false } }
							}
						}}
					/>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.prose :global(ul) {
		list-style: disc;
		padding-left: 1.25rem;
	}
	.prose :global(ol) {
		list-style: decimal;
		padding-left: 1.25rem;
	}
	.prose :global(h3) {
		margin-top: 0.75rem;
	}
</style>
