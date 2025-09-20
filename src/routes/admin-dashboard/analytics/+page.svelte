<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fetchData, calculateSummaryStats } from '$lib/utils';
  import Chart from '$lib/components/Chart.svelte';

  let user = /** @type {any} */ (null);
  let vendorData = /** @type {any[]} */ ([]);
  let batchData = /** @type {any[]} */ ([]);
  let reportData = /** @type {any[]} */ ([]);
  let summaryStats = { totalVendors: 0, totalBatches: 0, pendingInspections: 0, failedBatches: 0 };
  let isLoading = true;

  // Chart data
  let qualityChartData = {
    labels: ['Pass', 'Fail', 'Pending'],
    datasets: [{ data: [0, 0, 0], backgroundColor: ['#10B981', '#EF4444', '#F59E0B'], borderColor: ['#059669', '#DC2626', '#D97706'], borderWidth: 2 }]
  };

  let vendorChartData = { labels: /** @type {string[]} */ ([]), datasets: [{ label: 'Batches', data: /** @type {number[]} */ ([]), backgroundColor: 'rgba(139, 92, 246, 0.8)', borderColor: '#8B5CF6', borderWidth: 2, borderRadius: 4 }] };

  let monthlyTrendsData = { labels: /** @type {string[]} */ ([]), datasets: [{ label: 'Passed', data: /** @type {number[]} */ ([]), borderColor: '#10B981', backgroundColor: 'rgba(16, 185, 129, 0.1)', borderWidth: 3, fill: true, tension: 0.4 }, { label: 'Failed', data: /** @type {number[]} */ ([]), borderColor: '#EF4444', backgroundColor: 'rgba(239, 68, 68, 0.1)', borderWidth: 3, fill: true, tension: 0.4 }] };


  onMount(async () => {
    const userData = sessionStorage.getItem('user');
    if (!userData) return goto('/login');
    
    user = JSON.parse(userData);
    
    try {
      const [vendors, batches, reports] = await Promise.all([fetchData('/api/vendors'), fetchData('/api/batches'), fetchData('/api/reports')]);
      vendorData = vendors || [];
      batchData = batches || [];
      reportData = reports || [];
      summaryStats = calculateSummaryStats(vendorData, batchData, reportData);
      updateChartData();
      isLoading = false;
    } catch (error) {
      console.error('Error loading data:', error);
      isLoading = false;
    }
  });

  function updateChartData() {
    // Quality chart - handle different status formats with enhanced colors
    const passCount = batchData.filter(b => b.qc_status === 'Pass' || b.qc_status === 'PASS').length;
    const failCount = batchData.filter(b => b.qc_status === 'Fail' || b.qc_status === 'FAIL').length;
    const pendingCount = batchData.filter(b => b.qc_status === 'Pending' || b.qc_status === 'PENDING' || b.qc_status === 'PENDING INSPECTION').length;
    
    // Ensure we have at least some data for the chart
    const total = passCount + failCount + pendingCount;
    if (total === 0) {
      qualityChartData = { 
        labels: ['No Data'], 
        datasets: [{ 
          data: [1], 
          backgroundColor: ['#6B7280'], 
          borderColor: ['#4B5563'], 
          borderWidth: 3
        }] 
      };
    } else {
      qualityChartData = { 
        labels: ['Pass', 'Fail', 'Pending'], 
        datasets: [{ 
          data: [passCount, failCount, pendingCount], 
          backgroundColor: ['#10B981', '#EF4444', '#F59E0B'], 
          borderColor: ['#059669', '#DC2626', '#D97706'], 
          borderWidth: 3
        }] 
      };
    }

    // Vendor chart - top 8 vendors with enhanced colors and real data
    const vendorCounts = vendorData.map(v => ({ 
      name: v.city || v.vendor_name || 'Unknown', 
      count: batchData.filter(b => b.vendor_id === v.vendor_id).length,
      totalBatches: v.no_of_batches || 0
    })).sort((a, b) => b.count - a.count).slice(0, 8);
    
    if (vendorCounts.length === 0 || vendorCounts.every(v => v.count === 0)) {
      vendorChartData = {
        labels: ['No Data'],
        datasets: [{
          label: 'Batches',
          data: [1],
          backgroundColor: 'rgba(107, 114, 128, 0.8)',
          borderColor: '#6B7280',
          borderWidth: 3,
          borderRadius: 8
        }]
      };
    } else {
      const barColors = vendorCounts.map((_, index) => {
        const colorPalette = [
          'rgba(139, 92, 246, 0.8)', 'rgba(59, 130, 246, 0.8)', 'rgba(16, 185, 129, 0.8)',
          'rgba(245, 158, 11, 0.8)', 'rgba(239, 68, 68, 0.8)', 'rgba(168, 85, 247, 0.8)',
          'rgba(34, 197, 94, 0.8)', 'rgba(249, 115, 22, 0.8)'
        ];
        return colorPalette[index % colorPalette.length];
      });
      
      const barBorderColors = vendorCounts.map((_, index) => {
        const borderPalette = [
          '#8B5CF6', '#3B82F6', '#10B981', '#F59E0B', '#EF4444', '#A855F7', '#22C55E', '#F97316'
        ];
        return borderPalette[index % borderPalette.length];
      });
      
      vendorChartData = { 
        labels: /** @type {string[]} */ (vendorCounts.map(v => v.name)), 
        datasets: [{ 
          label: 'Batches', 
          data: /** @type {number[]} */ (vendorCounts.map(v => v.count)), 
          backgroundColor: /** @type {any} */ (barColors), 
          borderColor: /** @type {any} */ (barBorderColors), 
          borderWidth: 3, 
          borderRadius: 8
        }] 
      };
    }

    // Monthly trends data with enhanced styling
    const monthlyData = calculateMonthlyTrends();
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

      const passed = monthBatches.filter((/** @type {any} */ batch) => batch.qc_status === 'Pass' || batch.qc_status === 'PASS').length;
      const failed = monthBatches.filter((/** @type {any} */ batch) => batch.qc_status === 'Fail' || batch.qc_status === 'FAIL').length;
      
      passedData.push(passed);
      failedData.push(failed);
    }

    return { labels: months, passedData, failedData };
  }

