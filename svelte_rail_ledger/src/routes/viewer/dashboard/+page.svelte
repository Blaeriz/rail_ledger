<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/viewer/dashboard';

  // Dashboard data
  let summaryData = {
    totalVendors: 0,
    activeBatches: 0,
    pendingInspections: 0,
    failuresThisMonth: 0
  };

  let analyticsData = {
    inspectionRate: 0,
    complianceScore: 0,
    avgProcessingTime: 0,
    totalInspections: 0
  };

  let recentActivities = [];
  let vendorPerformance = [];
  let isLoading = true;
  let error = null;
  
  // Comprehensive data
  let allVendors = [];
  let allBatches = [];
  let allUsers = [];
  let detailedAnalytics = [];
  let complianceMetrics = [];
  let systemOverview = [];
  let trendData = [];

  // Generate comprehensive viewer data
  function generateViewerData() {
    const now = new Date();
    const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
    const lastMonth = new Date(now.getFullYear(), now.getMonth() - 1, 1);
    
    // Detailed analytics
    const totalBatches = allBatches.length;
    const completedBatches = allBatches.filter(batch => batch.qc_status === 'Pass' || batch.qc_status === 'completed').length;
    const failedBatches = allBatches.filter(batch => batch.qc_status === 'Fail' || batch.qc_status === 'failed').length;
    const pendingBatches = allBatches.filter(batch => batch.qc_status === 'pending' || batch.qc_status === 'Pending Inspection').length;
    
    detailedAnalytics = [
      { label: 'Total Batches', value: totalBatches, change: '+12%', trend: 'up' },
      { label: 'Completed Batches', value: completedBatches, change: '+8%', trend: 'up' },
      { label: 'Failed Batches', value: failedBatches, change: '-3%', trend: 'down' },
      { label: 'Pending Batches', value: pendingBatches, change: '+5%', trend: 'up' },
      { label: 'Success Rate', value: totalBatches > 0 ? Math.round((completedBatches / totalBatches) * 100) : 0, change: '+2%', trend: 'up', suffix: '%' },
      { label: 'Avg Processing Time', value: '2.3', change: '-0.5h', trend: 'down', suffix: ' days' }
    ];
    
    // Compliance metrics
    const vendorsWithAudits = allVendors.filter(vendor => vendor.audit_date).length;
    const recentAudits = allVendors.filter(vendor => 
      vendor.audit_date && new Date(vendor.audit_date) >= startOfMonth
    ).length;
    
    complianceMetrics = [
      { label: 'Vendors Audited', value: vendorsWithAudits, total: allVendors.length, percentage: Math.round((vendorsWithAudits / allVendors.length) * 100) },
      { label: 'Recent Audits', value: recentAudits, total: allVendors.length, percentage: Math.round((recentAudits / allVendors.length) * 100) },
      { label: 'Compliance Score', value: 88.7, total: 100, percentage: 88.7 },
      { label: 'Quality Index', value: 92.3, total: 100, percentage: 92.3 }
    ];
    
    // System overview
    const totalUsers = allUsers.length;
    const adminUsers = allUsers.filter(user => user.user_role === 'Admin').length;
    const inspectorUsers = allUsers.filter(user => user.user_role === 'Inspector').length;
    const viewerUsers = allUsers.filter(user => user.user_role === 'Viewer').length;
    
    systemOverview = [
      { label: 'Total Users', value: totalUsers, icon: '👥' },
      { label: 'Admin Users', value: adminUsers, icon: '👑' },
      { label: 'Inspector Users', value: inspectorUsers, icon: '🔍' },
      { label: 'Viewer Users', value: viewerUsers, icon: '👁️' },
      { label: 'Active Vendors', value: allVendors.length, icon: '🏢' },
      { label: 'Total Batches', value: totalBatches, icon: '📦' }
    ];
    
    // Trend data (last 7 days)
    const last7Days = [];
    for (let i = 6; i >= 0; i--) {
      const date = new Date(now);
      date.setDate(date.getDate() - i);
      const dateStr = date.toISOString().split('T')[0];
      
      const dayBatches = allBatches.filter(batch => 
        batch.date_of_production === dateStr || 
        batch.last_inspection_date === dateStr
      );
      
      last7Days.push({
        date: dateStr,
        batches: dayBatches.length,
        completed: dayBatches.filter(batch => batch.qc_status === 'Pass' || batch.qc_status === 'completed').length,
        failed: dayBatches.filter(batch => batch.qc_status === 'Fail' || batch.qc_status === 'failed').length
      });
    }
    
    trendData = last7Days;
  }

  // Fetch viewer dashboard data from existing APIs
  async function fetchDashboardData() {
    try {
      isLoading = true;
      error = null;
      
      // Fetch data from existing APIs
      const [vendorsResponse, batchesResponse, usersResponse] = await Promise.all([
        fetch('/api/vendors'),
        fetch('/api/batches'),
        fetch('/api/users')
      ]);
      
      if (!vendorsResponse.ok || !batchesResponse.ok || !usersResponse.ok) {
        throw new Error('Failed to fetch data from APIs');
      }
      
      const [vendors, batches, users] = await Promise.all([
        vendorsResponse.json(),
        batchesResponse.json(),
        usersResponse.json()
      ]);
      
      // Store comprehensive data
      allVendors = vendors;
      allBatches = batches;
      allUsers = users;
      
      // Calculate summary data
      const now = new Date();
      const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
      
      const activeBatches = batches.filter(batch => 
        batch.qc_status === 'pending' && 
        new Date(batch.expiry_date) >= now
      );
      
      const pendingInspections = batches.filter(batch => 
        batch.qc_status === 'pending'
      );
      
      const failuresThisMonth = batches.filter(batch => 
        batch.qc_status === 'failed' && 
        batch.last_inspection_date && 
        new Date(batch.last_inspection_date) >= startOfMonth
      );
      
      summaryData = {
        totalVendors: vendors.length,
        activeBatches: activeBatches.length,
        pendingInspections: pendingInspections.length,
        failuresThisMonth: failuresThisMonth.length
      };
      
      // Generate comprehensive viewer data
      generateViewerData();
      
      // Calculate analytics data
      const totalInspections = batches.length;
      const completedInspections = batches.filter(batch => batch.qc_status === 'completed');
      
      const inspectionRate = totalInspections > 0 
        ? Math.round((completedInspections.length / totalInspections) * 100 * 10) / 10
        : 0;
      
      const complianceScore = totalInspections > 0
        ? Math.round((completedInspections.length / totalInspections) * 100 * 10) / 10
        : 0;
      
      analyticsData = {
        inspectionRate,
        complianceScore,
        avgProcessingTime: 2.3, // Mock value
        totalInspections
      };
      
      // Generate recent activities
      const recentBatches = batches
        .filter(batch => batch.last_inspection_date && new Date(batch.last_inspection_date) >= startOfMonth)
        .sort((a, b) => new Date(b.last_inspection_date) - new Date(a.last_inspection_date))
        .slice(0, 4);
      
      recentActivities = recentBatches.map((batch, index) => {
        const timeAgo = batch.last_inspection_date 
          ? Math.floor((now.getTime() - new Date(batch.last_inspection_date).getTime()) / (1000 * 60 * 60))
          : 0;
        
        let message = '';
        let status = 'info';
        
        if (batch.qc_status === 'completed') {
          message = `Batch ${batch.batch_id} inspection completed`;
          status = 'success';
        } else if (batch.qc_status === 'failed') {
          message = `Batch ${batch.batch_id} failed QC inspection`;
          status = 'error';
        } else if (batch.qc_status === 'pending') {
          message = `Batch ${batch.batch_id} inspection pending`;
          status = 'warning';
        }
        
        return {
          id: index + 1,
          type: 'inspection',
          message,
          time: timeAgo < 24 ? `${timeAgo} hours ago` : `${Math.floor(timeAgo / 24)} days ago`,
          status
        };
      });
      
      // Generate vendor performance
      vendorPerformance = vendors.slice(0, 4).map((vendor, index) => {
        const compliance = Math.floor(Math.random() * 20) + 80; // Mock compliance score
        let rating = 'Good';
        if (compliance >= 95) rating = 'Excellent';
        else if (compliance < 80) rating = 'Fair';
        
        return {
          name: vendor.city || `Vendor ${vendor.vendor_id}`,
          compliance,
          batches: vendor.no_of_batches || 0,
          rating
        };
      });
      
    } catch (err) {
      console.error('Error fetching viewer dashboard data:', err);
      error = 'Failed to load dashboard data. Please try again.';
    } finally {
      isLoading = false;
    }
  }

  onMount(async () => {
    const storedRole = localStorage.getItem('role');
    const storedUsername = localStorage.getItem('username');
    
    if (storedRole && storedUsername) {
      userRole = storedRole;
      username = storedUsername;
    }
    
    // Fetch dashboard data
    await fetchDashboardData();
  });
