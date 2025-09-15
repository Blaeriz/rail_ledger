<script>
  import Layout from '$lib/components/Layout.svelte';
  import DataTable from '$lib/components/DataTable.svelte';
  import AnalyticsCard from '$lib/components/AnalyticsCard.svelte';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  // Removed problematic data store import

  let userRole = '';
  let username = '';
  let activePage = '/inspector/dashboard';
  // Simple data without data store
  let vendors = [];
  let batches = [];
  let users = [];
  let isLoading = true;

  function startInspection(batchId) {
    localStorage.setItem('currentInspectionBatch', batchId);
    goto('/inspector/scan');
  }

  // Computed values
  $: summaryData = {
    totalInspections: batches.length,
    pendingInspections: batches.filter(b => b.qc_status === 'pending').length,
    completedInspections: batches.filter(b => b.qc_status === 'completed').length,
    failedInspections: batches.filter(b => b.qc_status === 'failed').length
  };

  $: assignedInspections = batches.filter(b => b.qc_status === 'pending').slice(0, 10).map(batch => ({
    batch_id: batch.batch_id,
    vendor: vendors.find(v => v.vendor_id === batch.vendor_id)?.city || 'Unknown',
    location: batch.fitment_location || 'TBD',
    due_date: batch.expiry_date || 'N/A',
    status: batch.qc_status,
    priority: 'High'
  }));

  $: inspectionHistory = batches
    .filter(b => b.last_inspection_date)
    .sort((a, b) => new Date(b.last_inspection_date) - new Date(a.last_inspection_date))
    .slice(0, 10)
    .map(batch => ({
      ...batch,
      vendor_name: vendors.find(v => v.vendor_id === batch.vendor_id)?.city || 'Unknown'
    }));

  $: vendorPerformance = vendors.map(vendor => {
    const vendorBatches = batches.filter(b => b.vendor_id === vendor.vendor_id);
    const completedBatches = vendorBatches.filter(b => b.qc_status === 'completed');
    const successRate = vendorBatches.length > 0 ? Math.round((completedBatches.length / vendorBatches.length) * 100) : 0;
    
    return {
      vendor_id: vendor.vendor_id,
      city: vendor.city || 'Unknown',
      total_batches: vendorBatches.length,
      success_rate: successRate,
      contact: vendor.email || vendor.phone_number || 'N/A'
    };
  });

  onMount(async () => {
    const storedRole = localStorage.getItem('role');
    const storedUsername = localStorage.getItem('username');
    
    if (storedRole && storedUsername) {
      userRole = storedRole;
      username = storedUsername;
    }
    
    try {
      // Fetch real data from APIs
      const [vendorsResponse, batchesResponse, usersResponse] = await Promise.all([
        fetch('/api/vendors'),
        fetch('/api/batches'),
        fetch('/api/users')
      ]);

      if (vendorsResponse.ok) {
        vendors = await vendorsResponse.json();
      }
      
      if (batchesResponse.ok) {
        batches = await batchesResponse.json();
      }
      
      if (usersResponse.ok) {
        users = await usersResponse.json();
      }
    } catch (error) {
      console.error('Error fetching data:', error);
    }
    
    isLoading = false;
  });
</script>

<Layout {userRole} {username} {activePage}>
  <div class="inspector-dashboard">
    <h1>Inspector Dashboard</h1>
    
    {#if isLoading}
      <div class="loading">Loading...</div>
    {:else}
      <!-- Summary Cards -->
      <div class="summary-grid">
        <AnalyticsCard title="Total Inspections" value={summaryData.totalInspections} color="#3b82f6" />
        <AnalyticsCard title="Pending Inspections" value={summaryData.pendingInspections} color="#f59e0b" />
        <AnalyticsCard title="Completed Today" value={summaryData.completedInspections} color="#10b981" />
        <AnalyticsCard title="Failed Inspections" value={summaryData.failedInspections} color="#ef4444" />
      </div>

      <!-- Assigned Inspections -->
      <DataTable 
        title="Assigned Inspections" 
        data={assignedInspections} 
        columns={[
          { key: 'batch_id', label: 'Batch ID' },
          { key: 'vendor', label: 'Vendor' },
          { key: 'location', label: 'Location' },
          { key: 'due_date', label: 'Due Date', type: 'date' },
          { key: 'status', label: 'Status', type: 'status' },
          { key: 'priority', label: 'Priority' }
        ]}
        searchable={true}
      />

      <!-- Inspection History -->
      <DataTable 
        title="Recent Inspection History" 
        data={inspectionHistory} 
        columns={[
          { key: 'batch_id', label: 'Batch ID' },
          { key: 'vendor_name', label: 'Vendor' },
          { key: 'last_inspection_date', label: 'Inspection Date', type: 'date' },
          { key: 'qc_status', label: 'Status', type: 'status' },
          { key: 'fitment_location', label: 'Location' }
        ]}
        searchable={true}
      />

      <!-- Vendor Performance -->
      <DataTable 
        title="Vendor Performance" 
        data={vendorPerformance} 
        columns={[
          { key: 'vendor_id', label: 'Vendor ID' },
          { key: 'city', label: 'Location' },
          { key: 'total_batches', label: 'Total Batches' },
          { key: 'success_rate', label: 'Success Rate', type: 'progress' },
          { key: 'contact', label: 'Contact' }
        ]}
        searchable={true}
      />

      <!-- Quick Actions -->
      <div class="actions">
        <button on:click={() => goto('/inspector/scan')}>Start New Inspection</button>
        <button on:click={() => goto('/inspector/inspections')}>View All Inspections</button>
        <button on:click={() => goto('/inspector/fitments')}>Manage Fitments</button>
      </div>
    {/if}
  </div>
</Layout>

<style>
  .inspector-dashboard {
    padding: 2rem;
  }

  .inspector-dashboard h1 {
    color: #000000;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
  }

  .summary-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-top: 2rem;
    flex-wrap: wrap;
  }

  .actions button {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.3s ease;
  }

  .actions button:hover {
    background: #2563eb;
    transform: translateY(-1px);
  }

  .loading, .error {
    text-align: center;
    padding: 2rem;
    font-size: 1.1rem;
  }

  .error {
    color: #ef4444;
  }

  @media (max-width: 768px) {
    .inspector-dashboard {
      padding: 1rem;
    }

    .summary-grid {
      grid-template-columns: 1fr;
    }

    .actions {
      flex-direction: column;
    }
  }
</style>
