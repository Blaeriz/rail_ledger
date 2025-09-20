<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let user = /** @type {any} */ (null);
  let vendorData = /** @type {any[]} */ ([]);
  let batchData = /** @type {any[]} */ ([]);
  let reportData = /** @type {any[]} */ ([]);
  let isLoading = true;

  // Viewer-specific statistics
  let viewerStats = /** @type {any} */ ({
    totalVendors: 0,
    totalBatches: 0,
    passedBatches: 0,
    failedBatches: 0,
    pendingInspections: 0,
    recentActivity: /** @type {any[]} */ ([])
  });

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
    calculateViewerStats();
    isLoading = false;
  });

  async function fetchAllData() {
    try {
      const [vendorsRes, batchesRes, reportsRes] = await Promise.all([
        fetch('/api/vendors'),
        fetch('/api/batches'),
        fetch('/api/reports')
      ]);

      vendorData = await vendorsRes.json();
      batchData = await batchesRes.json();
      reportData = await reportsRes.json();
    } catch (error) {
      console.error('Error fetching data:', error);
    }
  }

  function calculateViewerStats() {
    viewerStats.totalVendors = vendorData.length;
    viewerStats.totalBatches = batchData.length;
    viewerStats.passedBatches = batchData.filter(b => b.qc_status === 'Pass').length;
    viewerStats.failedBatches = batchData.filter(b => b.qc_status === 'Fail').length;
    viewerStats.pendingInspections = batchData.filter(b => !b.last_inspection_date).length;

    // Recent activity (last 10 reports)
    viewerStats.recentActivity = reportData
      .sort((/** @type {any} */ a, /** @type {any} */ b) => {
        const dateA = new Date(a.createdAt).getTime();
        const dateB = new Date(b.createdAt).getTime();
        return dateB - dateA;
      })
      .slice(0, 10);
  }

  function formatDate(/** @type {any} */ dateString) {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleDateString();
  }

  function getPassRate() {
    if (viewerStats.totalBatches === 0) return 0;
    return Math.round((viewerStats.passedBatches / viewerStats.totalBatches) * 100);
  }
</script>

<svelte:head>
  <title>System Overview - Rail Ledger</title>
</svelte:head>

<div class="p-4 sm:p-6 lg:p-8">
  <!-- Page Header -->
  <div class="mb-6 sm:mb-8">
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white mb-2">System Overview</h1>
  </div>
  
  <!-- Summary Cards -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6 mb-6 sm:mb-8">
    <div class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg">
      <div>
        <p class="text-gray-400 text-xs sm:text-sm font-medium mb-2">TOTAL VENDORS</p>
        <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">{viewerStats.totalVendors}</p>
      </div>
    </div>

    <div class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg">
      <div>
        <p class="text-gray-400 text-xs sm:text-sm font-medium mb-2">TOTAL BATCHES</p>
        <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">{viewerStats.totalBatches}</p>
      </div>
    </div>

    <div class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg">
      <div>
        <p class="text-gray-400 text-xs sm:text-sm font-medium mb-2">PASSED BATCHES</p>
        <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">{viewerStats.passedBatches}</p>
      </div>
    </div>

    <div class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg">
      <div>
        <p class="text-gray-400 text-xs sm:text-sm font-medium mb-2">FAILED BATCHES</p>
        <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">{viewerStats.failedBatches}</p>
      </div>
    </div>
  </div>

  <!-- Quality Metrics -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6 mb-6 sm:mb-8">
    <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 sm:p-6">
      <h3 class="text-lg sm:text-xl font-bold text-white mb-4">Quality Overview</h3>
      <div class="space-y-4">
        <div class="flex justify-between items-center">
          <span class="text-gray-300 text-sm">Overall Pass Rate</span>
          <span class="text-2xl font-bold text-green-400">{getPassRate()}%</span>
        </div>
        <div class="w-full bg-gray-700 rounded-full h-3">
          <div class="bg-gradient-to-r from-green-500 to-emerald-500 h-3 rounded-full" style="width: {getPassRate()}%"></div>
        </div>
        <div class="grid grid-cols-2 gap-4 mt-4">
          <div class="text-center">
            <p class="text-2xl font-bold text-blue-400">{viewerStats.pendingInspections}</p>
            <p class="text-gray-400 text-sm">Pending</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-yellow-400">{viewerStats.totalVendors}</p>
            <p class="text-gray-400 text-sm">Active Vendors</p>
          </div>
        </div>
      </div>
    </div>

    <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 sm:p-6">
      <h3 class="text-lg sm:text-xl font-bold text-white mb-4">Recent Activity</h3>
      <div class="space-y-3 max-h-64 overflow-y-auto">
        {#each viewerStats.recentActivity as activity}
          <div class="flex items-center space-x-3 p-3 bg-gray-700 rounded-lg">
            <div class="w-8 h-8 bg-blue-600/20 rounded-full flex items-center justify-center">
              <svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"></path>
              </svg>
            </div>
            <div class="flex-1">
              <p class="text-white text-sm">Report #{activity.reportId}</p>
              <p class="text-gray-400 text-xs">Batch {activity.batchId} • {formatDate(activity.createdAt)}</p>
            </div>
            <span class="px-3 py-1 rounded-full text-xs font-semibold border-2 {
              activity.status === 1 ? 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10' : 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10'
            }">
              {activity.status === 1 ? 'Pass' : 'Fail'}
            </span>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
