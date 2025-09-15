<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/viewer/analytics';

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
  <div class="analytics-page">
    <h1>Analytics Dashboard</h1>
    
    <div class="charts-grid">
      <div class="chart-card">
        <h3>Vendor Performance</h3>
        <div class="chart-placeholder">
          <p>Pass: 85% | Fail: 15%</p>
        </div>
      </div>
      
      <div class="chart-card">
        <h3>Failures Over Time</h3>
        <div class="chart-placeholder">
          <p>Jan: 2 | Feb: 1 | Mar: 3</p>
        </div>
      </div>
      
      <div class="chart-card">
        <h3>Expiry Timeline</h3>
        <div class="chart-placeholder">
          <p>Next 6 months: 5 batches</p>
        </div>
      </div>
      
      <div class="chart-card">
        <h3>Inspection Compliance</h3>
        <div class="chart-placeholder">
          <p>Zone A: 95% | Zone B: 88%</p>
        </div>
      </div>
    </div>
  </div>
</Layout>

<style>
  .analytics-page {
    padding: 2rem;
  }

  .analytics-page h1 {
    color: #1e293b;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .charts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .chart-card {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .chart-card h3 {
    color: #1e293b;
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
  }

  .chart-placeholder {
    height: 200px;
    background: #f8fafc;
    border: 2px dashed #cbd5e1;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #64748b;
  }
</style>
