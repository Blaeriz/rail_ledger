<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let user = null;
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
  <title>Inspection Reports - Rail Ledger</title>
</svelte:head>

<div class="space-y-6">
  <h2 class="text-3xl font-bold text-white">Inspection Reports</h2>

  <div class="bg-gray-900/40 backdrop-blur-xl border border-gray-700/40 rounded-2xl p-6">
    <div class="overflow-x-auto">
      <table class="w-full">
        <thead>
          <tr class="border-b border-gray-700/40">
            <th class="text-left text-gray-300 py-3 px-4">Report ID</th>
            <th class="text-left text-gray-300 py-3 px-4">Batch ID</th>
            <th class="text-left text-gray-300 py-3 px-4">Inspector</th>
            <th class="text-left text-gray-300 py-3 px-4">Status</th>
            <th class="text-left text-gray-300 py-3 px-4">Remark</th>
            <th class="text-left text-gray-300 py-3 px-4">Created</th>
          </tr>
        </thead>
        <tbody>
          {#each reportData as report}
            <tr class="border-b border-gray-700/20 hover:bg-gray-800/30">
              <td class="py-3 px-4 text-white font-mono">{report.reportId}</td>
              <td class="py-3 px-4 text-gray-300">{report.batchId}</td>
              <td class="py-3 px-4 text-gray-300">{report.inspectorName}</td>
              <td class="py-3 px-4">
                <span class="px-3 py-1 rounded-full text-xs font-semibold border-2 {
                  report.status === 1 ? 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10' : 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10'
                }">
                  {report.status === 1 ? 'Pass' : 'Fail'}
                </span>
              </td>
              <td class="py-3 px-4 text-gray-300">{report.remark || 'N/A'}</td>
              <td class="py-3 px-4 text-gray-300">{formatDate(report.createdAt)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</div>
