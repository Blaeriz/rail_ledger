<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fetchData, calculateSummaryStats } from '$lib/utils';
  import DataTable from '$lib/components/DataTable.svelte';

  let user = /** @type {any} */ (null);
  let vendorData = /** @type {any[]} */ ([]);
  let batchData = /** @type {any[]} */ ([]);
  let reportData = /** @type {any[]} */ ([]);
  let summaryStats = { totalVendors: 0, totalBatches: 0, pendingInspections: 0, failedBatches: 0 };

  const tableColumns = [
    { key: 'batch_id', label: 'BATCH ID' },
    { key: 'vendor_id', label: 'VENDOR' },
    { key: 'qc_status', label: 'STATUS', render: (/** @type {any} */ item) => `<span class="px-3 py-1 rounded-full text-xs font-semibold border-2 ${item.qc_status === 'Pass' ? 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10' : item.qc_status === 'Fail' ? 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10' : 'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10'}">${item.qc_status}</span>` },
    { key: 'date_of_production', label: 'PRODUCTION DATE', render: (/** @type {any} */ item) => new Date(item.date_of_production).toLocaleDateString() },
    { key: 'expiry_date', label: 'EXPIRY DATE', render: (/** @type {any} */ item) => new Date(item.expiry_date).toLocaleDateString() }
  ];

  onMount(async () => {
    const userData = sessionStorage.getItem('user');
    if (!userData) return goto('/login');
    
    user = JSON.parse(userData);
    
    try {
      const [vendors, batches, reports] = await Promise.all([fetchData('/api/vendors'), fetchData('/api/batches'), fetchData('/api/reports')]);
      vendorData = vendors;
      batchData = batches;
      reportData = reports;
      summaryStats = calculateSummaryStats(vendorData, batchData, reportData);
    } catch (error) {
      console.error('Error loading data:', error);
    }
  });
</script>

