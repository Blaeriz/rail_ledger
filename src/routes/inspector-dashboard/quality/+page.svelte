<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fetchData, calculateSummaryStats } from '$lib/utils';
  import Chart from '$lib/components/Chart.svelte';

  let user = /** @type {any} */ (null);
  let batchData = /** @type {any[]} */ ([]);
  let reportData = /** @type {any[]} */ ([]);
  let vendorData = /** @type {any[]} */ ([]);
  let isLoading = true;
  let inspectorStats = { 
    completedToday: 0, 
    failedBatches: 0, 
    urgentInspections: 0, 
    avgScore: 0,
    totalInspections: 0,
    passRate: 0,
    efficiency: 0
  };

  // Chart data
  let qualityChartData = {
    labels: ['Pass', 'Fail', 'Pending'],
    datasets: [{ data: [0, 0, 0], backgroundColor: ['#10B981', '#EF4444', '#F59E0B'], borderColor: ['#059669', '#DC2626', '#D97706'], borderWidth: 3 }]
  };

  let performanceChartData = { 
    labels: /** @type {string[]} */ ([]), 
    datasets: [{ 
      label: 'Inspections', 
      data: /** @type {number[]} */ ([]), 
      backgroundColor: 'rgba(139, 92, 246, 0.8)', 
      borderColor: '#8B5CF6', 
      borderWidth: 3, 
      borderRadius: 8 
    }] 
  };


  let efficiencyChartData = { 
    labels: /** @type {string[]} */ ([]), 
    datasets: [
      { 
        label: 'Inspections Completed', 
        data: /** @type {number[]} */ ([]), 
        borderColor: '#8B5CF6', 
        backgroundColor: 'rgba(139, 92, 246, 0.15)', 
        borderWidth: 4, 
        fill: true, 
        tension: 0.4,
        pointRadius: 6
      },
      { 
        label: 'Quality Score', 
        data: /** @type {number[]} */ ([]), 
        borderColor: '#10B981', 
        backgroundColor: 'rgba(16, 185, 129, 0.15)', 
        borderWidth: 4, 
        fill: true, 
        tension: 0.4,
        pointRadius: 6
      }
    ] 
  };

  let failureChartData = { 
    labels: /** @type {string[]} */ ([]), 
    datasets: [{ 
      label: 'Failures', 
      data: /** @type {number[]} */ ([]), 
      backgroundColor: /** @type {string[]} */ ([]), 
      borderColor: /** @type {string[]} */ ([]), 
      borderWidth: 3, 
      borderRadius: 8 
    }] 
  };

  onMount(async () => {
    const userData = sessionStorage.getItem('user');
    if (!userData) return goto('/login');
    
    user = JSON.parse(userData);
    
    try {
      const [batches, reports, vendors] = await Promise.all([
        fetchData('/api/batches'), 
        fetchData('/api/reports'), 
        fetchData('/api/vendors')
      ]);
      
      batchData = batches || [];
      reportData = reports || [];
      vendorData = vendors || [];
      
      calculateInspectorStats();
      updateChartData();
      isLoading = false;
    } catch (error) {
      console.error('Error loading data:', error);
      isLoading = false;
    }
  });

  function calculateInspectorStats() {
    // Completed today (reports created today)
    const today = new Date().toISOString().split('T')[0];
    inspectorStats.completedToday = reportData.filter((/** @type {any} */ r) => 
      r.createdAt?.startsWith(today) || r.created_at?.startsWith(today)
    ).length;
    
    // Failed batches (more comprehensive check)
    inspectorStats.failedBatches = batchData.filter((/** @type {any} */ b) => 
      b.qc_status === 'Fail' || b.qc_status === 'FAIL' || b.qc_status === 'REJECTED'
    ).length;
    
    // Urgent inspections (pending for more than 2 days)
    const twoDaysAgo = new Date();
    twoDaysAgo.setDate(twoDaysAgo.getDate() - 2);
    inspectorStats.urgentInspections = batchData.filter((/** @type {any} */ b) => {
      const prodDate = new Date(b.date_of_production);
      return !b.last_inspection_date && prodDate < twoDaysAgo;
    }).length;
    
    // Average score and pass rate
    const totalBatches = batchData.length;
    const passedBatches = batchData.filter((/** @type {any} */ b) => 
      b.qc_status === 'Pass' || b.qc_status === 'PASS'
    ).length;
    
    inspectorStats.passRate = totalBatches > 0 ? 
      parseFloat(((passedBatches / totalBatches) * 100).toFixed(1)) : 0;
    
    // Average quality score from reports
    const scores = reportData
      .map((/** @type {any} */ r) => r.quality_score || r.overall_score || 0)
      .filter(s => s > 0);
    inspectorStats.avgScore = scores.length > 0 ? 
      parseFloat((scores.reduce((a, b) => a + b, 0) / scores.length).toFixed(1)) : 0;
    
    // Total inspections and efficiency
    inspectorStats.totalInspections = reportData.length;
    inspectorStats.efficiency = inspectorStats.completedToday > 0 ? 
      parseFloat(((inspectorStats.completedToday / (inspectorStats.completedToday + inspectorStats.urgentInspections)) * 100).toFixed(1)) : 0;
  }

  function updateChartData() {
    // Quality Status Distribution
    const passCount = batchData.filter((/** @type {any} */ b) => 
      b.qc_status === 'Pass' || b.qc_status === 'PASS' || 
      b.status === 'Pass' || b.status === 'PASS' ||
      b.inspection_status === 'Pass' || b.inspection_status === 'PASS'
    ).length;
    const failCount = batchData.filter((/** @type {any} */ b) => 
      b.qc_status === 'Fail' || b.qc_status === 'FAIL' || 
      b.status === 'Fail' || b.status === 'FAIL' ||
      b.inspection_status === 'Fail' || b.inspection_status === 'FAIL'
    ).length;
    const pendingCount = batchData.filter((/** @type {any} */ b) => 
      b.qc_status === 'Pending Inspection' || b.qc_status === 'Pending' || b.qc_status === 'PENDING' || b.qc_status === 'PENDING INSPECTION' ||
      b.status === 'Pending Inspection' || b.status === 'Pending' || b.status === 'PENDING' || b.status === 'PENDING INSPECTION' ||
      b.inspection_status === 'Pending Inspection' || b.inspection_status === 'Pending' || b.inspection_status === 'PENDING' || b.inspection_status === 'PENDING INSPECTION'
    ).length;

    if (passCount === 0 && failCount === 0 && pendingCount === 0) {
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

    // Weekly Performance Trends
    const weeklyData = calculateWeeklyPerformance();
    performanceChartData = { 
      labels: weeklyData.labels, 
      datasets: [{ 
        label: 'Inspections', 
        data: weeklyData.data, 
        backgroundColor: 'rgba(139, 92, 246, 0.8)', 
        borderColor: '#8B5CF6', 
        borderWidth: 3, 
        borderRadius: 8
      }] 
    };


    // Weekly Efficiency & Quality Trends
    const efficiencyData = calculateEfficiencyTrends();
    efficiencyChartData = { 
      labels: efficiencyData.labels, 
      datasets: [
        { 
          label: 'Inspections Completed', 
          data: efficiencyData.inspections, 
          borderColor: '#8B5CF6', 
          backgroundColor: 'rgba(139, 92, 246, 0.15)', 
          borderWidth: 4, 
          fill: true, 
          tension: 0.4,
          pointRadius: 6
        },
        { 
          label: 'Quality Score', 
          data: efficiencyData.qualityScores, 
          borderColor: '#10B981', 
          backgroundColor: 'rgba(16, 185, 129, 0.15)', 
          borderWidth: 4, 
          fill: true, 
          tension: 0.4,
          pointRadius: 6
        }
      ] 
    };

    // Failure Analysis by Vendor
    const failureData = calculateFailureByVendor();
    const colors = [
      'rgba(239, 68, 68, 0.8)', 'rgba(245, 158, 11, 0.8)', 'rgba(168, 85, 247, 0.8)',
      'rgba(34, 197, 94, 0.8)', 'rgba(249, 115, 22, 0.8)', 'rgba(59, 130, 246, 0.8)'
    ];
    const borderColors = ['#EF4444', '#F59E0B', '#A855F7', '#22C55E', '#F97316', '#3B82F6'];
    
    failureChartData = { 
      labels: failureData.labels, 
      datasets: [{ 
        label: 'Failures', 
        data: failureData.data, 
        backgroundColor: colors.slice(0, failureData.labels.length), 
        borderColor: borderColors.slice(0, failureData.labels.length), 
        borderWidth: 3, 
        borderRadius: 8
      }] 
    };
  }

  function calculateWeeklyPerformance() {
    const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
    const data = [];
    
    for (let i = 6; i >= 0; i--) {
      const date = new Date();
      date.setDate(date.getDate() - i);
      const dateStr = date.toISOString().split('T')[0];
      
      const dayReports = reportData.filter((/** @type {any} */ r) => 
        r.createdAt?.startsWith(dateStr) || r.created_at?.startsWith(dateStr)
      ).length;
      
      data.push(dayReports);
    }
    
    return { labels: days, data };
  }


  function calculateEfficiencyTrends() {
    const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
    const inspections = [];
    const qualityScores = [];
    
    for (let i = 6; i >= 0; i--) {
      const date = new Date();
      date.setDate(date.getDate() - i);
      const dateStr = date.toISOString().split('T')[0];
      
      // Count inspections completed on this day
      const dayReports = reportData.filter((/** @type {any} */ r) => 
        r.createdAt?.startsWith(dateStr) || r.created_at?.startsWith(dateStr)
      );
      
      inspections.push(dayReports.length);
      
      // Calculate average quality score for this day
      const scores = dayReports
        .map((/** @type {any} */ r) => r.quality_score || r.overall_score || 0)
        .filter(s => s > 0);
      
      const avgScore = scores.length > 0 ? 
        scores.reduce((sum, score) => sum + score, 0) / scores.length : 0;
      
      qualityScores.push(avgScore);
    }
    
    return { labels: days, inspections, qualityScores };
  }

  function calculateFailureByVendor() {
    const vendorFailures = /** @type {Record<string, number>} */ ({});
    
    batchData.forEach((/** @type {any} */ batch) => {
      if (batch.qc_status === 'Fail' || batch.qc_status === 'FAIL') {
        const vendorName = batch.vendor_id || 'Unknown';
        vendorFailures[vendorName] = (vendorFailures[vendorName] || 0) + 1;
      }
    });
    
    const sortedFailures = Object.entries(vendorFailures)
      .sort(([,a], [,b]) => b - a)
      .slice(0, 6);
    
    return {
      labels: sortedFailures.map(([name]) => name),
      data: sortedFailures.map(([,count]) => count)
    };
  }
</script>

<svelte:head>
  <title>Quality Analysis - Rail Ledger</title>
</svelte:head>

<div class="p-6">
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-white mb-2">Quality Analysis</h1>
    <p class="text-gray-400">Comprehensive quality metrics and performance insights</p>
  </div>

  <!-- Summary Cards -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
    <div class="bg-black border border-gray-700 rounded-lg p-6 hover:border-gray-500/50 transition-all duration-300">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-gray-400 text-sm font-medium mb-2">Completed Today</p>
          <p class="text-2xl font-bold text-white">{inspectorStats.completedToday}</p>
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
          <p class="text-gray-400 text-sm font-medium mb-2">Failed Batches</p>
          <p class="text-2xl font-bold text-white">{inspectorStats.failedBatches}</p>
        </div>
        <div class="w-12 h-12 bg-red-500/20 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </div>
      </div>
    </div>

    <div class="bg-black border border-gray-700 rounded-lg p-6 hover:border-gray-500/50 transition-all duration-300">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-gray-400 text-sm font-medium mb-2">Urgent Inspections</p>
          <p class="text-2xl font-bold text-white">{inspectorStats.urgentInspections}</p>
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
          <p class="text-gray-400 text-sm font-medium mb-2">Pass Rate</p>
          <p class="text-2xl font-bold text-white">{inspectorStats.passRate}%</p>
        </div>
        <div class="w-12 h-12 bg-blue-500/20 rounded-lg flex items-center justify-center">
          <svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path>
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
        <p class="text-gray-400">Loading quality analysis...</p>
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
                  tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.9)', titleColor: '#FFFFFF', bodyColor: '#D1D5DB', borderColor: '#374151', borderWidth: 2, cornerRadius: 12, titleFont: { size: 14, weight: 'bold' }, bodyFont: { size: 13 } }
                },
                cutout: '65%',
                animation: { animateRotate: true, animateScale: true, duration: 2000 }
              }}
            />
          </div>
        </div>
      </div>

      <!-- Weekly Performance Trends -->
      <div class="bg-black border border-gray-700 rounded-lg p-6 overflow-hidden min-h-[400px] lg:min-h-[500px]">
        <h3 class="text-lg sm:text-xl font-bold text-white mb-4 sm:mb-6">Weekly Performance Trends</h3>
        <div class="h-72 sm:h-80 lg:h-96 w-full overflow-hidden">
          <div class="w-full h-full">
            <Chart 
              type="bar" 
              data={performanceChartData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
                plugins: { legend: { display: false }, tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.9)', titleColor: '#FFFFFF', bodyColor: '#D1D5DB', borderColor: '#374151', borderWidth: 2, cornerRadius: 12 } },
                scales: {
                  y: { beginAtZero: true, ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, grid: { color: '#374151' }, border: { display: false } },
                  x: { ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, grid: { display: false }, border: { display: false } }
                },
                elements: { bar: { borderRadius: 8 } },
                animation: { duration: 2000 }
              }}
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Full Width Charts -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
      <!-- Weekly Efficiency & Quality Trends -->
      <div class="bg-black border border-gray-700 rounded-lg p-6 overflow-hidden min-h-[400px] lg:min-h-[500px]">
        <h3 class="text-lg sm:text-xl font-bold text-white mb-4 sm:mb-6">Weekly Efficiency & Quality Trends</h3>
        <div class="h-72 sm:h-80 lg:h-96 w-full overflow-hidden">
          <div class="w-full h-full">
            <Chart 
              type="line" 
              data={efficiencyChartData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                  legend: { position: 'top', labels: { color: '#D1D5DB', font: { size: 12, weight: 'bold' }, padding: 20, usePointStyle: true } },
                  tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.9)', titleColor: '#FFFFFF', bodyColor: '#D1D5DB', borderColor: '#374151', borderWidth: 2, cornerRadius: 12 }
                },
                scales: {
                  y: { 
                    beginAtZero: true, 
                    ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, 
                    grid: { color: '#374151' }, 
                    border: { display: false } 
                  },
                  x: { 
                    ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, 
                    grid: { display: false }, 
                    border: { display: false } 
                  }
                },
                elements: { point: { radius: 4, hoverRadius: 6 } },
                animation: { duration: 2000 }
              }}
            />
          </div>
        </div>
      </div>

      <!-- Failure Analysis by Vendor -->
      <div class="bg-black border border-gray-700 rounded-lg p-6 overflow-hidden min-h-[400px] lg:min-h-[500px]">
        <h3 class="text-lg sm:text-xl font-bold text-white mb-4 sm:mb-6">Failure Analysis by Vendor</h3>
        <div class="h-72 sm:h-80 lg:h-96 w-full overflow-hidden">
          <div class="w-full h-full">
            <Chart 
              type="bar" 
              data={failureChartData}
              options={{
                responsive: true,
                maintainAspectRatio: false,
                plugins: { legend: { display: false }, tooltip: { backgroundColor: 'rgba(0, 0, 0, 0.9)', titleColor: '#FFFFFF', bodyColor: '#D1D5DB', borderColor: '#374151', borderWidth: 2, cornerRadius: 12 } },
                scales: {
                  y: { beginAtZero: true, ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, grid: { color: '#374151' }, border: { display: false } },
                  x: { ticks: { color: '#9ca3af', font: { size: 11, weight: 'bold' } }, grid: { display: false }, border: { display: false } }
                },
                elements: { bar: { borderRadius: 8 } },
                animation: { duration: 2000 }
              }}
            />
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>