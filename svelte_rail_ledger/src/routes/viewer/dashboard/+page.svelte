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

  // Enhanced data for viewer dashboard
  let analyticsData = {
    inspectionRate: 95.2,
    complianceScore: 88.7,
    avgProcessingTime: 2.3,
    totalInspections: 156
  };

  let recentActivities = [
    { id: 1, type: 'inspection', message: 'Batch B001 inspection completed', time: '2 hours ago', status: 'success' },
    { id: 2, type: 'batch', message: 'New batch B012 created by Steel Works Ltd', time: '4 hours ago', status: 'info' },
    { id: 3, type: 'vendor', message: 'Vendor audit scheduled for Metal Solutions Inc', time: '1 day ago', status: 'warning' },
    { id: 4, type: 'failure', message: 'Batch B008 failed quality check', time: '2 days ago', status: 'error' }
  ];

  let vendorPerformance = [
    { name: 'Steel Works Ltd', compliance: 98, batches: 45, rating: 'Excellent' },
    { name: 'Metal Solutions Inc', compliance: 92, batches: 32, rating: 'Good' },
    { name: 'Rail Components Co', compliance: 87, batches: 28, rating: 'Good' },
    { name: 'Quality Steel Corp', compliance: 78, batches: 15, rating: 'Fair' }
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
  <div class="viewer-dashboard">
    <h1>Viewer Dashboard</h1>
    
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

</style>
