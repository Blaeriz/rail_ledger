<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';

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

    // Check if we should auto-open the form
    const urlParams = new URLSearchParams($page.url.search);
    if (urlParams.get('action') === 'add') {
      showAddForm = true;
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
            <label for="vendor-name">Vendor Name</label>
            <input id="vendor-name" type="text" bind:value={newVendor.name} placeholder="Enter vendor name" />
          </div>
          <div class="form-group">
            <label for="gst-number">GST Number</label>
            <input id="gst-number" type="text" bind:value={newVendor.gst_no} placeholder="Enter GST number" />
          </div>
          <div class="form-group">
            <label for="pan-number">PAN Number</label>
            <input id="pan-number" type="text" bind:value={newVendor.pan_number} placeholder="Enter PAN number" />
          </div>
          <div class="form-group">
            <label for="vendor-email">Email</label>
            <input id="vendor-email" type="email" bind:value={newVendor.email} placeholder="Enter email" />
          </div>
          <div class="form-group">
            <label for="vendor-phone">Phone</label>
            <input id="vendor-phone" type="tel" bind:value={newVendor.phone} placeholder="Enter phone number" />
          </div>
          <div class="form-group">
            <label for="vendor-city">City</label>
            <input id="vendor-city" type="text" bind:value={newVendor.city} placeholder="Enter city" />
          </div>
          <div class="form-group">
            <label for="vendor-state">State</label>
            <input id="vendor-state" type="text" bind:value={newVendor.state} placeholder="Enter state" />
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
      
      <!-- Desktop Table View -->
      <div class="desktop-table">
        <table class="vendors-table">
          <thead>
            <tr>
              <th class="col-id">ID</th>
              <th class="col-name">Vendor Name</th>
              <th class="col-location">Location</th>
              <th class="col-contact">Contact</th>
              <th class="col-docs">Documents</th>
              <th class="col-actions">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each vendors as vendor}
              <tr>
                <td class="col-id">
                  <span class="vendor-id">{vendor.vendor_id}</span>
                </td>
                <td class="col-name">
                  <div class="vendor-info">
                    <div class="vendor-name">{vendor.name}</div>
                    <div class="audit-date">Audit: {vendor.audit_date}</div>
                  </div>
                </td>
                <td class="col-location">
                  <div class="location-info">
                    <div class="city">{vendor.city}</div>
                    <div class="state">{vendor.state}</div>
                  </div>
                </td>
                <td class="col-contact">
                  <div class="contact-info">
                    <div class="email">{vendor.email}</div>
                    <div class="phone">{vendor.phone}</div>
                  </div>
                </td>
                <td class="col-docs">
                  <div class="documents">
                    <div class="gst">GST: {vendor.gst_no}</div>
                    <div class="pan">PAN: {vendor.pan_number}</div>
                  </div>
                </td>
                <td class="col-actions">
                  <div class="actions">
                    <button class="action-btn edit-btn" title="Edit Vendor" aria-label="Edit Vendor">
                      <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        <path d="M18.5 2.5C18.8978 2.10218 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10218 21.5 2.5C21.8978 2.89782 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10218 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                      </svg>
                    </button>
                    <button class="action-btn delete-btn" on:click={() => deleteVendor(vendor.vendor_id)} title="Delete Vendor" aria-label="Delete Vendor">
                      <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M3 6H5H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        <path d="M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                      </svg>
                    </button>
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <!-- Mobile Card View -->
      <div class="mobile-cards">
        {#each vendors as vendor}
          <div class="vendor-card">
            <div class="card-header">
              <div class="vendor-id">{vendor.vendor_id}</div>
              <div class="actions">
                <button class="action-btn edit-btn" title="Edit Vendor" aria-label="Edit Vendor">
                  <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M18.5 2.5C18.8978 2.10218 19.4374 1.87868 20 1.87868C20.5626 1.87868 21.1022 2.10218 21.5 2.5C21.8978 2.89782 22.1213 3.43739 22.1213 4C22.1213 4.56261 21.8978 5.10218 21.5 5.5L12 15L8 16L9 12L18.5 2.5Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </button>
                <button class="action-btn delete-btn" on:click={() => deleteVendor(vendor.vendor_id)} title="Delete Vendor" aria-label="Delete Vendor">
                  <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M3 6H5H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                    <path d="M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </button>
              </div>
            </div>
            <div class="card-body">
              <div class="vendor-name">{vendor.name}</div>
              <div class="card-details">
                <div class="detail-row">
                  <span class="label">Location:</span>
                  <span class="value">{vendor.city}, {vendor.state}</span>
                </div>
                <div class="detail-row">
                  <span class="label">Email:</span>
                  <span class="value">{vendor.email}</span>
                </div>
                <div class="detail-row">
                  <span class="label">Phone:</span>
                  <span class="value">{vendor.phone}</span>
                </div>
                <div class="detail-row">
                  <span class="label">GST:</span>
                  <span class="value">{vendor.gst_no}</span>
                </div>
                <div class="detail-row">
                  <span class="label">PAN:</span>
                  <span class="value">{vendor.pan_number}</span>
                </div>
                <div class="detail-row">
                  <span class="label">Last Audit:</span>
                  <span class="value">{vendor.audit_date}</span>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
</Layout>

<style>
  .vendors-page {
    padding: 0;
    background: #f8fafc;
    min-height: 100vh;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding: 2rem;
    background: white;
    border-radius: 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    border: 1px solid #e2e8f0;
  }

  .page-header h1 {
    color: #000000;
    margin: 0;
    font-size: 2rem;
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
    padding: 0.875rem 1.75rem;
    border-radius: 12px;
    cursor: pointer;
    font-weight: 600;
    font-size: 0.95rem;
    transition: all 0.3s ease;
    box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .add-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(59, 130, 246, 0.4);
  }

  .add-form {
    background: white;
    padding: 2.5rem;
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
    margin-bottom: 2rem;
    border: 1px solid #e2e8f0;
  }

  .add-form h2 {
    color: #000000;
    margin: 0 0 2rem 0;
    font-size: 1.75rem;
    font-weight: 600;
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group label {
    margin-bottom: 0.75rem;
    color: #374151;
    font-weight: 600;
    font-size: 0.95rem;
  }

  .form-group input {
    padding: 1rem;
    border: 2px solid #e5e7eb;
    border-radius: 10px;
    font-size: 1rem;
    transition: all 0.3s ease;
    background: #fafbfc;
  }

  .form-group input:focus {
    outline: none;
    border-color: #3b82f6;
    background: white;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .form-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
  }

  .save-btn {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
    border: none;
    padding: 1rem 2rem;
    border-radius: 10px;
    cursor: pointer;
    font-weight: 600;
    font-size: 1rem;
    transition: all 0.3s ease;
    box-shadow: 0 4px 16px rgba(16, 185, 129, 0.3);
  }

  .save-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(16, 185, 129, 0.4);
  }

  .cancel-btn {
    background: #6b7280;
    color: white;
    border: none;
    padding: 1rem 2rem;
    border-radius: 10px;
    cursor: pointer;
    font-weight: 600;
    font-size: 1rem;
    transition: all 0.3s ease;
  }

  .cancel-btn:hover {
    background: #4b5563;
    transform: translateY(-2px);
  }

  .table-container {
    background: white;
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
    overflow: hidden;
    border: 1px solid #e2e8f0;
  }

  .table-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 2rem;
    border-bottom: 1px solid #e5e7eb;
    background: #fafbfc;
  }

  .table-header h2 {
    color: #000000;
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
  }

  .count {
    color: #64748b;
    font-size: 1rem;
    font-weight: 500;
    background: #e2e8f0;
    padding: 0.5rem 1rem;
    border-radius: 20px;
  }

  /* Desktop Table Styles */
  .desktop-table {
    display: block;
  }

  .mobile-cards {
    display: none;
  }

  .vendors-table {
    width: 100%;
    border-collapse: collapse;
  }

  .vendors-table th {
    background: #f8fafc;
    padding: 1.25rem 1rem;
    text-align: left;
    font-weight: 600;
    color: #374151;
    border-bottom: 2px solid #e5e7eb;
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .vendors-table td {
    padding: 1.25rem 1rem;
    border-bottom: 1px solid #f1f5f9;
    color: #64748b;
    font-size: 0.95rem;
    vertical-align: top;
  }

  .vendors-table tr:hover {
    background: #f8fafc;
  }

  /* Column specific styles */
  .col-id {
    width: 80px;
    text-align: center;
  }

  .col-name {
    width: 200px;
  }

  .col-location {
    width: 150px;
  }

  .col-contact {
    width: 180px;
  }

  .col-docs {
    width: 200px;
  }

  .col-actions {
    width: 120px;
    text-align: center;
  }

  .vendor-id {
    background: linear-gradient(135deg, #3b82f6 0%, #1e40af 100%);
    color: white;
    padding: 0.5rem 0.75rem;
    border-radius: 8px;
    font-weight: 600;
    font-size: 0.85rem;
    display: inline-block;
  }

  .vendor-name {
    font-weight: 600;
    color: #000000;
    margin-bottom: 0.25rem;
  }

  .audit-date {
    font-size: 0.8rem;
    color: #64748b;
  }

  .city, .state {
    font-size: 0.9rem;
  }

  .city {
    font-weight: 500;
    color: #374151;
  }

  .state {
    color: #64748b;
  }

  .email, .phone {
    font-size: 0.9rem;
  }

  .email {
    color: #3b82f6;
    font-weight: 500;
  }

  .phone {
    color: #64748b;
  }

  .gst, .pan {
    font-size: 0.8rem;
    font-family: 'Courier New', monospace;
    background: #f1f5f9;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    margin-bottom: 0.25rem;
  }

  .gst {
    color: #059669;
  }

  .pan {
    color: #dc2626;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
  }

  .action-btn {
    width: 36px;
    height: 36px;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
  }

  .action-btn svg {
    width: 16px;
    height: 16px;
  }

  .edit-btn {
    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
    color: white;
  }

  .edit-btn:hover {
    transform: scale(1.1);
    box-shadow: 0 4px 12px rgba(245, 158, 11, 0.4);
  }

  .delete-btn {
    background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
    color: white;
  }

  .delete-btn:hover {
    transform: scale(1.1);
    box-shadow: 0 4px 12px rgba(239, 68, 68, 0.4);
  }

  /* Mobile Card Styles */
  @media (max-width: 1024px) {
    .desktop-table {
      display: none;
    }

    .mobile-cards {
      display: block;
      padding: 1.5rem;
    }

    .vendor-card {
      background: white;
      border: 1px solid #e2e8f0;
      border-radius: 12px;
      margin-bottom: 1rem;
      overflow: hidden;
      transition: all 0.3s ease;
    }

    .vendor-card:hover {
      box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
      transform: translateY(-2px);
    }

    .card-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 1.25rem;
      background: #f8fafc;
      border-bottom: 1px solid #e5e7eb;
    }

    .card-body {
      padding: 1.25rem;
    }

    .vendor-name {
      font-size: 1.1rem;
      font-weight: 600;
      color: #000000;
      margin-bottom: 1rem;
    }

    .card-details {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 0.75rem;
    }

    .detail-row {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
    }

    .label {
      font-size: 0.8rem;
      font-weight: 600;
      color: #64748b;
      text-transform: uppercase;
      letter-spacing: 0.05em;
    }

    .value {
      font-size: 0.9rem;
      color: #374151;
      word-break: break-all;
    }

    .actions {
      gap: 0.5rem;
    }
  }

  @media (max-width: 768px) {
    .vendors-page {
      padding: 1rem;
    }
    
    .page-header {
      flex-direction: column;
      gap: 1.5rem;
      align-items: stretch;
      padding: 1.5rem;
    }
    
    .add-form {
      padding: 1.5rem;
    }
    
    .form-grid {
      grid-template-columns: 1fr;
    }

    .card-details {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 480px) {
    .vendors-page {
      padding: 0.5rem;
    }
    
    .page-header {
      padding: 1rem;
    }
    
    .add-form {
      padding: 1rem;
    }

    .mobile-cards {
      padding: 1rem;
    }

    .card-header {
      padding: 1rem;
    }

    .card-body {
      padding: 1rem;
    }
  }
</style>
