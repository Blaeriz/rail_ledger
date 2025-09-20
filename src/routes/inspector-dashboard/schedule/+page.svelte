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
      batchData = await batchesRes.json() || [];
    } catch (error) {
      console.error('Error fetching data:', error);
      batchData = [];
    }
  }

  // Get today's inspections (batches without inspection date)
  $: todaysInspections = batchData.filter(b => !b.last_inspection_date).slice(0, 5);
  
  // Get upcoming inspections (remaining batches)
  $: upcomingInspections = batchData.filter(b => !b.last_inspection_date).slice(5, 10);
</script>

<svelte:head>
  <title>Inspection Schedule - Rail Ledger</title>
</svelte:head>

<div class="p-6">
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-white mb-2">Inspection Schedule</h1>
    <p class="text-gray-400">Manage your inspection tasks and schedule</p>
  </div>

  {#if isLoading}
    <div class="flex items-center justify-center py-12">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500 mx-auto mb-4"></div>
        <p class="text-gray-400">Loading schedule...</p>
      </div>
    </div>
  {:else}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Today's Inspections -->
      <div class="bg-black border border-gray-700 rounded-lg p-6 hover:border-gray-500/50 transition-all duration-300">
        <h3 class="text-xl font-bold text-white mb-6">Today's Inspections</h3>
        <div class="space-y-4">
          {#if todaysInspections.length > 0}
            {#each todaysInspections as batch}
              <div class="flex items-center justify-between p-4 bg-gray-800/30 rounded-lg border border-gray-600/30 hover:border-gray-500/50 transition-all duration-200">
                <div class="flex-1">
                  <p class="text-white font-semibold text-lg">{batch.batch_id || 'BATCH' + Math.random().toString(36).substr(2, 6).toUpperCase()}</p>
                  <p class="text-gray-400 text-sm mt-1">{batch.vendor_id || 'VEND' + Math.random().toString(36).substr(2, 6).toUpperCase()}</p>
                </div>
                <button class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 hover:shadow-lg hover:shadow-blue-500/25">
                  Start
                </button>
              </div>
            {/each}
          {:else}
            <div class="text-center py-8">
              <div class="w-16 h-16 bg-gray-700/30 rounded-full flex items-center justify-center mx-auto mb-4">
                <svg class="w-8 h-8 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                </svg>
              </div>
              <p class="text-gray-400">No inspections scheduled for today</p>
            </div>
          {/if}
        </div>
      </div>

      <!-- Upcoming Inspections -->
      <div class="bg-black border border-gray-700 rounded-lg p-6 hover:border-gray-500/50 transition-all duration-300">
        <h3 class="text-xl font-bold text-white mb-6">Upcoming Inspections</h3>
        <div class="space-y-4">
          {#if upcomingInspections.length > 0}
            {#each upcomingInspections as batch}
              <div class="flex items-center justify-between p-4 bg-gray-800/30 rounded-lg border border-gray-600/30 hover:border-gray-500/50 transition-all duration-200">
                <div class="flex-1">
                  <p class="text-white font-semibold text-lg">{batch.batch_id || 'BATCH' + Math.random().toString(36).substr(2, 6).toUpperCase()}</p>
                  <p class="text-gray-400 text-sm mt-1">{batch.vendor_id || 'VEND' + Math.random().toString(36).substr(2, 6).toUpperCase()}</p>
                </div>
                <span class="text-gray-400 text-sm font-medium px-3 py-1 bg-gray-700/30 rounded-lg">
                  Scheduled
                </span>
              </div>
            {/each}
          {:else}
            <div class="text-center py-8">
              <div class="w-16 h-16 bg-gray-700/30 rounded-full flex items-center justify-center mx-auto mb-4">
                <svg class="w-8 h-8 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
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
