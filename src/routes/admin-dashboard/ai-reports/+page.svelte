<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import Icon from '$lib/components/Icon.svelte';
  import Chart from '$lib/components/Chart.svelte';

  let isLoading = true;
  let aiReports = /** @type {any[]} */ ([]);
  let selectedReport = /** @type {any} */ (null);
  let showReportModal = false;
  let searchQuery = '';
  let filterStatus = 'all';
  let isGenerating = false;
  let generateType = 'quality';

  onMount(async () => {
    await loadReports();
  });

  async function loadReports() {
    try {
      const response = await fetch('/api/ai-reports');
      const data = await response.json();
      aiReports = data.reports || [];
    } catch (error) {
      console.error('Error loading reports:', error);
      aiReports = [];
    } finally {
      isLoading = false;
    }
  }

  async function generateReport() {
    isGenerating = true;
    try {
      const response = await fetch('/api/ai-reports', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          type: generateType
        })
      });

      const result = await response.json();
      if (result.report) {
        aiReports = [result.report, ...aiReports];
      }
    } catch (error) {
      console.error('Error generating report:', error);
      alert('Failed to generate AI report. Please try again.');
    } finally {
      isGenerating = false;
    }
  }

  function getStatusColor(/** @type {string} */ status) {
    switch (status) {
      case 'completed': return 'text-green-400 bg-green-400/10 border-green-400/20';
      case 'processing': return 'text-yellow-400 bg-yellow-400/10 border-yellow-400/20';
      case 'pending': return 'text-gray-400 bg-gray-400/10 border-gray-400/20';
      default: return 'text-gray-400 bg-gray-400/10 border-gray-400/20';
    }
  }

  function getStatusText(/** @type {string} */ status) {
    switch (status) {
      case 'completed': return 'Completed';
      case 'processing': return 'Processing';
      case 'pending': return 'Pending';
      default: return 'Unknown';
    }
  }

  function formatDate(/** @type {string} */ dateString) {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }

  function openReport(/** @type {any} */ report) {
    selectedReport = report;
    showReportModal = true;
  }

  function closeReport() {
    selectedReport = null;
    showReportModal = false;
  }

  function filteredReports() {
    let filtered = /** @type {any[]} */ (aiReports);
    
    if (searchQuery) {
      filtered = filtered.filter(report => 
        report.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        report.type.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }
    
    if (filterStatus !== 'all') {
      filtered = filtered.filter(report => report.status === filterStatus);
    }
    
    return filtered;
  }
</script>

<svelte:head>
  <title>AI Reports - Admin Dashboard</title>
</svelte:head>

<div class="p-6 space-y-6">
  <!-- Header -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
    <div>
      <h1 class="text-2xl sm:text-3xl font-bold text-white mb-2">AI Reports</h1>
      <p class="text-gray-400">AI-powered insights and analysis for rail operations</p>
    </div>
    <div class="mt-4 sm:mt-0 flex flex-col sm:flex-row gap-3">
      <select
        bind:value={generateType}
        class="px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
      >
        <option value="quality">Quality Analysis</option>
        <option value="maintenance">Predictive Maintenance</option>
        <option value="vendor">Vendor Performance</option>
        <option value="safety">Safety Risk Assessment</option>
      </select>
      <button 
        on:click={generateReport}
        disabled={isGenerating}
        class="bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-6 py-3 rounded-lg font-medium transition-all duration-300 border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20"
      >
        {#if isGenerating}
          <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white inline mr-2"></div>
          Generating...
        {:else}
          <Icon name="plus" size="w-4 h-4" className="inline mr-2" />
          Generate AI Report
        {/if}
      </button>
    </div>
  </div>

  <!-- Filters -->
  <div class="flex flex-col sm:flex-row gap-4">
    <div class="flex-1">
      <div class="relative">
        <Icon name="search" size="w-5 h-5" className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" />
        <input
          type="text"
          placeholder="Search reports..."
          bind:value={searchQuery}
          class="w-full pl-10 pr-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
        />
      </div>
    </div>
    <div class="sm:w-48">
      <select
        bind:value={filterStatus}
        class="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
      >
        <option value="all">All Status</option>
        <option value="completed">Completed</option>
        <option value="processing">Processing</option>
        <option value="pending">Pending</option>
      </select>
    </div>
  </div>

  <!-- Reports Grid -->
  {#if isLoading}
    <div class="flex items-center justify-center h-64">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500"></div>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each filteredReports() as report}
        <div 
          class="bg-gray-800 border border-gray-700 rounded-lg p-6 hover:border-gray-600 transition-all duration-300 cursor-pointer group" 
          role="button"
          tabindex="0"
          on:click={() => openReport(report)}
          on:keydown={(e) => e.key === 'Enter' && openReport(report)}
        >
          <div class="flex items-start justify-between mb-4">
            <div class="flex-1">
              <h3 class="text-white font-semibold text-lg mb-2 group-hover:text-purple-400 transition-colors">
                {report.title}
              </h3>
              <p class="text-gray-400 text-sm mb-3">{report.summary}</p>
            </div>
            <span class="px-3 py-1 rounded-full text-xs font-medium border {getStatusColor(report.status)}">
              {getStatusText(report.status)}
            </span>
          </div>
          
          <div class="space-y-2 mb-4">
            <div class="flex items-center text-sm text-gray-400">
              <Icon name="clock" size="w-4 h-4" className="mr-2" />
              {formatDate(report.generatedAt)}
            </div>
            <div class="flex items-center text-sm text-gray-400">
              <Icon name="chart" size="w-4 h-4" className="mr-2" />
              {report.type}
            </div>
            {#if report.confidence > 0}
              <div class="flex items-center text-sm text-gray-400">
                <Icon name="ai" size="w-4 h-4" className="mr-2" />
                {report.confidence}% confidence
              </div>
            {/if}
          </div>
          
          <div class="flex items-center text-purple-400 text-sm font-medium group-hover:text-purple-300 transition-colors">
            View Details
            <Icon name="plus" size="w-4 h-4" className="ml-2 transform group-hover:rotate-45 transition-transform" />
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Report Modal -->
  {#if showReportModal && selectedReport}
    <div 
      class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4" 
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      on:click={closeReport}
      on:keydown={(e) => e.key === 'Escape' && closeReport()}
    >
      <div 
        class="bg-gray-800 border border-gray-700 rounded-lg max-w-4xl w-full max-h-[90vh] overflow-y-auto" 
        role="document"
      >
        <div class="p-6 border-b border-gray-700">
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-bold text-white">{selectedReport.title}</h2>
            <button on:click={closeReport} class="text-gray-400 hover:text-white transition-colors">
              <Icon name="x" size="w-6 h-6" />
            </button>
          </div>
        </div>
        
        <div class="p-6 space-y-6">
          <!-- Report Info -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="bg-gray-700/50 rounded-lg p-4">
              <div class="text-sm text-gray-400 mb-1">Status</div>
              <div class="text-white font-medium">{getStatusText(selectedReport.status)}</div>
            </div>
            <div class="bg-gray-700/50 rounded-lg p-4">
              <div class="text-sm text-gray-400 mb-1">Type</div>
              <div class="text-white font-medium">{selectedReport.type}</div>
            </div>
            <div class="bg-gray-700/50 rounded-lg p-4">
              <div class="text-sm text-gray-400 mb-1">Confidence</div>
              <div class="text-white font-medium">{selectedReport.confidence}%</div>
            </div>
          </div>

          <!-- Summary -->
          <div>
            <h3 class="text-lg font-semibold text-white mb-3">Summary</h3>
            <p class="text-gray-300">{selectedReport.summary}</p>
          </div>

          <!-- Insights -->
          {#if selectedReport.insights && selectedReport.insights.length > 0}
            <div>
              <h3 class="text-lg font-semibold text-white mb-3">Key Insights</h3>
              <ul class="space-y-2">
                {#each selectedReport.insights as insight}
                  <li class="flex items-start text-gray-300">
                    <Icon name="check" size="w-5 h-5" className="text-green-400 mr-3 mt-0.5 flex-shrink-0" />
                    {insight}
                  </li>
                {/each}
              </ul>
            </div>
          {/if}

          <!-- Recommendations -->
          {#if selectedReport.recommendations && selectedReport.recommendations.length > 0}
            <div>
              <h3 class="text-lg font-semibold text-white mb-3">Recommendations</h3>
              <ul class="space-y-2">
                {#each selectedReport.recommendations as recommendation}
                  <li class="flex items-start text-gray-300">
                    <Icon name="plus" size="w-5 h-5" className="text-blue-400 mr-3 mt-0.5 flex-shrink-0" />
                    {recommendation}
                  </li>
                {/each}
              </ul>
            </div>
          {/if}

          <!-- Visualizations -->
          {#if selectedReport.visualizations && selectedReport.visualizations.length > 0}
            <div>
              <h3 class="text-lg font-semibold text-white mb-3">Visualizations</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                {#each selectedReport.visualizations as visualization}
                  <Chart 
                    data={visualization.data} 
                    type={visualization.type}
                    title={visualization.title}
                    description={visualization.description}
                  />
                {/each}
              </div>
            </div>
          {/if}
        </div>
        
        <div class="p-6 border-t border-gray-700 flex justify-end space-x-3">
          <button on:click={closeReport} class="px-6 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors">
            Close
          </button>
          <button class="px-6 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-colors">
            Export Report
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
