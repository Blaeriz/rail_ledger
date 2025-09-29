<!-- @ts-nocheck -->
<script>
	import Icon from './Icon.svelte';

	export let data = /** @type {any[]} */ ([]);
	export let columns = /** @type {any[]} */ ([]);
	export let searchable = false;
	export let searchPlaceholder = 'Search...';
	export let showPagination = false;
	export let pageSize = 20;

	let searchTerm = '';
	let currentPage = 1;

	$: filteredData =
		searchable && searchTerm
			? data.filter((item) =>
					Object.values(item).some((value) =>
						String(value).toLowerCase().includes(searchTerm.toLowerCase())
					)
				)
			: data;

	$: paginatedData = showPagination
		? filteredData.slice((currentPage - 1) * pageSize, currentPage * pageSize)
		: filteredData;

	$: totalPages = Math.ceil(filteredData.length / pageSize);

	function handleSearch(/** @type {any} */ event) {
		searchTerm = event.target.value;
		currentPage = 1;
	}

	function goToPage(/** @type {number} */ page) {
		currentPage = page;
	}
</script>

<div
	class="group overflow-hidden rounded-lg border border-gray-700 bg-black shadow-2xl shadow-black/20 transition-all duration-500 hover:shadow-2xl hover:shadow-purple-500/5"
>
	{#if searchable}
		<div
			class="border-b border-gray-700 bg-gradient-to-r from-gray-900/50 to-gray-800/30 p-4 sm:p-6"
		>
			<div class="group relative">
				<input
					type="text"
					placeholder={searchPlaceholder}
					bind:value={searchTerm}
					on:input={handleSearch}
					class="w-full rounded-xl border border-gray-700 bg-gray-800/80 px-4 py-3 pl-12 text-white backdrop-blur-sm transition-all duration-300 hover:border-gray-600/50 hover:bg-gray-800/90 focus:border-purple-500/50 focus:bg-gray-800 focus:ring-2 focus:ring-purple-500/20 focus:outline-none"
				/>
				<div
					class="absolute top-3.5 left-4 text-gray-400 transition-colors duration-300 group-hover:text-purple-400"
				>
					<Icon name="search" size="w-5 h-5" />
				</div>
				<div
					class="pointer-events-none absolute inset-0 rounded-xl bg-gradient-to-r from-purple-500/5 to-blue-500/5 opacity-0 transition-opacity duration-300 group-hover:opacity-100"
				></div>
			</div>
		</div>
	{/if}

	<div class="overflow-x-auto">
		<table class="w-full min-w-[600px]">
			<thead>
				<tr
					class="border-b border-gray-700 bg-gradient-to-r from-gray-900 via-gray-800 to-gray-900"
				>
					{#each columns as column}
						<th
							class="group relative px-4 py-4 text-left text-xs font-bold tracking-wider text-white uppercase sm:px-6 sm:py-5 sm:text-sm"
						>
							<span class="relative z-10">{column.label}</span>
							<div
								class="absolute inset-0 bg-gradient-to-r from-purple-500/5 to-blue-500/5 opacity-0 transition-opacity duration-300 group-hover:opacity-100"
							></div>
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each paginatedData as item, index}
					<tr
						class="group animate-fade-in border-b border-gray-700/50 bg-black transition-all duration-300 hover:bg-gradient-to-r hover:from-gray-800/20 hover:to-gray-700/10 hover:shadow-lg hover:shadow-purple-500/5"
						style="animation-delay: {index * 50}ms"
					>
						{#each columns as column}
							<td
								class="relative px-4 py-4 text-xs text-white transition-all duration-300 group-hover:bg-gray-800/10 sm:px-6 sm:py-5 sm:text-sm"
							>
								<div class="relative z-10">
									{#if column.render}
										{@html column.render(item)}
									{:else}
										{item[column.key]}
									{/if}
								</div>
								<div
									class="absolute inset-0 rounded-sm bg-gradient-to-r from-purple-500/3 to-blue-500/3 opacity-0 transition-opacity duration-300 group-hover:opacity-100"
								></div>
							</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	{#if showPagination && totalPages > 1}
		<div
			class="flex flex-col items-start justify-between gap-4 border-t border-gray-700/50 bg-gradient-to-r from-gray-900/50 to-gray-800/30 p-4 backdrop-blur-sm sm:flex-row sm:items-center sm:p-6"
		>
			<span class="text-sm font-medium text-gray-400">
				Showing <span class="font-semibold text-purple-400">{(currentPage - 1) * pageSize + 1}</span
				>
				to
				<span class="font-semibold text-purple-400"
					>{Math.min(currentPage * pageSize, filteredData.length)}</span
				>
				of <span class="font-semibold text-purple-400">{filteredData.length}</span> results
			</span>
			<div class="flex space-x-2">
				<button
					on:click={() => goToPage(currentPage - 1)}
					disabled={currentPage === 1}
					class="group rounded-lg border border-gray-600/50 bg-gray-800/80 px-4 py-2 font-medium text-white backdrop-blur-sm transition-all duration-300 hover:border-purple-500/50 hover:bg-gray-700/30 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600/40 active:shadow-xl active:shadow-purple-500/30 disabled:cursor-not-allowed disabled:opacity-50"
				>
					<span class="transition-transform duration-300 group-hover:translate-x-[-2px]"
						>Previous</span
					>
				</button>
				<div
					class="rounded-lg border border-purple-500/20 bg-gradient-to-r from-purple-500/10 to-blue-500/10 px-4 py-2 text-purple-300 backdrop-blur-sm"
				>
					<span class="font-semibold">Page {currentPage} of {totalPages}</span>
				</div>
				<button
					on:click={() => goToPage(currentPage + 1)}
					disabled={currentPage === totalPages}
					class="group rounded-lg border border-gray-600/50 bg-gray-800/80 px-4 py-2 font-medium text-white backdrop-blur-sm transition-all duration-300 hover:border-purple-500/50 hover:bg-gray-700/30 hover:shadow-lg hover:shadow-purple-500/20 active:border-purple-400/60 active:bg-gray-600/40 active:shadow-xl active:shadow-purple-500/30 disabled:cursor-not-allowed disabled:opacity-50"
				>
					<span class="transition-transform duration-300 group-hover:translate-x-[2px]">Next</span>
				</button>
			</div>
		</div>
	{/if}
</div>
