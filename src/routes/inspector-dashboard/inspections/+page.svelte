<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let user = /** @type {any} */ (null);
  let vendorData = /** @type {any[]} */ ([]);
  let batchData = /** @type {any[]} */ ([]);
  let reportData = /** @type {any[]} */ ([]);
  let isLoading = true;

  // Inspector-specific statistics
  let inspectorStats = {
    pendingInspections: 0,
    completedToday: 0,
    failedBatches: 0,
    urgentInspections: 0
  };

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
    calculateInspectorStats();
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

  function calculateInspectorStats() {
    inspectorStats.pendingInspections = batchData.filter(b => !b.last_inspection_date).length;
    inspectorStats.failedBatches = batchData.filter(b => b.qc_status === 'Fail').length;
    
    // Count reports created today by this inspector
    const today = new Date().toDateString();
    inspectorStats.completedToday = reportData.filter(r => 
      r.inspectorName === user.username && 
      new Date(r.createdAt).toDateString() === today
    ).length;

    // Urgent inspections (batches older than 30 days without inspection)
    const thirtyDaysAgo = new Date();
    thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
    inspectorStats.urgentInspections = batchData.filter(b => 
      !b.last_inspection_date && 
      new Date(b.date_of_production) < thirtyDaysAgo
    ).length;
  }
</script>

<svelte:head>
  <title>Inspection Overview - Rail Ledger</title>
</svelte:head>

<div class="p-4 sm:p-6 lg:p-8">
  <!-- Page Header -->
  <div class="mb-6 sm:mb-8">
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white mb-2">Inspection Overview</h1>
  </div>
  
  <!-- Inspector Stats Cards -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6 mb-6 sm:mb-8">
    <div class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg">
      <div>
        <p class="text-gray-400 text-xs sm:text-sm font-medium mb-2">PENDING INSPECTIONS</p>
        <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">{inspectorStats.pendingInspections}</p>
      </div>
    </div>

    <div class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg">
      <div>
        <p class="text-gray-400 text-xs sm:text-sm font-medium mb-2">COMPLETED TODAY</p>
        <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">{inspectorStats.completedToday}</p>
      </div>
    </div>

    <div class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg">
      <div>
        <p class="text-gray-400 text-xs sm:text-sm font-medium mb-2">FAILED BATCHES</p>
        <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">{inspectorStats.failedBatches}</p>
      </div>
    </div>

    <div class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg">
      <div>
        <p class="text-gray-400 text-xs sm:text-sm font-medium mb-2">URGENT INSPECTIONS</p>
        <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">{inspectorStats.urgentInspections}</p>
      </div>
    </div>
  </div>

  <!-- Quick Actions -->
  <div class="mb-6 sm:mb-8">
    <h2 class="text-xl sm:text-2xl font-bold text-white mb-4 sm:mb-6">Quick Actions</h2>
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
      <button class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg hover:bg-gray-700 transition-all duration-300 text-left group">
        <div>
          <p class="font-bold text-white text-sm sm:text-base mb-1">Start Inspection</p>
          <p class="text-gray-400 text-xs sm:text-sm">Begin new inspection</p>
        </div>
      </button>
      
      <button class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg hover:bg-gray-700 transition-all duration-300 text-left group">
        <div>
          <p class="font-bold text-white text-sm sm:text-base mb-1">Generate Report</p>
          <p class="text-gray-400 text-xs sm:text-sm">Create inspection report</p>
        </div>
      </button>
      
      <button class="bg-gray-800 border border-gray-700 p-4 sm:p-6 rounded-lg hover:bg-gray-700 transition-all duration-300 text-left group">
        <div>
          <p class="font-bold text-white text-sm sm:text-base mb-1">View Analytics</p>
          <p class="text-gray-400 text-xs sm:text-sm">Quality insights</p>
        </div>
      </button>
    </div>
  </div>
</div>
