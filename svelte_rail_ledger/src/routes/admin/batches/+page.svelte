<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';

  let userRole = '';
  let username = '';
  let activePage = '/admin/batches';
  let showAddForm = false;

  // Batch data from API
  let batches = [];
  let vendors = [];
  let isLoading = true;

  // Form data
  let newBatch = {
    vendor_id: '',
    batch_size: '',
    production_date: '',
    qc_status: 'Pending Inspection',
    expiry_date: ''
  };

  onMount(async () => {
    const storedRole = localStorage.getItem('role');
    const storedUsername = localStorage.getItem('username');
    
    if (storedRole && storedUsername) {
      userRole = storedRole;
      username = storedUsername;
    }

    // Check if we should auto-open the form
    const urlParams = new URLSearchParams($page.url.search);
    if (urlParams.get('action') === 'add') {
      showAddForm = true;
    }

    // Fetch data from APIs
    try {
      const [batchesResponse, vendorsResponse] = await Promise.all([
        fetch('/api/batches'),
        fetch('/api/vendors')
      ]);

      if (batchesResponse.ok) {
        batches = await batchesResponse.json();
        console.log('Batches loaded:', batches.length);
      } else {
        console.error('Failed to fetch batches:', batchesResponse.status);
      }
      
      if (vendorsResponse.ok) {
        vendors = await vendorsResponse.json();
        console.log('Vendors loaded:', vendors.length);
      } else {
        console.error('Failed to fetch vendors:', vendorsResponse.status);
      }
    } catch (error) {
      console.error('Error fetching data:', error);
    }
    
    isLoading = false;
  });

  function toggleAddForm() {
    showAddForm = !showAddForm;
    if (!showAddForm) {
      resetForm();
    }
  }

  function resetForm() {
    newBatch = {
      vendor_id: '',
      batch_size: '',
      production_date: '',
      qc_status: 'Pending Inspection',
      expiry_date: ''
    };
  }

  async function addBatch() {
    if (newBatch.vendor_id && newBatch.batch_size && newBatch.production_date) {
      try {
        const batchData = {
          batch_id: `B${String(batches.length + 1).padStart(3, '0')}`,
          vendor_id: newBatch.vendor_id,
          batch_size: parseInt(newBatch.batch_size),
          date_of_production: newBatch.production_date,
          qc_status: newBatch.qc_status.toLowerCase(),
          expiry_date: newBatch.expiry_date || null,
          last_inspection_date: null,
          fitment_date: null,
          fitment_location: null,
          qr_hash: `qr_hash_${String(batches.length + 1).padStart(3, '0')}_${Math.random().toString(36).substr(2, 9)}`
        };

        const response = await fetch('/api/batches', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(batchData)
        });

        if (response.ok) {
          // Refresh the batches list
          const batchesResponse = await fetch('/api/batches');
          if (batchesResponse.ok) {
            batches = await batchesResponse.json();
          }
          resetForm();
          showAddForm = false;
        } else {
          console.error('Failed to add batch');
        }
      } catch (error) {
        console.error('Error adding batch:', error);
      }
    }
  }

  function generateQR(batchId) {
    alert(`QR Code generated for batch ${batchId}`);
  }

  function deleteBatch(batchId) {
    batches = batches.filter(b => b.batch_id !== batchId);
  }
</script>

