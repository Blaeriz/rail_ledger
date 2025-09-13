<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let userRole = '';
  let username = '';
  let activePage = '/admin/dashboard';

  // Sample data
  let summaryData = {
    totalVendors: 25,
    activeBatches: 150,
    pendingInspections: 12,
    failuresThisMonth: 3
  };

  let alerts = [
    { id: 1, type: 'warning', message: 'Batch #B045 nearing expiry in 7 days', priority: 'high' },
    { id: 2, type: 'info', message: 'New vendor V026 requires audit', priority: 'medium' },
    { id: 3, type: 'error', message: 'Batch #B023 failed QC inspection', priority: 'high' },
    { id: 4, type: 'success', message: 'All fitments completed for Zone A', priority: 'low' }
  ];

  // Quick action handlers
  function handleAddVendor() {
    goto('/admin/vendors?action=add');
  }

  function handleCreateBatch() {
    goto('/admin/batches?action=add');
  }

  function handleAddUser() {
    goto('/admin/users?action=add');
  }

  function handleGenerateReport() {
    goto('/reports');
  }

  onMount(() => {
    const storedRole = localStorage.getItem('role');
    const storedUsername = localStorage.getItem('username');
    
    if (storedRole && storedUsername) {
      userRole = storedRole;
      username = storedUsername;
    }
  });
</script>

<Layout {userRole} {username} {activePage}>
  <div class="admin-dashboard">
    <h1>Admin Dashboard</h1>
    
    <!-- Summary Cards -->
    <div class="summary-cards">
      <div class="summary-card vendors">
        <div class="card-content">
          <h3>Total Vendors</h3>
          <p class="card-number">{summaryData.totalVendors}</p>
        </div>
      </div>
      
      <div class="summary-card batches">
        <div class="card-content">
          <h3>Active Batches</h3>
          <p class="card-number">{summaryData.activeBatches}</p>
        </div>
      </div>
      
      <div class="summary-card inspections">
        <div class="card-content">
          <h3>Pending Inspections</h3>
          <p class="card-number">{summaryData.pendingInspections}</p>
        </div>
      </div>
      
      <div class="summary-card failures">
        <div class="card-content">
          <h3>Failures This Month</h3>
          <p class="card-number">{summaryData.failuresThisMonth}</p>
        </div>
      </div>
    </div>

    <!-- Alerts Panel -->
    <div class="alerts-section">
      <h2>AI Alerts & Notifications</h2>
      <div class="alerts-grid">
        {#each alerts as alert}
          <div class="alert-card alert-{alert.type}">
            <div class="alert-priority priority-{alert.priority}">
              {alert.priority.toUpperCase()}
            </div>
            <p class="alert-message">{alert.message}</p>
          </div>
        {/each}
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="quick-actions">
      <h2>Quick Actions</h2>
      <div class="actions-grid">
        <button class="action-btn" on:click={handleAddVendor}>
          <span class="action-icon add-vendor">
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M3 21H21M5 21V7L12 3L19 7V21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </span>
          Add New Vendor
        </button>
        <button class="action-btn" on:click={handleCreateBatch}>
          <span class="action-icon create-batch">
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M20 7L12 3L4 7M20 7V17L12 21M20 7L12 11M4 7V17L12 21M4 7L12 11" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </span>
          Create Batch
        </button>
        <button class="action-btn" on:click={handleAddUser}>
          <span class="action-icon add-user">
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M20 21V19C20 17.9391 19.5786 16.9217 18.8284 16.1716C18.0783 15.4214 17.0609 15 16 15H8C6.93913 15 5.92172 15.4214 5.17157 16.1716C4.42143 16.9217 4 17.9391 4 19V21M16 7C16 9.20914 14.2091 11 12 11C9.79086 11 8 9.20914 8 7C8 4.79086 9.79086 3 12 3C14.2091 3 16 4.79086 16 7Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </span>
          Add User
        </button>
        <button class="action-btn" on:click={handleGenerateReport}>
          <span class="action-icon generate-report">
            <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M9 17H15M9 13H15M9 9H12M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </span>
          Generate Report
        </button>
      </div>
    </div>
  </div>
</Layout>

<style>
  .admin-dashboard {
    padding: 0;
  }

  .admin-dashboard h1 {
    color: #000000;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
  }

  .summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
    margin-bottom: 3rem;
  }

  @media (max-width: 768px) {
    .admin-dashboard {
      padding: 1rem;
    }
    
    .summary-cards {
      grid-template-columns: 1fr;
      gap: 1rem;
      margin-bottom: 2rem;
    }
    
    .summary-card {
      padding: 1rem;
    }
    
    .card-icon {
      font-size: 2rem;
    }
  }

  @media (max-width: 480px) {
    .admin-dashboard {
      padding: 0.5rem;
    }
    
    .summary-card {
      flex-direction: column;
      text-align: center;
      gap: 0.5rem;
    }
  }

  .summary-card {
    background: white;
    padding: 0.75rem 1rem;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    transition: all 0.3s ease;
    border: 1px solid #e2e8f0;
    min-height: 60px;
    border-left: 4px solid #e5e7eb;
  }

  .summary-card.vendors {
    border-left-color: #8b5cf6;
  }

  .summary-card.batches {
    border-left-color: #06b6d4;
  }

  .summary-card.inspections {
    border-left-color: #3b82f6;
  }

  .summary-card.failures {
    border-left-color: #ef4444;
  }

  .summary-card:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .card-content h3 {
    margin: 0 0 0.125rem 0;
    color: #64748b;
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .card-number {
    margin: 0;
    font-size: 1.25rem;
    font-weight: bold;
    color: #000000;
  }

  .alerts-section {
    margin-bottom: 3rem;
  }

  .alerts-section h2 {
    color: #000000;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .alerts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1rem;
  }

  .alert-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #e5e7eb;
  }

  .alert-warning {
    border-left-color: #f59e0b;
  }

  .alert-info {
    border-left-color: #3b82f6;
  }

  .alert-error {
    border-left-color: #ef4444;
  }

  .alert-success {
    border-left-color: #10b981;
  }

  .alert-priority {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
  }

  .priority-high {
    background: #fef2f2;
    color: #dc2626;
  }

  .priority-medium {
    background: #fef3c7;
    color: #d97706;
  }

  .priority-low {
    background: #f0fdf4;
    color: #059669;
  }

  .alert-message {
    margin: 0;
    color: #374151;
    font-size: 0.9rem;
  }

  .quick-actions h2 {
    color: #000000;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .action-btn {
    background: white;
    border: 2px solid #e5e7eb;
    padding: 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.3s;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    color: #374151;
    font-weight: 500;
  }

  .action-btn:hover {
    border-color: #3b82f6;
    background: #f8fafc;
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
  }

  .action-btn:active {
    transform: translateY(0);
    box-shadow: 0 2px 4px rgba(59, 130, 246, 0.1);
  }

  .action-icon {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 6px;
    transition: all 0.3s ease;
  }

  .action-icon svg {
    width: 16px;
    height: 16px;
  }

  .action-icon.add-vendor {
    background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
    color: white;
  }

  .action-icon.create-batch {
    background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);
    color: white;
  }

  .action-icon.add-user {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
  }

  .action-icon.generate-report {
    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
    color: white;
  }

  .action-btn:hover .action-icon {
    transform: scale(1.1);
  }
</style>