</script>

<div class="p-6">
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-white mb-2">Analytics Dashboard</h1>
    <p class="text-gray-400">Comprehensive insights and performance metrics</p>
  </div>

  <!-- Summary Cards -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
    <div class="bg-black border border-gray-700 rounded-lg p-6 hover:border-gray-500/50 transition-all duration-300">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-gray-400 text-sm font-medium mb-2">Total Vendors</p>
          <p class="text-2xl font-bold text-white">{summaryStats.totalVendors}</p>
        </div>
        <div class="w-12 h-12 bg-blue-500/20 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
          </svg>
        </div>
      </div>
    </div>

    <div class="bg-black border border-gray-700 rounded-lg p-6 hover:border-gray-500/50 transition-all duration-300">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-gray-400 text-sm font-medium mb-2">Total Batches</p>
          <p class="text-2xl font-bold text-white">{summaryStats.totalBatches}</p>
        </div>
        <div class="w-12 h-12 bg-green-500/20 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
        </div>
      </div>
    </div>

    <div class="bg-black border border-gray-700 rounded-lg p-6 hover:border-gray-500/50 transition-all duration-300">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-gray-400 text-sm font-medium mb-2">Pending Inspections</p>
          <p class="text-2xl font-bold text-white">{summaryStats.pendingInspections}</p>
        </div>
        <div class="w-12 h-12 bg-yellow-500/20 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
        </div>
      </div>
    </div>

    <div class="bg-black border border-gray-700 rounded-lg p-6 hover:border-gray-500/50 transition-all duration-300">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-gray-400 text-sm font-medium mb-2">Failed Batches</p>
          <p class="text-2xl font-bold text-white">{summaryStats.failedBatches}</p>
        </div>
        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </div>
      </div>
    </div>
  </div>

  <!-- Loading State -->
  {#if isLoading}
    <div class="flex items-center justify-center py-12">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500 mx-auto mb-4"></div>
        <p class="text-gray-400">Loading charts...</p>
      </div>
    </div>
  {:else}
    <!-- Charts Section -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
    <!-- Quality Status Distribution -->
    <div class="bg-black border border-gray-700 rounded-lg p-6 overflow-hidden min-h-[400px] lg:min-h-[500px]">
      <h3 class="text-lg sm:text-xl font-bold text-white mb-4 sm:mb-6">Quality Status Distribution</h3>
      <div class="h-72 sm:h-80 lg:h-96 w-full overflow-hidden">
        <div class="w-full h-full">
          <Chart 
            type="doughnut" 
            data={qualityChartData}
            options={{
              responsive: true,
              maintainAspectRatio: false,
              plugins: {
                legend: { position: 'bottom', labels: { color: '#D1D5DB', font: { size: 12, weight: 'bold' }, padding: 20, usePointStyle: true } },
                tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.8)', titleColor: '#FFFFFF', bodyColor: '#D1D5DB', borderColor: '#374151', borderWidth: 1, cornerRadius: 8 }
              },
              cutout: '60%'
            }}
          />
        </div>
      </div>
    </div>

    <!-- Top Vendor Performance -->
    <div class="bg-black border border-gray-700 rounded-lg p-6 overflow-hidden min-h-[400px] lg:min-h-[500px]">
      <h3 class="text-lg sm:text-xl font-bold text-white mb-4 sm:mb-6">Top Vendor Performance</h3>
      <div class="h-72 sm:h-80 lg:h-96 w-full overflow-hidden">
        <div class="w-full h-full">
          <Chart 
            type="bar" 
            data={vendorChartData}
            options={{
              responsive: true,
              maintainAspectRatio: false,
              plugins: { legend: { display: false }, tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.8)', titleColor: '#FFFFFF', bodyColor: '#D1D5DB', borderColor: '#374151', borderWidth: 1, cornerRadius: 8 } },
              scales: {
                y: { beginAtZero: true, ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, grid: { color: '#374151' }, border: { display: false } },
                x: { ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, grid: { display: false }, border: { display: false } }
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
    <div class="bg-black border border-gray-700 rounded-lg p-6 overflow-hidden min-h-[400px] lg:min-h-[500px]">
      <h3 class="text-lg sm:text-xl font-bold text-white mb-4 sm:mb-6">Monthly Quality Trends</h3>
      <div class="h-72 sm:h-80 lg:h-96 w-full overflow-hidden">
        <div class="w-full h-full">
          <Chart 
            type="line" 
            data={monthlyTrendsData}
            options={{
              responsive: true,
              maintainAspectRatio: false,
              plugins: {
                legend: { position: 'top', labels: { color: '#D1D5DB', font: { size: 12, weight: 'bold' }, padding: 20, usePointStyle: true } },
                tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.8)', titleColor: '#FFFFFF', bodyColor: '#D1D5DB', borderColor: '#374151', borderWidth: 1, cornerRadius: 8 }
              },
              scales: {
                y: { beginAtZero: true, ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, grid: { color: '#374151' }, border: { display: false } },
                x: { ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, grid: { display: false }, border: { display: false } }
              },
              elements: { point: { radius: 4, hoverRadius: 6 } },
              animation: { duration: 2000 }
            }}
          />
        </div>
      </div>
    </div>
  </div>
  {/if}
</div>