<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/inspector/scan';
  let batchId = '';
  let batchDetails = null;

  onMount(() => {
    const storedRole = localStorage.getItem('role');
    const storedUsername = localStorage.getItem('username');
    
    if (storedRole && storedUsername) {
      userRole = storedRole;
      username = storedUsername;
    }

    // Check if we have a batch ID from the dashboard
    const currentBatch = localStorage.getItem('currentInspectionBatch');
    if (currentBatch) {
      batchId = currentBatch;
      searchBatch();
      // Clear the stored batch ID after using it
      localStorage.removeItem('currentInspectionBatch');
    }
  });

  function scanQR() {
    alert('QR Scanner would open here');
  }

  function searchBatch() {
    if (batchId) {
      batchDetails = {
        batch_id: batchId,
        vendor: 'Steel Works Ltd',
        production_date: '2024-01-10',
        qc_status: 'Pass',
        expiry_date: '2025-01-10'
      };
    }
  }
</script>

<Layout {userRole} {username} {activePage}>
  <div class="scan-page">
    <h1>QR Scanner</h1>
    
    <div class="scan-section">
      <div class="camera-view">
        <div class="camera-placeholder">
          <span class="camera-icon">📱</span>
          <p>Camera View</p>
          <button class="scan-btn" on:click={scanQR}>Start Scanning</button>
        </div>
      </div>
      
      <div class="manual-entry">
        <h3>Manual Entry</h3>
        <div class="input-group">
          <input type="text" bind:value={batchId} placeholder="Enter Batch ID" />
          <button on:click={searchBatch}>Search</button>
        </div>
      </div>
    </div>

    {#if batchDetails}
      <div class="batch-details">
        <h3>Batch Details</h3>
        <div class="details-grid">
          <div class="detail-item">
            <label>Batch ID:</label>
            <span>{batchDetails.batch_id}</span>
          </div>
          <div class="detail-item">
            <label>Vendor:</label>
            <span>{batchDetails.vendor}</span>
          </div>
          <div class="detail-item">
            <label>Production Date:</label>
            <span>{batchDetails.production_date}</span>
          </div>
          <div class="detail-item">
            <label>QC Status:</label>
            <span class="status status-{batchDetails.qc_status.toLowerCase()}">{batchDetails.qc_status}</span>
          </div>
          <div class="detail-item">
            <label>Expiry Date:</label>
            <span>{batchDetails.expiry_date}</span>
          </div>
        </div>
      </div>
    {/if}
  </div>
</Layout>

<style>
  .scan-page {
    padding: 2rem;
  }

  .scan-page h1 {
    color: #000000;
    margin-bottom: 2rem;
    font-size: 1.75rem;
    font-weight: 700;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .scan-section {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    margin-bottom: 2rem;
  }

  .camera-view, .manual-entry {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .camera-placeholder {
    text-align: center;
    padding: 2rem;
    background: #f8fafc;
    border: 2px dashed #cbd5e1;
    border-radius: 8px;
  }

  .camera-icon {
    font-size: 3rem;
    display: block;
    margin-bottom: 1rem;
  }

  .scan-btn {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    margin-top: 1rem;
  }

  .input-group {
    display: flex;
    gap: 1rem;
  }

  .input-group input {
    flex: 1;
    padding: 0.75rem;
    border: 2px solid #e5e7eb;
    border-radius: 6px;
  }

  .input-group button {
    background: #10b981;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
  }

  .batch-details {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .detail-item label {
    font-weight: 600;
    color: #374151;
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
</style>