<Layout {userRole} {username} {activePage}>
  <div class="batches-page">
    <div class="page-header">
      <h1>Batch Management</h1>
      <button class="add-btn" on:click={toggleAddForm}>
        {showAddForm ? 'Cancel' : 'Add Batch'}
      </button>
    </div>

    <!-- Add Batch Form -->
    {#if showAddForm}
      <div class="add-form">
        <h2>Add New Batch</h2>
        <div class="form-grid">
          <div class="form-group">
            <label for="batch-vendor">Vendor</label>
            <select id="batch-vendor" bind:value={newBatch.vendor_id}>
              <option value="">Select Vendor</option>
              {#each vendors as vendor}
                <option value={vendor.vendor_id}>{vendor.name}</option>
              {/each}
            </select>
          </div>
          <div class="form-group">
            <label for="batch-size">Batch Size</label>
            <input id="batch-size" type="number" bind:value={newBatch.batch_size} placeholder="Enter batch size" />
          </div>
          <div class="form-group">
            <label for="production-date">Production Date</label>
            <input id="production-date" type="date" bind:value={newBatch.production_date} />
          </div>
          <div class="form-group">
            <label for="qc-status">QC Status</label>
            <select id="qc-status" bind:value={newBatch.qc_status}>
              <option value="Pending Inspection">Pending Inspection</option>
              <option value="Pass">Pass</option>
              <option value="Fail">Fail</option>
            </select>
          </div>
          <div class="form-group">
            <label for="expiry-date">Expiry Date</label>
            <input id="expiry-date" type="date" bind:value={newBatch.expiry_date} />
          </div>
        </div>
        <div class="form-actions">
          <button class="save-btn" on:click={addBatch}>Save Batch</button>
          <button class="cancel-btn" on:click={toggleAddForm}>Cancel</button>
        </div>
      </div>
    {/if}

    <!-- Batches Table -->
    <div class="table-container">
      <div class="table-header">
        <h2>Batch List</h2>
        <span class="count">{batches.length} batches</span>
      </div>
      
      {#if isLoading}
        <div class="loading">Loading batches...</div>
      {:else if batches.length === 0}
        <div class="loading">No batches found in database</div>
      {:else}
      <div class="table-wrapper">
        <table class="batches-table">
          <thead>
            <tr>
              <th>Batch ID</th>
              <th>Vendor</th>
              <th>Batch Size</th>
              <th>Production Date</th>
              <th>QC Status</th>
              <th>Fitment Status</th>
              <th>Expiry Date</th>
              <th>QR Hash</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each batches as batch}
              <tr>
                <td>{batch.batch_id}</td>
                <td>
                  {#each vendors as vendor}
                    {#if vendor.vendor_id === batch.vendor_id}
                      {vendor.city || batch.vendor_id}
                    {/if}
                  {/each}
                </td>
                <td>{batch.batch_size}</td>
                <td>{batch.date_of_production}</td>
                <td>
                  <span class="status status-{batch.qc_status?.toLowerCase().replace(' ', '-')}">
                    {batch.qc_status}
                  </span>
                </td>
                <td>
                  <span class="status status-{batch.fitment_date ? 'completed' : 'not-started'}">
                    {batch.fitment_date ? 'Completed' : 'Not Started'}
                  </span>
                </td>
                <td>{batch.expiry_date}</td>
                <td class="qr-hash">{batch.qr_hash}</td>
                <td class="actions">
                  <button class="qr-btn" on:click={() => generateQR(batch.batch_id)}>Generate QR</button>
                  <button class="delete-btn" on:click={() => deleteBatch(batch.batch_id)}>Delete</button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
      {/if}
    </div>
  </div>
</Layout>

<style>
  .batches-page {
    padding: 2rem;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .page-header h1 {
    color: #000000;
    margin: 0;
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
    font-weight: 500;
    transition: background 0.3s;
  }

  .add-btn:hover {
    background: #2563eb;
  }

  .add-form {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    margin-bottom: 2rem;
  }

  .add-form h2 {
    color: #000000;
    margin: 0 0 1.5rem 0;
    font-size: 1.5rem;
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group label {
    margin-bottom: 0.5rem;
    color: #374151;
    font-weight: 500;
  }

  .form-group input, .form-group select {
    padding: 0.75rem;
    border: 2px solid #e5e7eb;
    border-radius: 6px;
    font-size: 0.9rem;
    transition: border-color 0.3s;
  }

  .form-group input:focus, .form-group select:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .form-actions {
    display: flex;
    gap: 1rem;
  }

  .save-btn {
    background: #10b981;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
  }

  .cancel-btn {
    background: #6b7280;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 500;
  }

  .table-container {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  .table-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #e5e7eb;
  }

  .table-header h2 {
    color: #000000;
    margin: 0;
    font-size: 1.25rem;
  }

  .count {
    color: #64748b;
    font-size: 0.9rem;
  }

  .table-wrapper {
    overflow-x: auto;
  }

  .batches-table {
    width: 100%;
    border-collapse: collapse;
    min-width: 600px;
  }

  .table-container {
    overflow-x: auto;
  }

  .batches-table th {
    background: #f8fafc;
    padding: 1rem;
    text-align: left;
    font-weight: 600;
    color: #374151;
    border-bottom: 1px solid #e5e7eb;
  }

  .batches-table td {
    padding: 1rem;
    border-bottom: 1px solid #f1f5f9;
    color: #64748b;
  }

  .batches-table tr:hover {
    background: #f8fafc;
  }

  .status {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-pass, .status-completed {
    background: #d1fae5;
    color: #065f46;
  }

  .status-fail, .status-not-started {
    background: #fee2e2;
    color: #991b1b;
  }

  .status-pending-inspection {
    background: #fef3c7;
    color: #92400e;
  }

  .qr-hash {
    font-family: monospace;
    font-size: 0.8rem;
    max-width: 150px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  @media (max-width: 768px) {
    .batches-page {
      padding: 1rem;
    }
    
    .page-header {
      flex-direction: column;
      gap: 1rem;
      align-items: stretch;
    }
    
    .add-form {
      padding: 1rem;
    }
    
    .form-grid {
      grid-template-columns: 1fr;
    }
    
    .actions {
      flex-direction: column;
    }
  }

  @media (max-width: 480px) {
    .batches-page {
      padding: 0.5rem;
    }
    
    .batches-table th,
    .batches-table td {
      padding: 0.5rem 0.25rem;
      font-size: 0.875rem;
    }
  }

  .qr-btn, .delete-btn {
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .qr-btn {
    background: #8b5cf6;
    color: white;
  }

  .delete-btn {
    background: #ef4444;
    color: white;
  }

  .qr-btn:hover {
    background: #7c3aed;
  }

  .delete-btn:hover {
    background: #dc2626;
  }

  .loading {
    text-align: center;
    padding: 2rem;
    font-size: 1.1rem;
    color: #64748b;
  }
</style>
