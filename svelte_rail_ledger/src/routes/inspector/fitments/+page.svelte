<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/inspector/fitments';
  let showAddForm = false;

  let fitments = [
    { batch_id: 'B001', fitment_date: '2024-02-15', location: 'Platform 1, Mumbai Central', photo: 'photo1.jpg' },
    { batch_id: 'B002', fitment_date: '2024-02-16', location: 'Platform 2, Delhi Station', photo: 'photo2.jpg' }
  ];

  let newFitment = {
    batch_id: '',
    fitment_date: '',
    location: '',
    photo: ''
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

  function addFitment() {
    if (newFitment.batch_id && newFitment.fitment_date && newFitment.location) {
      fitments = [...fitments, { ...newFitment }];
      newFitment = { batch_id: '', fitment_date: '', location: '', photo: '' };
      showAddForm = false;
    }
  }
</script>

<Layout {userRole} {username} {activePage}>
  <div class="fitments-page">
    <div class="page-header">
      <h1>Fitments</h1>
      <button class="add-btn" on:click={toggleAddForm}>
        {showAddForm ? 'Cancel' : 'Add Fitment'}
      </button>
    </div>

    {#if showAddForm}
      <div class="add-form">
        <h2>Add Fitment</h2>
        <div class="form-grid">
          <div class="form-group">
            <label>Batch ID</label>
            <input type="text" bind:value={newFitment.batch_id} />
          </div>
          <div class="form-group">
            <label>Fitment Date</label>
            <input type="date" bind:value={newFitment.fitment_date} />
          </div>
          <div class="form-group">
            <label>Location</label>
            <input type="text" bind:value={newFitment.location} />
          </div>
          <div class="form-group">
            <label>Upload Photo</label>
            <input type="file" accept="image/*" />
          </div>
        </div>
        <button class="save-btn" on:click={addFitment}>Save</button>
      </div>
    {/if}

    <div class="table-container">
      <table class="fitments-table">
        <thead>
          <tr>
            <th>Batch ID</th>
            <th>Fitment Date</th>
            <th>Location</th>
            <th>Photo</th>
          </tr>
        </thead>
        <tbody>
          {#each fitments as fitment}
            <tr>
              <td>{fitment.batch_id}</td>
              <td>{fitment.fitment_date}</td>
              <td>{fitment.location}</td>
              <td>{fitment.photo}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
</Layout>

<style>
  .fitments-page {
    padding: 2rem;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .fitments-page h1 {
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

  .form-group input {
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

  .fitments-table {
    width: 100%;
    border-collapse: collapse;
  }

  .fitments-table th {
    background: #f8fafc;
    padding: 1rem;
    text-align: left;
    font-weight: 600;
    color: #374151;
  }

  .fitments-table td {
    padding: 1rem;
    border-bottom: 1px solid #f1f5f9;
    color: #64748b;
  }
</style>
