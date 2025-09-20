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

  function formatDate(/** @type {any} */ dateString) {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleDateString();
  }
</script>

<svelte:head>
  <title>Batch Information - Rail Ledger</title>
</svelte:head>

<div class="space-y-6">
  <h2 class="text-3xl font-bold text-white">Batch Information</h2>

  <div class="bg-gray-900/40 backdrop-blur-xl border border-gray-700/40 rounded-2xl p-6">
    <div class="overflow-x-auto">
      <table class="w-full">
        <thead>
          <tr class="border-b border-gray-700/40">
            <th class="text-left text-gray-300 py-3 px-4">Batch ID</th>
            <th class="text-left text-gray-300 py-3 px-4">Vendor ID</th>
            <th class="text-left text-gray-300 py-3 px-4">Size</th>
            <th class="text-left text-gray-300 py-3 px-4">Status</th>
            <th class="text-left text-gray-300 py-3 px-4">Production Date</th>
            <th class="text-left text-gray-300 py-3 px-4">Last Inspection</th>
          </tr>
        </thead>
        <tbody>
          {#each batchData as batch}
            <tr class="border-b border-gray-700/20 hover:bg-gray-800/30">
              <td class="py-3 px-4 text-white font-mono">{batch.batch_id}</td>
              <td class="py-3 px-4 text-gray-300">{batch.vendor_id}</td>
              <td class="py-3 px-4 text-gray-300">{batch.batch_size}</td>
              <td class="py-3 px-4">
                <span class="px-3 py-1 rounded-full text-xs font-semibold border-2 {
                  batch.qc_status === 'Pass' ? 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10' :
                  batch.qc_status === 'Fail' ? 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10' :
                  'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10'
                }">
                  {batch.qc_status || 'Pending'}
                </span>
              </td>
              <td class="py-3 px-4 text-gray-300">{formatDate(batch.date_of_production)}</td>
              <td class="py-3 px-4 text-gray-300">{formatDate(batch.last_inspection_date)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>
