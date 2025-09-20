<script>
  // @ts-nocheck
  import { onMount } from 'svelte';
  import { pipeline } from '@xenova/transformers';

  let status = "Loading AI model...";
  let generator = null;
  let loading = true;
  let error = null;

  // Data from APIs
  let vendorData = [];
  let batchData = [];
  let reportData = [];
  let aiOutput = "";
  let selectedRole = "Admin";
  
  // AI Generated Insights
  let aiInsights = { summary: "", criticalAlerts: [], vendorPerformance: [], inspectionPending: [], recommendations: [] };

  // Summary statistics
  let summaryStats = { totalVendors: 0, totalBatches: 0, pendingInspections: 0, failedBatches: 0 };

  // Load AI model
  async function loadModel() {
    try {
      generator = await pipeline("text2text-generation", "Xenova/flan-t5-small");
      status = "AI Model Ready";
    } catch (err) {
      status = "AI Model Failed to Load";
      error = err?.message || 'Unknown error occurred';
    }
  }

  // Fetch data from APIs
  async function fetchData() {
    try {
      const [vendorResponse, batchResponse, reportResponse] = await Promise.all([
        fetch('/api/vendors'),
        fetch('/api/batches'),
        fetch('/api/reports')
      ]);

      if (vendorResponse.ok) vendorData = await vendorResponse.json();
      if (batchResponse.ok) batchData = await batchResponse.json();
      if (reportResponse.ok) reportData = await reportResponse.json();

      calculateSummaryStats();
    } catch (err) {
      console.error('Error fetching data:', err);
      error = 'Failed to fetch data from APIs';
    }
  }

  // Calculate summary statistics
  function calculateSummaryStats() {
    summaryStats.totalVendors = vendorData.length;
    summaryStats.totalBatches = batchData.length;
    summaryStats.pendingInspections = batchData.filter(batch => !batch.last_inspection_date).length;
    summaryStats.failedBatches = batchData.filter(batch => batch.qc_status === 'Fail').length;
  }

  // Build AI prompt based on role and data
  function buildPrompt(role, vendorData, batchData, reportData) {
    const stats = summaryStats;
    
    const vendorSummary = vendorData.map(v => ({
      vendor_id: v.vendor_id || 'Unknown',
      state: v.state || 'Unknown',
      total_batches: v.no_of_batches || 0,
      city: v.city || 'Unknown'
    }));
    
    const batchSummary = batchData.map(b => ({
      batch_id: b.batch_id || 'Unknown',
      vendor_id: b.vendor_id || 'Unknown',
      qc_status: b.qc_status || 'Unknown',
      production_date: b.date_of_production || 'Unknown',
      location: b.fitment_location || 'Unknown'
    }));
    
    const dataContext = `
    VENDORS (${vendorData.length}): ${JSON.stringify(vendorSummary.slice(0, 10))}
    BATCHES (${batchData.length}): ${JSON.stringify(batchSummary.slice(0, 10))}
    STATS: ${stats.totalVendors} vendors, ${stats.totalBatches} batches, ${stats.pendingInspections} pending inspections, ${stats.failedBatches} failed batches
    `;
    
    const prompts = {
      Admin: `{"summary":"Railway system with ${stats.totalVendors} vendors and ${stats.totalBatches} batches","critical_alerts":["${stats.pendingInspections} pending inspections","${stats.failedBatches} failed batches"],"vendor_performance":[{"vendor_id":"V001","state":"Delhi","total_batches":10,"failed_batches":2,"failure_rate":"20%"}],"inspection_pending":[{"batch_id":"B001","vendor_id":"V001","days_since_production":35}],"recommendations":["Schedule inspections","Review quality"]}`,
      Inspector: `{"inspection_summary":"${stats.pendingInspections} pending and ${stats.failedBatches} failed","urgent_inspections":[{"batch_id":"B001","vendor_id":"V001","days_overdue":15,"priority":"HIGH"}],"failed_batches":[{"batch_id":"B002","vendor_id":"V001","qc_status":"Fail","last_inspection":"2024-01-15"}],"inspection_schedule":[{"batch_id":"B003","vendor_id":"V002","scheduled_date":"2024-01-20","location":"Delhi"}],"quality_trends":"Analysis needed"}`,
      Viewer: `{"overview":"${stats.totalVendors} vendors managing ${stats.totalBatches} batches","vendor_summary":[{"vendor_id":"V001","state":"Delhi","no_of_batches":10,"status":"Active"}],"batch_status":{"total":${stats.totalBatches},"passed":${stats.totalBatches - stats.failedBatches},"failed":${stats.failedBatches},"pending":${stats.pendingInspections}},"geographic_distribution":[{"state":"Delhi","vendors":3,"batches":25}],"recent_activity":"System monitoring"}`
    };
    
    return prompts[role] || prompts.Viewer;
  }

  // Generate AI report
  async function generateReport() {
    if (!generator) {
      status = "AI Model not ready yet...";
      return;
    }
    
    status = "Generating AI report...";
    loading = true;
    
    try {
      const prompt = buildPrompt(selectedRole, vendorData, batchData, reportData);
      const result = await generator(prompt, { max_length: 1000, temperature: 0.7 });
      aiOutput = result[0].generated_text;
      
      // Parse AI output
      try {
        const parsed = JSON.parse(aiOutput);
        aiInsights = {
          summary: parsed.summary || "AI analysis completed",
          criticalAlerts: parsed.critical_alerts || parsed.criticalAlerts || [],
          vendorPerformance: parsed.vendor_performance || parsed.vendorPerformance || [],
          inspectionPending: parsed.inspection_pending || parsed.inspectionPending || [],
          recommendations: parsed.recommendations || []
        };
      } catch (parseError) {
        // Fallback if JSON parsing fails
        aiInsights.summary = aiOutput;
        aiInsights.criticalAlerts = [`${summaryStats.pendingInspections} pending inspections`, `${summaryStats.failedBatches} failed batches`];
        aiInsights.recommendations = ["Review pending inspections", "Address failed batches"];
      }
      
      status = "AI Report Generated Successfully";
    } catch (err) {
      status = "AI Report Generation Failed";
      error = err?.message || 'Unknown error occurred';
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    await loadModel();
    await fetchData();
  });
