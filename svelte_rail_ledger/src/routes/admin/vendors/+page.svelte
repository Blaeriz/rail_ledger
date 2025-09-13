<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/admin/vendors';
  let showAddForm = false;

  // Sample vendor data
  let vendors = [
    {
      vendor_id: 'V001',
      name: 'Steel Works Ltd',
      city: 'Mumbai',
      state: 'Maharashtra',
      audit_date: '2024-01-15',
      gst_no: '29ABCDE1234F1Z5',
      pan_number: 'ABCDE1234F',
      email: 'contact@steelworks.com',
      phone: '+91-9876543210'
    },
    {
      vendor_id: 'V002',
      name: 'Metal Solutions Inc',
      city: 'Delhi',
      state: 'Delhi',
      audit_date: '2024-02-10',
      gst_no: '07FGHIJ5678K2L6',
      pan_number: 'FGHIJ5678K',
      email: 'info@metalsolutions.com',
      phone: '+91-9876543211'
    },
    {
      vendor_id: 'V003',
      name: 'Rail Components Co',
      city: 'Bangalore',
      state: 'Karnataka',
      audit_date: '2024-01-28',
      gst_no: '33MNOPQ9012R3S7',
      pan_number: 'MNOPQ9012R',
      email: 'sales@railcomponents.com',
      phone: '+91-9876543212'
    }
  ];

  // Form data
  let newVendor = {
    name: '',
    gst_no: '',
    pan_number: '',
    email: '',
    phone: '',
    city: '',
    state: ''
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
    if (!showAddForm) {
      resetForm();
    }
  }

  function resetForm() {
    newVendor = {
      name: '',
      gst_no: '',
      pan_number: '',
      email: '',
      phone: '',
      city: '',
      state: ''
    };
  }

  function addVendor() {
    if (newVendor.name && newVendor.gst_no && newVendor.pan_number) {
      const vendor = {
        vendor_id: `V${String(vendors.length + 1).padStart(3, '0')}`,
        name: newVendor.name,
        city: newVendor.city,
        state: newVendor.state,
        audit_date: new Date().toISOString().split('T')[0],
        gst_no: newVendor.gst_no,
        pan_number: newVendor.pan_number,
        email: newVendor.email,
        phone: newVendor.phone
      };
      vendors = [...vendors, vendor];
      resetForm();
      showAddForm = false;
    }
  }

  function deleteVendor(vendorId) {
    vendors = vendors.filter(v => v.vendor_id !== vendorId);
  }
</script>

