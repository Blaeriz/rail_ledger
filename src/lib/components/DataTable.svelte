<!-- @ts-nocheck -->
<script>
  import Icon from './Icon.svelte';
  
  export let data = /** @type {any[]} */ ([]);
  export let columns = /** @type {any[]} */ ([]);
  export let searchable = false;
  export let searchPlaceholder = 'Search...';
  export let showPagination = false;
  export let pageSize = 10;
  
  let searchTerm = '';
  let currentPage = 1;
  
  $: filteredData = searchable && searchTerm 
    ? data.filter(item => 
        Object.values(item).some(value => 
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

<div class="bg-black border border-gray-700 rounded-lg overflow-hidden shadow-2xl shadow-black/20 hover:shadow-2xl hover:shadow-purple-500/5 transition-all duration-500 group">
  {#if searchable}
    <div class="p-4 sm:p-6 border-b border-gray-700 bg-gradient-to-r from-gray-900/50 to-gray-800/30">
      <div class="relative group">
        <input 
          type="text" 
          placeholder={searchPlaceholder}
          bind:value={searchTerm}
          on:input={handleSearch}
          class="w-full bg-gray-800/80 text-white px-4 py-3 pl-12 rounded-xl border border-gray-700 focus:border-purple-500/50 focus:outline-none focus:ring-2 focus:ring-purple-500/20 focus:bg-gray-800 transition-all duration-300 hover:border-gray-600/50 hover:bg-gray-800/90 backdrop-blur-sm"
        />
        <div class="absolute left-4 top-3.5 text-gray-400 group-hover:text-purple-400 transition-colors duration-300">
          <Icon name="search" size="w-5 h-5" />
        </div>
        <div class="absolute inset-0 rounded-xl bg-gradient-to-r from-purple-500/5 to-blue-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"></div>
      </div>
    </div>
  {/if}
  
  <div class="overflow-x-auto">
    <table class="w-full min-w-[600px]">
      <thead>
        <tr class="border-b border-gray-700 bg-gradient-to-r from-gray-900 via-gray-800 to-gray-900">
          {#each columns as column}
            <th class="text-left text-white font-bold py-4 sm:py-5 px-4 sm:px-6 text-xs sm:text-sm uppercase tracking-wider relative group">
              <span class="relative z-10">{column.label}</span>
              <div class="absolute inset-0 bg-gradient-to-r from-purple-500/5 to-blue-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each paginatedData as item, index}
          <tr class="border-b border-gray-700/50 hover:bg-gradient-to-r hover:from-gray-800/20 hover:to-gray-700/10 bg-black group transition-all duration-300 hover:shadow-lg hover:shadow-purple-500/5 animate-fade-in" style="animation-delay: {index * 50}ms">
            {#each columns as column}
              <td class="py-4 sm:py-5 px-4 sm:px-6 text-white text-xs sm:text-sm relative group-hover:bg-gray-800/10 transition-all duration-300">
                <div class="relative z-10">
                  {#if column.render}
                    {@html column.render(item)}
                  {:else}
                    {item[column.key]}
                  {/if}
                </div>
                <div class="absolute inset-0 bg-gradient-to-r from-purple-500/3 to-blue-500/3 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-sm"></div>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  
  {#if showPagination && totalPages > 1}
    <div class="p-4 sm:p-6 border-t border-gray-700/50 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 bg-gradient-to-r from-gray-900/50 to-gray-800/30 backdrop-blur-sm">
      <span class="text-gray-400 text-sm font-medium">
        Showing <span class="text-purple-400 font-semibold">{((currentPage - 1) * pageSize) + 1}</span> to <span class="text-purple-400 font-semibold">{Math.min(currentPage * pageSize, filteredData.length)}</span> of <span class="text-purple-400 font-semibold">{filteredData.length}</span> results
      </span>
      <div class="flex space-x-2">
        <button 
          on:click={() => goToPage(currentPage - 1)}
          disabled={currentPage === 1}
          class="px-4 py-2 bg-gray-800/80 text-white rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-700/30 border border-gray-600/50 hover:border-purple-500/50 hover:shadow-lg hover:shadow-purple-500/20 active:bg-gray-600/40 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/30 transition-all duration-300 font-medium backdrop-blur-sm group"
        >
          <span class="group-hover:translate-x-[-2px] transition-transform duration-300">Previous</span>
        </button>
        <div class="px-4 py-2 bg-gradient-to-r from-purple-500/10 to-blue-500/10 text-purple-300 rounded-lg border border-purple-500/20 backdrop-blur-sm">
          <span class="font-semibold">Page {currentPage} of {totalPages}</span>
        </div>
        <button 
          on:click={() => goToPage(currentPage + 1)}
          disabled={currentPage === totalPages}
          class="px-4 py-2 bg-gray-800/80 text-white rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-700/30 border border-gray-600/50 hover:border-purple-500/50 hover:shadow-lg hover:shadow-purple-500/20 active:bg-gray-600/40 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/30 transition-all duration-300 font-medium backdrop-blur-sm group"
        >
          <span class="group-hover:translate-x-[2px] transition-transform duration-300">Next</span>
        </button>
      </div>
    </div>
  {/if}
</div>
