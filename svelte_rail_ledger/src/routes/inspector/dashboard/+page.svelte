<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/inspector/dashboard';

  // Sample inspection data
  let assignedInspections = [
    {
      batch_id: 'B001',
      due_date: '2024-02-15',
      status: 'Pending',
      priority: 'High',
      vendor: 'Steel Works Ltd',
      location: 'Mumbai Central'
    },
    {
      batch_id: 'B002',
      due_date: '2024-02-18',
      status: 'In Progress',
      priority: 'Medium',
      vendor: 'Steel Works Ltd',
      location: 'Mumbai Central'
    },
    {
      batch_id: 'B003',
      due_date: '2024-02-20',
      status: 'Pending',
      priority: 'Low',
      vendor: 'Metal Solutions Inc',
      location: 'Delhi Station'
    }
  ];

  let alerts = [
    { id: 1, type: 'warning', message: 'Batch #B001 inspection overdue by 2 days', priority: 'high' },
    { id: 2, type: 'info', message: 'New inspection assigned for Batch #B004', priority: 'medium' },
    { id: 3, type: 'error', message: 'Batch #B002 failed initial inspection', priority: 'high' }
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
  <div class="inspector-dashboard">
    <h1>Inspector Dashboard</h1>
    
    <!-- Summary Cards -->
    <div class="summary-cards">
      <div class="summary-card">
        <div class="card-icon">📋</div>
        <div class="card-content">
          <h3>Assigned Inspections</h3>
          <p class="card-number">{assignedInspections.length}</p>
        </div>
      </div>
      
      <div class="summary-card">
        <div class="card-icon">⏰</div>
        <div class="card-content">
          <h3>Overdue</h3>
          <p class="card-number">{assignedInspections.filter(i => i.status === 'Pending' && new Date(i.due_date) < new Date()).length}</p>
        </div>
      </div>
      
      <div class="summary-card">
        <div class="card-icon">✅</div>
        <div class="card-content">
          <h3>Completed Today</h3>
          <p class="card-number">3</p>
        </div>
      </div>
      
      <div class="summary-card">
        <div class="card-icon">❌</div>
        <div class="card-content">
          <h3>Failed Inspections</h3>
          <p class="card-number">1</p>
        </div>
      </div>
    </div>

    <!-- Alerts Section -->
    <div class="alerts-section">
      <h2>Alerts & Notifications</h2>
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

    <!-- Assigned Inspections -->
    <div class="inspections-section">
      <h2>Assigned Inspections</h2>
      <div class="table-container">
        <table class="inspections-table">
          <thead>
            <tr>
              <th>Batch ID</th>
              <th>Vendor</th>
              <th>Location</th>
              <th>Due Date</th>
              <th>Status</th>
              <th>Priority</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each assignedInspections as inspection}
              <tr>
                <td>{inspection.batch_id}</td>
                <td>{inspection.vendor}</td>
                <td>{inspection.location}</td>
                <td>{inspection.due_date}</td>
                <td>
                  <span class="status status-{inspection.status.toLowerCase().replace(' ', '-')}">
                    {inspection.status}
                  </span>
                </td>
                <td>
                  <span class="priority priority-{inspection.priority.toLowerCase()}">
                    {inspection.priority}
                  </span>
                </td>
                <td class="actions">
                  <button class="inspect-btn">Start Inspection</button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</Layout>

<style>
  .inspector-dashboard {
    padding: 2rem;
  }

  .inspector-dashboard h1 {
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
    .inspector-dashboard {
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
    
    .inspections-table {
      font-size: 0.875rem;
    }
  }

  @media (max-width: 480px) {
    .inspector-dashboard {
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
    transition: transform 0.3s;
  }

  .summary-card:hover {
    transform: translateY(-2px);
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

  .inspections-section h2 {
    color: #1e293b;
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
  }

  .table-container {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .inspections-table {
    width: 100%;
    border-collapse: collapse;
  }

  .inspections-table th {
    background: #f8fafc;
    padding: 1rem;
    text-align: left;
    font-weight: 600;
    color: #374151;
    border-bottom: 1px solid #e5e7eb;
  }

  .inspections-table td {
    padding: 1rem;
    border-bottom: 1px solid #f1f5f9;
    color: #64748b;
  }

  .inspections-table tr:hover {
    background: #f8fafc;
  }

  .status {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-pending {
    background: #fef3c7;
    color: #92400e;
  }

  .status-in-progress {
    background: #dbeafe;
    color: #1e40af;
  }

  .priority {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
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

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  .inspect-btn {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .inspect-btn:hover {
    background: #2563eb;
  }
</style>
