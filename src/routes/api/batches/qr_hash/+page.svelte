<script lang="ts">
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
  
    type Batch = {
      batch_id: string;
      vendor_id: string;
      batch_size: number;
      date_of_production: string;
      qc_status: string;
      expiry_date: string;
      last_inspection_date?: string;
      fitment_date?: string;
      fitment_location?: string;
      qr_hash?: string;
    };
  
    let qrHashInput = '';
    let searchResult: Batch | null = null;
    let searchError = '';
    let isLoading = false;
    let urlParams: URLSearchParams;
  
    onMount(() => {
      // Get QR hash from URL parameters if present
      urlParams = new URLSearchParams($page.url.search);
      const qrHash = urlParams.get('qr_hash');
      if (qrHash) {
        qrHashInput = qrHash;
        fetchBatchByQRHash();
      }
    });
  
    // Fetch batch by QR hash
    async function fetchBatchByQRHash() {
      if (!qrHashInput.trim()) {
        searchError = 'Please enter a QR hash';
        return;
      }
      
      isLoading = true;
      searchError = '';
      
      try {
        const res = await fetch(`/api/batches/qr_hash?qr_hash=${encodeURIComponent(qrHashInput.trim())}`);
        const data = await res.json();
        
        if (res.ok) {
          searchResult = data;
        } else {
          searchError = data.error || 'Batch not found';
          searchResult = null;
        }
      } catch (err) {
        console.error('Failed to fetch batch by QR hash:', err);
        searchError = 'Failed to fetch batch data';
        searchResult = null;
      } finally {
        isLoading = false;
      }
    }

    // Clear search results
    function clearSearch() {
      qrHashInput = '';
      searchResult = null;
      searchError = '';
    }

    // Format date for display
    function formatDate(dateString: string | null | undefined): string {
      if (!dateString) return 'N/A';
      return new Date(dateString).toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      });
    }

    // Get status color
    function getStatusColor(status: string): string {
      switch (status.toLowerCase()) {
        case 'passed':
        case 'approved':
          return 'text-green-400 bg-green-500/20';
        case 'pending inspection':
        case 'pending':
          return 'text-yellow-400 bg-yellow-500/20';
        case 'failed':
        case 'rejected':
          return 'text-red-400 bg-red-500/20';
        default:
          return 'text-gray-400 bg-gray-500/20';
      }
    }
  </script>
  
<svelte:head>
  <title>Batch QR Hash Lookup | Rail Ledger</title>
</svelte:head>