</script>

<svelte:head>
  <title>AI-Powered Reports - Rail Ledger</title>
</svelte:head>

<div class="min-h-screen bg-black text-white">
  <!-- Header -->
  <div class="bg-gradient-to-r from-purple-900/20 to-blue-900/20 backdrop-blur-2xl border-b border-purple-500/30">
    <div class="container mx-auto px-6 py-8">
      <div class="text-center">
        <h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold mb-4 bg-gradient-to-r from-white via-purple-200 to-blue-200 bg-clip-text text-transparent">
          AI-Powered Analytics
        </h1>
        <p class="text-xl text-gray-300 max-w-3xl mx-auto">
          Advanced AI analysis of railway infrastructure data with intelligent insights and recommendations
        </p>
      </div>
    </div>
  </div>

  <!-- Main Content -->
  <div class="container mx-auto px-6 py-12">
    <!-- Status and Controls -->
    <div class="mb-12">
      <div class="bg-gray-900/40 backdrop-blur-2xl border border-gray-700/40 rounded-3xl p-8">
        <div class="flex flex-col lg:flex-row items-center justify-between gap-6">
          <div class="flex-1">
            <h2 class="text-2xl font-bold text-white mb-2">AI Analysis Status</h2>
            <p class="text-gray-300 mb-4">{status}</p>
            {#if error}
              <div class="bg-red-900/20 border border-red-500/30 text-red-300 px-4 py-3 rounded-xl text-sm">
                {error}
              </div>
            {/if}
          </div>
          
          <div class="flex flex-col sm:flex-row gap-4">
            <select bind:value={selectedRole} class="bg-gray-800/50 border border-gray-600/50 rounded-xl px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50">
              <option value="Admin">Admin View</option>
              <option value="Inspector">Inspector View</option>
              <option value="Viewer">Viewer View</option>
            </select>
            
            <button 
              on:click={generateReport}
              disabled={loading || !generator}
              class="bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 disabled:from-gray-600 disabled:to-gray-600 text-white px-8 py-3 rounded-xl font-bold transition-all duration-300 disabled:cursor-not-allowed"
            >
              {loading ? 'Generating...' : 'Generate AI Report'}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Data Overview -->
    <div class="mb-12">
      <h2 class="text-3xl font-bold text-white mb-8 text-center">System Overview</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
        <div class="bg-blue-800/30 rounded-xl p-6 border border-blue-500/40 text-center">
          <div class="text-3xl font-bold text-blue-300 mb-2">{summaryStats.totalVendors}</div>
          <div class="text-blue-100 text-sm">Total Vendors</div>
          <div class="text-blue-200 text-xs mt-1">Active suppliers</div>
        </div>
        
        <div class="bg-green-800/30 rounded-xl p-6 border border-green-500/40 text-center">
          <div class="text-3xl font-bold text-green-300 mb-2">{summaryStats.totalBatches}</div>
          <div class="text-green-100 text-sm">Total Batches</div>
          <div class="text-green-200 text-xs mt-1">Production units</div>
        </div>
        
        <div class="bg-yellow-800/30 rounded-xl p-6 border border-yellow-500/40 text-center">
          <div class="text-3xl font-bold text-yellow-300 mb-2">{summaryStats.pendingInspections}</div>
          <div class="text-yellow-100 text-sm">Pending Inspections</div>
          <div class="text-yellow-200 text-xs mt-1">Awaiting review</div>
        </div>
        
        <div class="bg-red-800/30 rounded-xl p-6 border border-red-500/40 text-center">
          <div class="text-3xl font-bold text-red-300 mb-2">{summaryStats.failedBatches}</div>
          <div class="text-red-100 text-sm">Failed Batches</div>
          <div class="text-red-200 text-xs mt-1">Quality failures</div>
        </div>
      </div>
    </div>

    <!-- AI Insights -->
    {#if aiInsights.summary}
      <div class="space-y-8">
        <!-- AI Summary -->
        <div class="bg-gradient-to-r from-blue-900/20 to-purple-900/20 backdrop-blur-2xl border border-blue-500/30 rounded-3xl p-8">
          <h4 class="text-2xl font-bold text-white mb-4 flex items-center">
            <svg class="w-6 h-6 mr-3 text-blue-400" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"/>
            </svg>
            AI Analysis Summary
          </h4>
          <p class="text-gray-300 text-lg leading-relaxed">{aiInsights.summary}</p>
        </div>

        <!-- Critical Alerts -->
        {#if aiInsights.criticalAlerts.length > 0}
          <div class="bg-red-900/20 backdrop-blur-2xl border border-red-500/30 rounded-3xl p-8">
            <h4 class="text-2xl font-bold text-red-300 mb-6 flex items-center">
              <svg class="w-6 h-6 mr-3" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
              </svg>
              Critical Alerts
            </h4>
            <div class="space-y-4">
              {#each aiInsights.criticalAlerts as alert}
                <div class="bg-red-800/30 rounded-xl p-4 border border-red-500/40">
                  <p class="text-red-100">{alert}</p>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Vendor Performance -->
        {#if aiInsights.vendorPerformance.length > 0}
          <div class="bg-gray-900/40 backdrop-blur-2xl border border-gray-700/40 rounded-3xl p-8">
            <div class="flex items-center justify-between mb-6">
              <h4 class="text-2xl font-bold text-white flex items-center">
                <svg class="w-6 h-6 mr-3 text-green-400" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zM8 7a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zM14 4a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z"/>
                </svg>
                Vendor Performance Analysis
              </h4>
              <div class="text-sm text-gray-400">
                Showing {aiInsights.vendorPerformance.length} of {summaryStats.totalVendors} vendors
              </div>
            </div>
            <div class="space-y-4">
              {#each aiInsights.vendorPerformance as vendor}
                <div class="bg-gray-800/30 rounded-xl p-4 border border-gray-600/40">
                  <div class="flex items-center justify-between mb-2">
                    <h5 class="text-lg font-semibold text-white">Vendor {vendor.vendor_id}</h5>
                    <span class="px-3 py-1 rounded-full text-sm font-medium {
                      vendor.failure_rate < 10 ? 'bg-green-900/30 text-green-300' :
                      vendor.failure_rate < 25 ? 'bg-yellow-900/30 text-yellow-300' :
                      'bg-red-900/30 text-red-300'
                    }">
                      {vendor.failure_rate}% failure rate
                    </span>
                  </div>
                  <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 text-sm">
                    <div>
                      <span class="text-gray-400">State:</span>
                      <span class="text-white ml-1">{vendor.state}</span>
                    </div>
                    <div>
                      <span class="text-gray-400">Total Batches:</span>
                      <span class="text-white ml-1">{vendor.total_batches}</span>
                    </div>
                    <div>
                      <span class="text-gray-400">Failed:</span>
                      <span class="text-white ml-1">{vendor.failed_batches}</span>
                    </div>
                    <div>
                      <span class="text-gray-400">Status:</span>
                      <span class="text-white ml-1">{vendor.status || 'Active'}</span>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        <!-- Recommendations -->
        {#if aiInsights.recommendations.length > 0}
          <div class="bg-gradient-to-r from-green-900/20 to-blue-900/20 backdrop-blur-2xl border border-green-500/30 rounded-3xl p-8">
            <h4 class="text-2xl font-bold text-green-300 mb-6 flex items-center">
              <svg class="w-6 h-6 mr-3" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd"/>
              </svg>
              AI Recommendations
            </h4>
            <div class="space-y-3">
              {#each aiInsights.recommendations as recommendation}
                <div class="flex items-start space-x-3">
                  <div class="w-2 h-2 bg-green-400 rounded-full mt-2 flex-shrink-0"></div>
                  <p class="text-gray-300">{recommendation}</p>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>