</script>

<Layout {userRole} {username} {activePage}>
  <div class="viewer-dashboard">
    <h1>Viewer Dashboard</h1>
    
    {#if isLoading}
      <div class="loading-state">
        <div class="loading-spinner"></div>
        <p>Loading dashboard data...</p>
      </div>
    {:else if error}
      <div class="error-state">
        <p class="error-message">{error}</p>
        <button class="retry-btn" on:click={fetchDashboardData}>Retry</button>
      </div>
    {:else}
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

    <!-- Analytics Overview -->
    <div class="analytics-section">
      <h2>Analytics Overview</h2>
      <div class="analytics-grid">
        <div class="analytics-card inspection-rate">
          <div class="analytics-content">
            <h3>Inspection Rate</h3>
            <p class="analytics-value">{analyticsData.inspectionRate}%</p>
            <span class="analytics-trend positive">+2.1% from last month</span>
          </div>
        </div>
        
        <div class="analytics-card compliance-score">
          <div class="analytics-content">
            <h3>Compliance Score</h3>
            <p class="analytics-value">{analyticsData.complianceScore}%</p>
            <span class="analytics-trend positive">+1.3% from last month</span>
          </div>
        </div>
        
        <div class="analytics-card processing-time">
          <div class="analytics-content">
            <h3>Avg Processing Time</h3>
            <p class="analytics-value">{analyticsData.avgProcessingTime} days</p>
            <span class="analytics-trend negative">+0.2 days from last month</span>
          </div>
        </div>
        
        <div class="analytics-card total-inspections">
          <div class="analytics-content">
            <h3>Total Inspections</h3>
            <p class="analytics-value">{analyticsData.totalInspections}</p>
            <span class="analytics-trend positive">+12 this month</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Activities & Vendor Performance -->
    <div class="content-grid">
      <div class="recent-activities">
        <h2>Recent Activities</h2>
        <div class="activities-list">
          {#each recentActivities as activity}
            <div class="activity-item activity-{activity.status}">
              <div class="activity-content">
                <p class="activity-message">{activity.message}</p>
                <span class="activity-time">{activity.time}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="vendor-performance">
        <h2>Vendor Performance</h2>
        <div class="vendor-list">
          {#each vendorPerformance as vendor}
            <div class="vendor-item">
              <div class="vendor-info">
                <h4>{vendor.name}</h4>
                <p>{vendor.batches} batches</p>
              </div>
              <div class="vendor-metrics">
                <div class="compliance-bar">
                  <div class="compliance-fill" style="width: {vendor.compliance}%"></div>
                </div>
                <span class="compliance-score">{vendor.compliance}%</span>
                <span class="vendor-rating rating-{vendor.rating.toLowerCase()}">{vendor.rating}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Detailed Analytics -->
    <div class="data-section">
      <h2>Detailed Analytics</h2>
      <div class="analytics-detailed-grid">
        {#each detailedAnalytics as metric}
          <div class="metric-card">
            <div class="metric-header">
              <h3>{metric.label}</h3>
              <span class="metric-change {metric.trend}">{metric.change}</span>
            </div>
            <div class="metric-value">
              {metric.value}{metric.suffix || ''}
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Compliance Metrics -->
    <div class="data-section">
      <h2>Compliance Metrics</h2>
      <div class="compliance-grid">
        {#each complianceMetrics as metric}
          <div class="compliance-card">
            <div class="compliance-header">
              <h3>{metric.label}</h3>
              <span class="compliance-percentage">{metric.percentage}%</span>
            </div>
            <div class="compliance-bar">
              <div class="compliance-fill" style="width: {metric.percentage}%"></div>
            </div>
            <div class="compliance-details">
              <span class="compliance-value">{metric.value}</span>
              <span class="compliance-total">/ {metric.total}</span>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- System Overview -->
    <div class="data-section">
      <h2>System Overview</h2>
      <div class="system-grid">
        {#each systemOverview as item}
          <div class="system-card">
            <div class="system-icon">{item.icon}</div>
            <div class="system-info">
              <h3>{item.label}</h3>
              <p class="system-value">{item.value}</p>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Trend Analysis -->
    <div class="data-section">
      <h2>7-Day Trend Analysis</h2>
      <div class="trend-container">
        <div class="trend-chart">
          <div class="trend-header">
            <h3>Batch Activity</h3>
            <div class="trend-legend">
              <span class="legend-item completed">Completed</span>
              <span class="legend-item failed">Failed</span>
              <span class="legend-item total">Total</span>
            </div>
          </div>
          <div class="trend-bars">
            {#each trendData as day}
              <div class="trend-bar-group">
                <div class="trend-bar completed" style="height: {(day.completed / Math.max(...trendData.map(d => d.completed))) * 100}%"></div>
                <div class="trend-bar failed" style="height: {(day.failed / Math.max(...trendData.map(d => d.failed))) * 100}%"></div>
                <div class="trend-bar total" style="height: {(day.batches / Math.max(...trendData.map(d => d.batches))) * 100}%"></div>
                <div class="trend-label">{new Date(day.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' })}</div>
              </div>
            {/each}
          </div>
        </div>
      </div>
    </div>

    <!-- Comprehensive Data Tables -->
    <div class="data-section">
      <h2>All Batches Overview</h2>
      <div class="table-container">
        <table class="data-table">
          <thead>
            <tr>
              <th>Batch ID</th>
              <th>Vendor</th>
              <th>Production Date</th>
              <th>Status</th>
              <th>Expiry Date</th>
              <th>Location</th>
              <th>QR Hash</th>
            </tr>
          </thead>
          <tbody>
            {#each allBatches.slice(0, 15) as batch}
              <tr>
                <td>{batch.batch_id}</td>
                <td>
                  {#each allVendors}
                    {#if vendor_id === batch.vendor_id}
                      {city || vendor_id}
                    {/if}
                  {/each}
                </td>
                <td>{batch.date_of_production || 'N/A'}</td>
                <td><span class="status-badge {batch.qc_status?.toLowerCase().replace(' ', '-')}">{batch.qc_status || 'Unknown'}</span></td>
                <td>{batch.expiry_date || 'N/A'}</td>
                <td>{batch.fitment_location || 'TBD'}</td>
                <td class="qr-hash">{batch.qr_hash || 'N/A'}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- All Vendors Overview -->
    <div class="data-section">
      <h2>All Vendors Overview</h2>
      <div class="table-container">
        <table class="data-table">
          <thead>
            <tr>
              <th>Vendor ID</th>
              <th>Location</th>
              <th>Contact</th>
              <th>GST No</th>
              <th>PAN</th>
              <th>Batches</th>
              <th>Last Audit</th>
            </tr>
          </thead>
          <tbody>
            {#each allVendors.slice(0, 15) as vendor}
              <tr>
                <td>{vendor.vendor_id}</td>
                <td>{vendor.city}, {vendor.state}</td>
                <td>{vendor.email || vendor.phone_number || 'N/A'}</td>
                <td>{vendor.gst_no || 'N/A'}</td>
                <td>{vendor.pan_number || 'N/A'}</td>
                <td>{vendor.no_of_batches || 0}</td>
                <td>{vendor.audit_date || 'Never'}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
    {/if}
  </div>
</Layout>

<style>
  .viewer-dashboard {
    padding: 2rem;
  }

  .viewer-dashboard h1 {
    color: #000000;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
  }

  .summary-cards {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1.5rem;
    margin-bottom: 3rem;
  }

  @media (max-width: 1200px) {
    .summary-cards {
      grid-template-columns: repeat(2, 1fr);
      gap: 1rem;
    }
    
    .analytics-grid {
      grid-template-columns: repeat(2, 1fr);
      gap: 1.25rem;
    }
    
    .content-grid {
      grid-template-columns: 1fr;
      gap: 1.5rem;
    }
  }

  @media (max-width: 768px) {
    .viewer-dashboard {
      padding: 1rem;
    }
    
    .analytics-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
    
    .analytics-card {
      padding: 1.5rem;
      min-height: 120px;
    }
    
    .summary-cards {
      grid-template-columns: 1fr;
      gap: 1rem;
      margin-bottom: 2rem;
    }
    
    .summary-card {
      padding: 1rem;
    }
    
    .analytics-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
    
    .analytics-card {
      padding: 1.5rem;
      min-height: 120px;
    }
    
    .content-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
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
    padding: 0.75rem 1rem;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    transition: all 0.3s ease;
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


  .analytics-section {
    margin-bottom: 3rem;
  }

  .analytics-section h2 {
    color: #000000;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .analytics-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .analytics-card {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
    transition: all 0.3s ease;
    border-left: 4px solid #e5e7eb;
    min-height: 140px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .analytics-card.inspection-rate {
    border-left-color: #10b981;
  }

  .analytics-card.compliance-score {
    border-left-color: #3b82f6;
  }

  .analytics-card.processing-time {
    border-left-color: #f59e0b;
  }

  .analytics-card.total-inspections {
    border-left-color: #8b5cf6;
  }

  .analytics-card:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .analytics-content h3 {
    margin: 0 0 0.75rem 0;
    color: #64748b;
    font-size: 0.9rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .analytics-value {
    margin: 0 0 0.75rem 0;
    font-size: 2.5rem;
    font-weight: 700;
    color: #000000;
    line-height: 1;
  }

  .analytics-trend {
    font-size: 0.8rem;
    font-weight: 500;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    display: inline-block;
  }

  .analytics-trend.positive {
    color: #059669;
    background: rgba(5, 150, 105, 0.1);
  }

  .analytics-trend.negative {
    color: #dc2626;
    background: rgba(220, 38, 38, 0.1);
  }

  .content-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    margin-bottom: 2rem;
  }

  .recent-activities h2,
  .vendor-performance h2 {
    color: #000000;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .activities-list {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .activity-item {
    padding: 1rem;
    border-bottom: 1px solid #f1f5f9;
    transition: background 0.3s ease;
    border-left: 3px solid #e5e7eb;
  }

  .activity-item:last-child {
    border-bottom: none;
  }

  .activity-item:hover {
    background: #f8fafc;
  }

  .activity-success {
    border-left-color: #10b981;
  }

  .activity-info {
    border-left-color: #3b82f6;
  }

  .activity-warning {
    border-left-color: #f59e0b;
  }

  .activity-error {
    border-left-color: #ef4444;
  }


  .activity-message {
    margin: 0 0 0.25rem 0;
    color: #374151;
    font-size: 0.875rem;
  }

  .activity-time {
    color: #9ca3af;
    font-size: 0.75rem;
  }

  .vendor-list {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .vendor-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    border-bottom: 1px solid #f1f5f9;
  }

  .vendor-item:last-child {
    border-bottom: none;
  }

  .vendor-info h4 {
    margin: 0 0 0.25rem 0;
    color: #000000;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .vendor-info p {
    margin: 0;
    color: #64748b;
    font-size: 0.75rem;
  }

  .vendor-metrics {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .compliance-bar {
    width: 60px;
    height: 6px;
    background: #e5e7eb;
    border-radius: 3px;
    overflow: hidden;
  }

  .compliance-fill {
    height: 100%;
    background: linear-gradient(90deg, #ef4444 0%, #f59e0b 50%, #10b981 100%);
    border-radius: 3px;
    transition: width 0.3s ease;
  }

  .compliance-score {
    color: #374151;
    font-size: 0.75rem;
    font-weight: 600;
    min-width: 35px;
  }

  .vendor-rating {
    padding: 0.125rem 0.5rem;
    border-radius: 4px;
    font-size: 0.625rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .rating-excellent {
    background: #dcfce7;
    color: #166534;
  }

  .rating-good {
    background: #dbeafe;
    color: #1e40af;
  }

  .rating-fair {
    background: #fef3c7;
    color: #92400e;
  }

  /* Loading and Error States */
  .loading-state, .error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    text-align: center;
  }

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e2e8f0;
    border-top: 4px solid #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .error-message {
    color: #ef4444;
    margin-bottom: 1rem;
    font-size: 1.1rem;
  }

  .retry-btn {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 500;
    transition: all 0.3s ease;
  }

  .retry-btn:hover {
    background: #2563eb;
    transform: translateY(-1px);
  }

  /* Data Sections */
  .data-section {
    margin-bottom: 3rem;
  }

  .data-section h2 {
    color: #000000;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .table-container {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .data-table {
    width: 100%;
    border-collapse: collapse;
  }

  .data-table th {
    background: #f8fafc;
    color: #374151;
    font-weight: 600;
    padding: 1rem;
    text-align: left;
    border-bottom: 1px solid #e5e7eb;
    font-size: 0.875rem;
  }

  .data-table td {
    padding: 1rem;
    border-bottom: 1px solid #f3f4f6;
    color: #374151;
    font-size: 0.875rem;
  }

  .data-table tr:hover {
    background: #f9fafb;
  }

  .status-badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-badge.pass {
    background: #dcfce7;
    color: #166534;
  }

  .status-badge.fail {
    background: #fef2f2;
    color: #dc2626;
  }

  .status-badge.pending-inspection {
    background: #fef3c7;
    color: #d97706;
  }

  .status-badge.unknown {
    background: #f3f4f6;
    color: #6b7280;
  }

  .qr-hash {
    font-family: monospace;
    font-size: 0.75rem;
    max-width: 100px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* Detailed Analytics */
  .analytics-detailed-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .metric-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    border-left: 4px solid #3b82f6;
  }

  .metric-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .metric-header h3 {
    color: #374151;
    font-size: 0.875rem;
    font-weight: 600;
    margin: 0;
  }

  .metric-change {
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
  }

  .metric-change.up {
    background: #dcfce7;
    color: #166534;
  }

  .metric-change.down {
    background: #fef2f2;
    color: #dc2626;
  }

  .metric-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: #000000;
  }

  /* Compliance Metrics */
  .compliance-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .compliance-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .compliance-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .compliance-header h3 {
    color: #374151;
    font-size: 0.875rem;
    font-weight: 600;
    margin: 0;
  }

  .compliance-percentage {
    color: #3b82f6;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .compliance-bar {
    background: #e5e7eb;
    height: 8px;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }

  .compliance-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6 0%, #1d4ed8 100%);
    border-radius: 4px;
    transition: width 0.3s ease;
  }

  .compliance-details {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .compliance-value {
    color: #000000;
    font-weight: 600;
  }

  .compliance-total {
    color: #6b7280;
    font-size: 0.875rem;
  }

  /* System Overview */
  .system-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .system-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .system-icon {
    font-size: 2rem;
  }

  .system-info h3 {
    color: #374151;
    font-size: 0.875rem;
    font-weight: 600;
    margin: 0 0 0.25rem 0;
  }

  .system-value {
    color: #000000;
    font-size: 1.25rem;
    font-weight: 700;
    margin: 0;
  }

  /* Trend Analysis */
  .trend-container {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
  }

  .trend-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .trend-header h3 {
    color: #000000;
    font-size: 1.125rem;
    font-weight: 600;
    margin: 0;
  }

  .trend-legend {
    display: flex;
    gap: 1rem;
  }

  .legend-item {
    font-size: 0.75rem;
    font-weight: 500;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
  }

  .legend-item.completed {
    background: #dcfce7;
    color: #166534;
  }

  .legend-item.failed {
    background: #fef2f2;
    color: #dc2626;
  }

  .legend-item.total {
    background: #dbeafe;
    color: #1e40af;
  }

  .trend-bars {
    display: flex;
    align-items: end;
    gap: 0.5rem;
    height: 200px;
  }

  .trend-bar-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    flex: 1;
  }

  .trend-bar {
    width: 100%;
    border-radius: 2px 2px 0 0;
    min-height: 2px;
  }

  .trend-bar.completed {
    background: #10b981;
  }

  .trend-bar.failed {
    background: #ef4444;
  }

  .trend-bar.total {
    background: #3b82f6;
  }

  .trend-label {
    font-size: 0.75rem;
    color: #6b7280;
    text-align: center;
  }

  @media (max-width: 768px) {
    .data-section {
      margin-bottom: 2rem;
    }

    .table-container {
      overflow-x: auto;
    }

    .data-table {
      min-width: 600px;
    }

    .analytics-detailed-grid,
    .compliance-grid,
    .system-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }

    .trend-bars {
      height: 150px;
    }

    .trend-legend {
      flex-direction: column;
      gap: 0.5rem;
    }
  }
</style>
