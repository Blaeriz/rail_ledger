<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/reports';

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
  <div class="reports-page">
    <h1>AI Reports</h1>
    
    <div class="reports-grid">
      <div class="report-card">
        <h3>Vendor Risk Levels</h3>
        <div class="report-content">
          <p>High Risk: 2 vendors</p>
          <p>Medium Risk: 3 vendors</p>
          <p>Low Risk: 5 vendors</p>
        </div>
      </div>
      
      <div class="report-card">
        <h3>Failure Hotspots</h3>
        <div class="report-content">
          <p>Zone A: 3 failures</p>
          <p>Zone B: 1 failure</p>
          <p>Zone C: 0 failures</p>
        </div>
      </div>
      
      <div class="report-card">
        <h3>Warranty Replacements Due</h3>
        <div class="report-content">
          <p>Next 30 days: 2 batches</p>
          <p>Next 60 days: 5 batches</p>
          <p>Next 90 days: 8 batches</p>
        </div>
      </div>
    </div>
  </div>
</Layout>

<style>
  .reports-page {
    padding: 2rem;
  }

  .reports-page h1 {
    color: #000000;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .reports-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
  }

  .report-card {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .report-card h3 {
    color: #000000;
    margin: 0 0 1rem 0;
    font-size: 1.25rem;
  }

  .report-content p {
    margin: 0.5rem 0;
    color: #64748b;
  }
</style>