<div class="p-4 sm:p-6 lg:p-8">
  <!-- Page Header -->
  <div class="mb-6 sm:mb-8">
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white mb-2">Admin Dashboard</h1>
  </div>
  
  <!-- Summary Cards -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6 mb-6 sm:mb-8">
    <div class="bg-black border border-gray-700 p-4 sm:p-6 rounded-lg hover:border-purple-500/50 hover:shadow-2xl hover:shadow-purple-500/10 transition-all duration-500 group animate-fade-in interactive relative overflow-hidden" style="animation-delay: 0ms">
      <div class="absolute inset-0 bg-gradient-to-br from-purple-500/5 to-blue-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      <div class="relative z-10">
        <div>
          <p class="text-gray-500 text-xs sm:text-sm font-medium mb-2 uppercase tracking-wide">TOTAL VENDORS</p>
          <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white group-hover:text-purple-300 transition-colors duration-300">{summaryStats.totalVendors}</p>
        </div>
      </div>
    </div>

    <div class="bg-black border border-gray-700 p-4 sm:p-6 rounded-lg hover:border-blue-500/50 hover:shadow-2xl hover:shadow-blue-500/10 transition-all duration-500 group animate-fade-in interactive relative overflow-hidden" style="animation-delay: 100ms">
      <div class="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-cyan-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      <div class="relative z-10">
        <div>
          <p class="text-gray-500 text-xs sm:text-sm font-medium mb-2 uppercase tracking-wide">ACTIVE BATCHES</p>
          <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white group-hover:text-blue-300 transition-colors duration-300">{summaryStats.totalBatches}</p>
        </div>
      </div>
    </div>

    <div class="bg-black border border-gray-700 p-4 sm:p-6 rounded-lg hover:border-yellow-500/50 hover:shadow-2xl hover:shadow-yellow-500/10 transition-all duration-500 group animate-fade-in interactive relative overflow-hidden" style="animation-delay: 200ms">
      <div class="absolute inset-0 bg-gradient-to-br from-yellow-500/5 to-orange-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      <div class="relative z-10">
        <div>
          <p class="text-gray-500 text-xs sm:text-sm font-medium mb-2 uppercase tracking-wide">PENDING INSPECTIONS</p>
          <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white group-hover:text-yellow-300 transition-colors duration-300">{summaryStats.pendingInspections}</p>
        </div>
      </div>
    </div>

    <div class="bg-black border border-gray-700 p-4 sm:p-6 rounded-lg hover:border-red-500/50 hover:shadow-2xl hover:shadow-red-500/10 transition-all duration-500 group animate-fade-in interactive relative overflow-hidden" style="animation-delay: 300ms">
      <div class="absolute inset-0 bg-gradient-to-br from-red-500/5 to-pink-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      <div class="relative z-10">
        <div>
          <p class="text-gray-500 text-xs sm:text-sm font-medium mb-2 uppercase tracking-wide">FAILURES THIS MONTH</p>
          <p class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white group-hover:text-red-300 transition-colors duration-300">{summaryStats.failedBatches}</p>
        </div>
      </div>
    </div>
  </div>

  <!-- Action Buttons -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4 mb-6 sm:mb-8">
    <button class="bg-gray-800 hover:bg-gray-700 text-white px-6 py-3 rounded-lg font-medium transition-all duration-500 text-sm flex items-center justify-center space-x-2 border border-purple-500/30 hover:border-purple-400/50 hover:shadow-2xl hover:shadow-purple-500/30 active:bg-gray-600 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/40 group animate-slide-in-left interactive relative overflow-hidden" style="animation-delay: 400ms">
      <div class="absolute inset-0 bg-gradient-to-r from-purple-500/10 to-blue-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      <div class="relative z-10 flex items-center space-x-2">
        <svg class="w-4 h-4 group-hover:scale-110 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
        </svg>
        <span class="group-hover:translate-x-1 transition-transform duration-300">Add Vendor</span>
      </div>
    </button>
    <button class="bg-gray-800 hover:bg-gray-700 text-white px-6 py-3 rounded-lg font-medium transition-all duration-500 text-sm flex items-center justify-center space-x-2 border border-blue-500/30 hover:border-blue-400/50 hover:shadow-2xl hover:shadow-blue-500/30 active:bg-gray-600 active:border-blue-400/60 active:shadow-xl active:shadow-blue-500/40 group animate-slide-in-left interactive relative overflow-hidden" style="animation-delay: 500ms">
      <div class="absolute inset-0 bg-gradient-to-r from-blue-500/10 to-cyan-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      <div class="relative z-10 flex items-center space-x-2">
        <svg class="w-4 h-4 group-hover:scale-110 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
        </svg>
        <span class="group-hover:translate-x-1 transition-transform duration-300">Generate Report</span>
      </div>
    </button>
    <button class="bg-gray-800 hover:bg-gray-700 text-white px-6 py-3 rounded-lg font-medium transition-all duration-500 text-sm flex items-center justify-center space-x-2 border border-green-500/30 hover:border-green-400/50 hover:shadow-2xl hover:shadow-green-500/30 active:bg-gray-600 active:border-green-400/60 active:shadow-xl active:shadow-green-500/40 group animate-slide-in-left interactive relative overflow-hidden" style="animation-delay: 600ms">
      <div class="absolute inset-0 bg-gradient-to-r from-green-500/10 to-emerald-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      <div class="relative z-10 flex items-center space-x-2">
        <svg class="w-4 h-4 group-hover:scale-110 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
        </svg>
        <span class="group-hover:translate-x-1 transition-transform duration-300">Settings</span>
      </div>
    </button>
  </div>

  <!-- Records Table -->
  <div class="mb-6">
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-4">
      <span class="text-gray-400 text-sm">{batchData.length} records</span>
      <button class="bg-gray-800 hover:bg-gray-700 text-white px-6 py-3 rounded-lg text-sm font-medium transition-all duration-500 flex items-center space-x-2 border border-yellow-500/30 hover:border-yellow-400/50 hover:shadow-2xl hover:shadow-yellow-500/30 active:bg-gray-600 active:border-yellow-400/60 active:shadow-xl active:shadow-yellow-500/40 group animate-slide-in-right interactive relative overflow-hidden">
        <div class="absolute inset-0 bg-gradient-to-r from-yellow-500/10 to-orange-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
        <div class="relative z-10 flex items-center space-x-2">
          <span class="group-hover:translate-x-1 transition-transform duration-300">View All Batches</span>
          <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
          </svg>
        </div>
      </button>
    </div>
    
    <DataTable 
      data={batchData.slice(0, 5)} 
      columns={tableColumns}
      searchable={true}
      searchPlaceholder="Search records..."
    />
  </div>
</div>