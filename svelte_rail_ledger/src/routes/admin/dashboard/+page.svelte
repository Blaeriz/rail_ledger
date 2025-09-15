<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

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
      <div class="summary-card">
        <div class="card-icon">🏢</div>
        <div class="card-content">
          <h3>Total Vendors</h3>
          <p class="card-number">{summaryData.totalVendors}</p>
        </div>
      </div>
      
      <div class="summary-card">
        <div class="card-icon">📦</div>
        <div class="card-content">
          <h3>Active Batches</h3>
          <p class="card-number">{summaryData.activeBatches}</p>
        </div>
      </div>
      
      <div class="summary-card">
        <div class="card-icon">🔍</div>
        <div class="card-content">
          <h3>Pending Inspections</h3>
          <p class="card-number">{summaryData.pendingInspections}</p>
        </div>
      </div>
      
      <div class="summary-card">
        <div class="card-icon">❌</div>
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
        <button class="action-btn">
          <span class="action-icon">➕</span>
          Add New Vendor
        </button>
        <button class="action-btn">
          <span class="action-icon">📦</span>
          Create Batch
        </button>
        <button class="action-btn">
          <span class="action-icon">👥</span>
          Add User
        </button>
        <button class="action-btn">
          <span class="action-icon">📊</span>
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
    color: #1e293b;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
    margin-bottom: 3rem;
  }

  @media (max-width: 1024px) {
    .alerts-grid {
      grid-template-columns: repeat(2, 1fr);
      gap: 1.25rem;
    }
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
    
    .alerts-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
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
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    display: flex;
    align-items: center;
    gap: 1rem;
    transition: all 0.3s ease;
    border: 1px solid #e2e8f0;
  }

  .summary-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }

  .card-icon {
    font-size: 2.5rem;
    opacity: 0.8;
  }

  .card-content h3 {
    margin: 0 0 0.5rem 0;
    color: #64748b;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .card-number {
    margin: 0;
    font-size: 2rem;
    font-weight: bold;
    color: #1e293b;
  }

  .alerts-section {
    margin-bottom: 3rem;
  }

  .alerts-section h2 {
    color: #1e293b;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .alerts-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
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
    color: #1e293b;
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
  }

  .action-btn:hover {
    border-color: #3b82f6;
    background: #f8fafc;
  }

  .action-icon {
    font-size: 1.5rem;
  }
</style>
