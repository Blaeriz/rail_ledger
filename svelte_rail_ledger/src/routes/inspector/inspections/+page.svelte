<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/inspector/inspections';
  let showAddForm = false;

  let inspections = [
    { batch_id: 'B001', inspection_date: '2024-02-10', type: 'Quality Check', status: 'Pass', remarks: 'All good' },
    { batch_id: 'B002', inspection_date: '2024-02-12', type: 'Safety Check', status: 'Fail', remarks: 'Minor issues found' }
  ];

  let newInspection = {
    batch_id: '',
    inspection_date: '',
    type: 'Quality Check',
    status: 'Pass',
    remarks: ''
  };

  onMount(() => {
    const storedRole = localStorage.getItem('role');
    const storedUsername = localStorage.getItem('username');
    
    if (storedRole && storedUsername) {
      userRole = storedRole;
      username = storedUsername;
    }
  });

  function toggleAddForm() {
    showAddForm = !showAddForm;
  }

  function addInspection() {
    if (newInspection.batch_id && newInspection.inspection_date) {
      inspections = [...inspections, { ...newInspection }];
      newInspection = { batch_id: '', inspection_date: '', type: 'Quality Check', status: 'Pass', remarks: '' };
      showAddForm = false;
    }
  }
</script>

<Layout {userRole} {username} {activePage}>
  <div class="inspections-page">
    <div class="page-header">
      <h1>Inspections</h1>
      <button class="add-btn" on:click={toggleAddForm}>
        {showAddForm ? 'Cancel' : 'Add Inspection'}
      </button>
    </div>

    {#if showAddForm}
      <div class="add-form">
        <h2>Add Inspection</h2>
        <div class="form-grid">
          <div class="form-group">
            <label>Batch ID</label>
            <input type="text" bind:value={newInspection.batch_id} />
          </div>
          <div class="form-group">
            <label>Inspection Date</label>
            <input type="date" bind:value={newInspection.inspection_date} />
          </div>
          <div class="form-group">
            <label>Type</label>
            <select bind:value={newInspection.type}>
              <option value="Quality Check">Quality Check</option>
              <option value="Safety Check">Safety Check</option>
            </select>
          </div>
          <div class="form-group">
            <label>Status</label>
            <select bind:value={newInspection.status}>
              <option value="Pass">Pass</option>
              <option value="Fail">Fail</option>
            </select>
          </div>
          <div class="form-group">
            <label>Remarks</label>
            <textarea bind:value={newInspection.remarks}></textarea>
          </div>
        </div>
        <button class="save-btn" on:click={addInspection}>Save</button>
      </div>
    {/if}

    <div class="table-container">
      <table class="inspections-table">
        <thead>
          <tr>
            <th>Batch ID</th>
            <th>Inspection Date</th>
            <th>Type</th>
            <th>Status</th>
            <th>Remarks</th>
          </tr>
        </thead>
        <tbody>
          {#each inspections as inspection}
            <tr>
              <td>{inspection.batch_id}</td>
              <td>{inspection.inspection_date}</td>
              <td>{inspection.type}</td>
              <td>
                <span class="status status-{inspection.status.toLowerCase()}">
                  {inspection.status}
                </span>
              </td>
              <td>{inspection.remarks}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</Layout>

<style>
  .inspections-page {
    padding: 2rem;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .inspections-page h1 {
    color: #1e293b;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .add-btn {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
  }

  .add-form {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    margin-bottom: 2rem;
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group input, .form-group select, .form-group textarea {
    padding: 0.75rem;
    border: 2px solid #e5e7eb;
    border-radius: 6px;
  }

  .save-btn {
    background: #10b981;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
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
  }

  .inspections-table td {
    padding: 1rem;
    border-bottom: 1px solid #f1f5f9;
    color: #64748b;
  }

  .status {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
  }

  .status-pass {
    background: #d1fae5;
    color: #065f46;
  }

  .status-fail {
    background: #fee2e2;
    color: #991b1b;
  }
</style>