<div class="min-h-screen bg-black text-white p-4">
  <div class="container mx-auto py-8">
    <h1 class="text-4xl font-bold mb-8 text-center">Batch QR Hash Lookup</h1>
    
    <!-- Search Section -->
    <div class="mb-8 rounded-lg border border-gray-800 bg-gray-900/50 p-6">
      <h2 class="text-2xl font-semibold mb-4 text-white">Search Batch by QR Hash</h2>
      <div class="flex gap-4 mb-4">
        <input 
          placeholder="Enter QR Hash (e.g., 0001cvc)" 
          bind:value={qrHashInput}
          class="flex-1 rounded-lg border border-gray-600 bg-gray-800 px-4 py-3 text-white placeholder-gray-400 focus:border-purple-500 focus:outline-none"
          on:keydown={(e) => e.key === 'Enter' && fetchBatchByQRHash()}
        />
        <button 
          on:click={fetchBatchByQRHash} 
          disabled={isLoading}
          class="rounded-lg bg-purple-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed"
        >
          {isLoading ? 'Searching...' : 'Search'}
        </button>
        <button 
          on:click={clearSearch}
          class="rounded-lg bg-gray-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-gray-700"
        >
          Clear
        </button>
      </div>
      
      {#if searchError}
        <div class="rounded-lg border border-red-500/30 bg-red-500/10 px-4 py-3 text-red-400">
          {searchError}
        </div>
      {/if}
    </div>

    <!-- Results Section -->
    {#if searchResult}
      <div class="rounded-lg border border-gray-800 bg-gray-900/50 p-6">
        <h2 class="text-2xl font-semibold mb-6 text-white">Batch Information</h2>
        
        <!-- Main Info Cards -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
          <!-- Batch ID Card -->
          <div class="rounded-lg border border-gray-700 bg-gray-800/50 p-4">
            <h3 class="text-sm font-medium text-gray-400 mb-2">Batch ID</h3>
            <p class="text-xl font-bold text-white">{searchResult.batch_id}</p>
          </div>
          
          <!-- Vendor ID Card -->
          <div class="rounded-lg border border-gray-700 bg-gray-800/50 p-4">
            <h3 class="text-sm font-medium text-gray-400 mb-2">Vendor ID</h3>
            <p class="text-xl font-bold text-white">{searchResult.vendor_id}</p>
          </div>
          
          <!-- Batch Size Card -->
          <div class="rounded-lg border border-gray-700 bg-gray-800/50 p-4">
            <h3 class="text-sm font-medium text-gray-400 mb-2">Batch Size</h3>
            <p class="text-xl font-bold text-white">{searchResult.batch_size.toLocaleString()}</p>
          </div>
          
          <!-- QR Hash Card -->
          <div class="rounded-lg border border-gray-700 bg-gray-800/50 p-4">
            <h3 class="text-sm font-medium text-gray-400 mb-2">QR Hash</h3>
            <p class="text-xl font-bold text-purple-400 font-mono">{searchResult.qr_hash}</p>
          </div>
          
          <!-- QC Status Card -->
          <div class="rounded-lg border border-gray-700 bg-gray-800/50 p-4">
            <h3 class="text-sm font-medium text-gray-400 mb-2">QC Status</h3>
            <span class="inline-flex items-center rounded-full px-3 py-1 text-sm font-medium {getStatusColor(searchResult.qc_status)}">
              {searchResult.qc_status}
            </span>
          </div>
          
          <!-- Fitment Location Card -->
          <div class="rounded-lg border border-gray-700 bg-gray-800/50 p-4">
            <h3 class="text-sm font-medium text-gray-400 mb-2">Fitment Location</h3>
            <p class="text-xl font-bold text-white">{searchResult.fitment_location || 'Not Assigned'}</p>
          </div>
        </div>

        <!-- Detailed Information Table -->
        <div class="overflow-x-auto">
          <table class="w-full border-collapse">
            <thead>
              <tr class="border-b border-gray-700">
                <th class="px-4 py-3 text-left text-sm font-semibold text-gray-300">Field</th>
                <th class="px-4 py-3 text-left text-sm font-semibold text-gray-300">Value</th>
              </tr>
            </thead>
            <tbody>
              <tr class="border-b border-gray-800">
                <td class="px-4 py-3 text-sm font-medium text-gray-400">Production Date</td>
                <td class="px-4 py-3 text-sm text-white">{formatDate(searchResult.date_of_production)}</td>
              </tr>
              <tr class="border-b border-gray-800">
                <td class="px-4 py-3 text-sm font-medium text-gray-400">Expiry Date</td>
                <td class="px-4 py-3 text-sm text-white">{formatDate(searchResult.expiry_date)}</td>
              </tr>
              <tr class="border-b border-gray-800">
                <td class="px-4 py-3 text-sm font-medium text-gray-400">Last Inspection Date</td>
                <td class="px-4 py-3 text-sm text-white">{formatDate(searchResult.last_inspection_date)}</td>
              </tr>
              <tr class="border-b border-gray-800">
                <td class="px-4 py-3 text-sm font-medium text-gray-400">Fitment Date</td>
                <td class="px-4 py-3 text-sm text-white">{formatDate(searchResult.fitment_date)}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    {:else if !isLoading && !searchError}
      <div class="text-center py-12">
        <div class="text-gray-400 text-lg">
          Enter a QR hash above to search for batch information
        </div>
      </div>
    {/if}
  </div>
</div>