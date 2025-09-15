<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/viewer/dashboard';

  let summaryData = {
    totalVendors: 5,
    activeBatches: 12,
    pendingInspections: 3,
    failuresThisMonth: 1
  };

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
  <div class="viewer-dashboard">
    <h1>Viewer Dashboard</h1>
    
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

    <div class="alerts-section">
      <h2>System Alerts</h2>
      <div class="alert-card">
        <p>Batch #B003 warranty expires in 15 days</p>
      </div>
      <div class="alert-card">
        <p>Zone A inspection compliance: 95%</p>
      </div>
    </div>
  </div>
</Layout>

<style>
  .viewer-dashboard {
    padding: 2rem;
  }

  .viewer-dashboard h1 {
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

  @media (max-width: 768px) {
    .viewer-dashboard {
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
    
    .alerts-grid {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 480px) {
    .viewer-dashboard {
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
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    gap: 1rem;
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

  .alerts-section h2 {
    color: #1e293b;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .alert-card {
    background: white;
    padding: 1rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    margin-bottom: 1rem;
  }

  .alert-card p {
    margin: 0;
    color: #374151;
  }
</style>
