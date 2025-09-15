<script>
  import Layout from '$lib/components/Layout.svelte';
  import DataTable from '$lib/components/DataTable.svelte';
  import AnalyticsCard from '$lib/components/AnalyticsCard.svelte';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  // Removed problematic data store import

  let userRole = '';
  let username = '';
  let activePage = '/admin/dashboard';
  // Simple data without data store
  let vendors = [];
  let batches = [];
  let users = [];
  let isLoading = true;

  // Quick action handlers
  function handleAddVendor() { goto('/admin/vendors?action=add'); }
  function handleCreateBatch() { goto('/admin/batches?action=add'); }
  function handleAddUser() { goto('/admin/users?action=add'); }
  function handleGenerateReport() { goto('/reports'); }

  // Computed values
  $: summaryData = {
    totalVendors: vendors.length,
    activeBatches: batches.filter(b => b.qc_status === 'pending').length,
    pendingInspections: batches.filter(b => b.qc_status === 'pending').length,
    failuresThisMonth: batches.filter(b => b.qc_status === 'failed').length
  };

  $: recentBatches = batches.slice(0, 5).map(batch => ({
    ...batch,
    vendor_name: vendors.find(v => v.vendor_id === batch.vendor_id)?.city || 'Unknown',
    date_of_production: batch.date_of_production
  }));

  $: vendorStats = vendors.map(vendor => {
    const vendorBatches = batches.filter(b => b.vendor_id === vendor.vendor_id);
    const completedBatches = vendorBatches.filter(b => b.qc_status === 'completed');
    const successRate = vendorBatches.length > 0 ? Math.round((completedBatches.length / vendorBatches.length) * 100) : 0;
    
    return {
      vendor_id: vendor.vendor_id,
      city: vendor.city || 'Unknown',
      total_batches: vendorBatches.length,
      success_rate: successRate,
      email: vendor.email || 'N/A'
    };
  });

  $: alerts = batches.filter(b => b.qc_status === 'pending' && b.expiry_date && new Date(b.expiry_date) < new Date()).slice(0, 4).map(batch => ({
    id: `overdue-${batch.batch_id}`,
    type: 'warning',
    message: `Batch #${batch.batch_id} is overdue`,
    priority: 'high'
  }));

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
  <div class="admin-dashboard">
    <h1>Admin Dashboard</h1>
    
    {#if isLoading}
      <div class="loading">Loading...</div>
    {:else}
      <!-- Summary Cards -->
      <div class="summary-grid">
        <AnalyticsCard title="Total Vendors" value={summaryData.totalVendors} color="#8b5cf6" />
        <AnalyticsCard title="Active Batches" value={summaryData.activeBatches} color="#06b6d4" />
        <AnalyticsCard title="Pending Inspections" value={summaryData.pendingInspections} color="#3b82f6" />
        <AnalyticsCard title="Failures This Month" value={summaryData.failuresThisMonth} color="#ef4444" />
      </div>

      <!-- Quick Actions -->
      <div class="actions">
        <button on:click={handleAddVendor}>Add Vendor</button>
        <button on:click={handleCreateBatch}>Create Batch</button>
        <button on:click={handleAddUser}>Add User</button>
        <button on:click={handleGenerateReport}>Generate Report</button>
      </div>

      <!-- Data Tables -->
      <DataTable 
        title="Recent Batches" 
        data={recentBatches} 
        columns={[
          { key: 'batch_id', label: 'Batch ID' },
          { key: 'vendor_name', label: 'Vendor' },
          { key: 'date_of_production', label: 'Production Date', type: 'date' },
          { key: 'qc_status', label: 'Status', type: 'status' },
          { key: 'expiry_date', label: 'Expiry Date', type: 'date' }
        ]}
        searchable={true}
      />

      <DataTable 
        title="Vendor Performance" 
        data={vendorStats} 
        columns={[
          { key: 'vendor_id', label: 'Vendor ID' },
          { key: 'city', label: 'Location' },
          { key: 'total_batches', label: 'Total Batches' },
          { key: 'success_rate', label: 'Success Rate', type: 'progress' },
          { key: 'email', label: 'Contact' }
        ]}
        searchable={true}
      />

      <!-- Alerts -->
      {#if alerts.length > 0}
        <div class="alerts">
          <h3>Alerts</h3>
          {#each alerts as alert}
            <div class="alert {alert.type}">{alert.message}</div>
          {/each}
        </div>
      {/if}
    {/if}
  </div>
</Layout>

<style>
  .admin-dashboard {
    padding: 2rem;
  }

  .admin-dashboard h1 {
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
    margin-bottom: 2rem;
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

  .alerts {
    margin-top: 2rem;
  }

  .alerts h3 {
    color: #000000;
    margin-bottom: 1rem;
  }

  .alert {
    padding: 1rem;
    border-radius: 6px;
    margin-bottom: 0.5rem;
    border-left: 4px solid #f59e0b;
  }

  .alert.warning {
    background: #fef3c7;
    color: #92400e;
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
    .admin-dashboard {
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