<Layout {userRole} {username} {activePage}>
  <div class="vendors-page">
    <div class="page-header">
      <h1>Vendor Management</h1>
      <button class="add-btn" on:click={toggleAddForm}>
        {showAddForm ? 'Cancel' : 'Add Vendor'}
      </button>
    </div>

    <!-- Add Vendor Form -->
    {#if showAddForm}
      <div class="add-form">
        <h2>Add New Vendor</h2>
        <div class="form-grid">
          <div class="form-group">
            <label>Vendor Name</label>
            <input type="text" bind:value={newVendor.name} placeholder="Enter vendor name" />
          </div>
          <div class="form-group">
            <label>GST Number</label>
            <input type="text" bind:value={newVendor.gst_no} placeholder="Enter GST number" />
          </div>
          <div class="form-group">
            <label>PAN Number</label>
            <input type="text" bind:value={newVendor.pan_number} placeholder="Enter PAN number" />
          </div>
          <div class="form-group">
            <label>Email</label>
            <input type="email" bind:value={newVendor.email} placeholder="Enter email" />
          </div>
          <div class="form-group">
            <label>Phone</label>
            <input type="tel" bind:value={newVendor.phone} placeholder="Enter phone number" />
          </div>
          <div class="form-group">
            <label>City</label>
            <input type="text" bind:value={newVendor.city} placeholder="Enter city" />
          </div>
          <div class="form-group">
            <label>State</label>
            <input type="text" bind:value={newVendor.state} placeholder="Enter state" />
          </div>
        </div>
        <div class="form-actions">
          <button class="save-btn" on:click={addVendor}>Save Vendor</button>
          <button class="cancel-btn" on:click={toggleAddForm}>Cancel</button>
        </div>
      </div>
    {/if}

    <!-- Vendors Table -->
    <div class="table-container">
      <div class="table-header">
        <h2>Vendor List</h2>
        <span class="count">{vendors.length} vendors</span>
      </div>
      
      <div class="table-wrapper">
        <table class="vendors-table">
          <thead>
            <tr>
              <th>Vendor ID</th>
              <th>Name</th>
              <th>City</th>
              <th>State</th>
              <th>Audit Date</th>
              <th>GST No</th>
              <th>PAN No</th>
              <th>Email</th>
              <th>Phone</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each vendors as vendor}
              <tr>
                <td>{vendor.vendor_id}</td>
                <td>{vendor.name}</td>
                <td>{vendor.city}</td>
                <td>{vendor.state}</td>
                <td>{vendor.audit_date}</td>
                <td>{vendor.gst_no}</td>
                <td>{vendor.pan_number}</td>
                <td>{vendor.email}</td>
                <td>{vendor.phone}</td>
                <td class="actions">
                  <button class="edit-btn">Edit</button>
                  <button class="delete-btn" on:click={() => deleteVendor(vendor.vendor_id)}>Delete</button>
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
  .vendors-page {
    padding: 0;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding: 1.5rem 2rem;
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
    border: 1px solid #e2e8f0;
  }

  .page-header h1 {
    color: #1e293b;
    margin: 0;
    font-size: 1.75rem;
    font-weight: 700;
    background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .add-btn {
    background: linear-gradient(135deg, #3b82f6 0%, #1e40af 100%);
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
    transition: all 0.3s ease;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .add-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(59, 130, 246, 0.4);
  }

  .add-form {
    background: white;
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    margin-bottom: 2rem;
    border: 1px solid #e2e8f0;
  }

  .add-form h2 {
    color: #1e293b;
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

  .form-group input {
    padding: 0.75rem;
    border: 2px solid #e5e7eb;
    border-radius: 6px;
    font-size: 0.9rem;
    transition: border-color 0.3s;
  }

  .form-group input:focus {
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
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    overflow: hidden;
    border: 1px solid #e2e8f0;
    overflow-x: auto;
  }

  .table-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #e5e7eb;
  }

  .table-header h2 {
    color: #1e293b;
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

  .vendors-table {
    width: 100%;
    border-collapse: collapse;
    min-width: 600px;
  }

  .vendors-table th {
    background: #f8fafc;
    padding: 0.75rem 0.5rem;
    text-align: left;
    font-weight: 600;
    color: #374151;
    border-bottom: 1px solid #e5e7eb;
    font-size: 0.875rem;
    white-space: nowrap;
  }

  .vendors-table td {
    padding: 0.75rem 0.5rem;
    border-bottom: 1px solid #f1f5f9;
    color: #64748b;
    font-size: 0.875rem;
    white-space: nowrap;
  }

  /* Make specific columns more compact */
  .vendors-table th:nth-child(1),
  .vendors-table td:nth-child(1) {
    width: 80px;
    min-width: 80px;
  }

  .vendors-table th:nth-child(2),
  .vendors-table td:nth-child(2) {
    width: 150px;
    min-width: 150px;
  }

  .vendors-table th:nth-child(3),
  .vendors-table td:nth-child(3) {
    width: 100px;
    min-width: 100px;
  }

  .vendors-table th:nth-child(4),
  .vendors-table td:nth-child(4) {
    width: 120px;
    min-width: 120px;
  }

  .vendors-table th:nth-child(5),
  .vendors-table td:nth-child(5) {
    width: 100px;
    min-width: 100px;
  }

  .vendors-table tr:hover {
    background: #f8fafc;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  @media (max-width: 768px) {
    .vendors-page {
      padding: 1rem;
    }
    
    .page-header {
      flex-direction: column;
      gap: 1rem;
      align-items: stretch;
      padding: 1rem;
    }
    
    .add-form {
      padding: 1rem;
    }
    
    .form-grid {
      grid-template-columns: 1fr;
    }
    
    .table-header {
      flex-direction: column;
      gap: 1rem;
      align-items: stretch;
    }
    
    .actions {
      flex-direction: column;
    }
    
    /* Hide less important columns on tablet */
    .vendors-table th:nth-child(6),
    .vendors-table th:nth-child(7),
    .vendors-table th:nth-child(8),
    .vendors-table th:nth-child(9),
    .vendors-table td:nth-child(6),
    .vendors-table td:nth-child(7),
    .vendors-table td:nth-child(8),
    .vendors-table td:nth-child(9) {
      display: none;
    }
  }

  @media (max-width: 480px) {
    .vendors-page {
      padding: 0.5rem;
    }
    
    .page-header {
      padding: 0.75rem;
    }
    
    .add-form {
      padding: 0.75rem;
    }
    
    .vendors-table th,
    .vendors-table td {
      padding: 0.5rem 0.25rem;
      font-size: 0.875rem;
    }
    
    /* Hide even more columns on mobile - show only essential ones */
    .vendors-table th:nth-child(3),
    .vendors-table th:nth-child(4),
    .vendors-table th:nth-child(5),
    .vendors-table th:nth-child(6),
    .vendors-table th:nth-child(7),
    .vendors-table th:nth-child(8),
    .vendors-table th:nth-child(9),
    .vendors-table td:nth-child(3),
    .vendors-table td:nth-child(4),
    .vendors-table td:nth-child(5),
    .vendors-table td:nth-child(6),
    .vendors-table td:nth-child(7),
    .vendors-table td:nth-child(8),
    .vendors-table td:nth-child(9) {
      display: none;
    }
  }

  .edit-btn, .delete-btn {
    padding: 0.5rem 0.75rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .edit-btn {
    background: #f59e0b;
    color: white;
  }

  .delete-btn {
    background: #ef4444;
    color: white;
  }

  .edit-btn:hover {
    background: #d97706;
  }

  .delete-btn:hover {
    background: #dc2626;
  }
</style>
