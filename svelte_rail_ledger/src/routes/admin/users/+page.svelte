<script>
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  let userRole = '';
  let username = '';
  let activePage = '/admin/users';
  let showAddForm = false;

  // Sample user data
  let users = [
    {
      user_id: 'u001',
      aadhar: '123456789012',
      name: 'Rajesh Kumar',
      phone: '+91-9876543210',
      role: 'Admin'
    },
    {
      user_id: 'u002',
      aadhar: '234567890123',
      name: 'Priya Sharma',
      phone: '+91-9876543211',
      role: 'Inspector'
    },
    {
      user_id: 'u003',
      aadhar: '345678901234',
      name: 'Amit Singh',
      phone: '+91-9876543212',
      role: 'Viewer'
    },
    {
      user_id: 'u004',
      aadhar: '456789012345',
      name: 'Sneha Patel',
      phone: '+91-9876543213',
      role: 'Inspector'
    }
  ];

  // Form data
  let newUser = {
    aadhar: '',
    name: '',
    phone: '',
    role: 'Viewer'
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
    newUser = {
      aadhar: '',
      name: '',
      phone: '',
      role: 'Viewer'
    };
  }

  function addUser() {
    if (newUser.aadhar && newUser.name && newUser.phone) {
      const user = {
        user_id: `u${String(users.length + 1).padStart(3, '0')}`,
        aadhar: newUser.aadhar,
        name: newUser.name,
        phone: newUser.phone,
        role: newUser.role
      };
      users = [...users, user];
      resetForm();
      showAddForm = false;
    }
  }

  function removeUser(userId) {
    users = users.filter(u => u.user_id !== userId);
  }
</script>

<Layout {userRole} {username} {activePage}>
  <div class="users-page">
    <div class="page-header">
      <h1>User Management</h1>
      <button class="add-btn" on:click={toggleAddForm}>
        {showAddForm ? 'Cancel' : 'Add User'}
      </button>
    </div>

    <!-- Add User Form -->
    {#if showAddForm}
      <div class="add-form">
        <h2>Add New User</h2>
        <div class="form-grid">
          <div class="form-group">
            <label>Aadhaar Number</label>
            <input type="text" bind:value={newUser.aadhar} placeholder="Enter 12-digit Aadhaar" maxlength="12" />
          </div>
          <div class="form-group">
            <label>Full Name</label>
            <input type="text" bind:value={newUser.name} placeholder="Enter full name" />
          </div>
          <div class="form-group">
            <label>Phone Number</label>
            <input type="tel" bind:value={newUser.phone} placeholder="Enter phone number" />
          </div>
          <div class="form-group">
            <label>Role</label>
            <select bind:value={newUser.role}>
              <option value="Admin">Admin</option>
              <option value="Inspector">Inspector</option>
              <option value="Viewer">Viewer</option>
            </select>
          </div>
        </div>
        <div class="form-actions">
          <button class="save-btn" on:click={addUser}>Save User</button>
          <button class="cancel-btn" on:click={toggleAddForm}>Cancel</button>
        </div>
      </div>
    {/if}

    <!-- Users Table -->
    <div class="table-container">
      <div class="table-header">
        <h2>User List</h2>
        <span class="count">{users.length} users</span>
      </div>
      
      <div class="table-wrapper">
        <table class="users-table">
          <thead>
            <tr>
              <th>User ID</th>
              <th>Aadhaar</th>
              <th>Name</th>
              <th>Phone</th>
              <th>Role</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each users as user}
              <tr>
                <td>{user.user_id}</td>
                <td>{user.aadhar}</td>
                <td>{user.name}</td>
                <td>{user.phone}</td>
                <td>
                  <span class="role role-{user.role.toLowerCase()}">
                    {user.role}
                  </span>
                </td>
                <td class="actions">
                  <button class="edit-btn">Edit</button>
                  <button class="remove-btn" on:click={() => removeUser(user.user_id)}>Remove</button>
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
  .users-page {
    padding: 2rem;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
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

  .users-table {
    width: 100%;
    border-collapse: collapse;
    min-width: 400px;
  }

  .table-container {
    overflow-x: auto;
  }

  .users-table th {
    background: #f8fafc;
    padding: 1rem;
    text-align: left;
    font-weight: 600;
    color: #374151;
    border-bottom: 1px solid #e5e7eb;
  }

  .users-table td {
    padding: 1rem;
    border-bottom: 1px solid #f1f5f9;
    color: #64748b;
  }

  .users-table tr:hover {
    background: #f8fafc;
  }

  .role {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
  }

  .role-admin {
    background: #dbeafe;
    color: #1e40af;
  }

  .role-inspector {
    background: #f3e8ff;
    color: #7c3aed;
  }

  .role-viewer {
    background: #dcfce7;
    color: #166534;
  }

  .actions {
    display: flex;
    gap: 0.5rem;
  }

  @media (max-width: 768px) {
    .users-page {
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
    .users-page {
      padding: 0.5rem;
    }
    
    .users-table th,
    .users-table td {
      padding: 0.5rem 0.25rem;
      font-size: 0.875rem;
    }
  }

  .edit-btn, .remove-btn {
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

  .remove-btn {
    background: #ef4444;
    color: white;
  }

  .edit-btn:hover {
    background: #d97706;
  }

  .remove-btn:hover {
    background: #dc2626;
  }
</style>
