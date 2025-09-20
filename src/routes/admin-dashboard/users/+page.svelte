<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fetchData, createItem, updateItem, deleteItem } from '$lib/utils';
  import DataTable from '$lib/components/DataTable.svelte';
  import Modal from '$lib/components/Modal.svelte';

  let user = null;
  let usersData = /** @type {any[]} */ ([]);
  let isLoading = true;
  let showUserModal = false;
  let editingItem = /** @type {any} */ (null);

  const userForm = {
    user_id: '',
    name: '',
    aadhar: '',
    phone_number: '',
    user_role: 'viewer'
  };

  const tableColumns = [
    { key: 'user_id', label: 'USER ID' },
    { key: 'name', label: 'NAME' },
    { key: 'aadhar', label: 'AADHAR' },
    { key: 'phone_number', label: 'PHONE' },
    { 
      key: 'user_role', 
      label: 'ROLE',
      render: (/** @type {any} */ item) => {
        return `<span class="text-gray-300 text-sm">${(item.user_role || 'viewer').toLowerCase()}</span>`;
      }
    },
    { 
      key: 'actions', 
      label: 'ACTIONS',
      render: (/** @type {any} */ item) => `
        <div class="flex space-x-2">
          <button onclick="editUser('${item.user_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20">Edit</button>
          <button onclick="deleteUser('${item.user_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-red-500/30 hover:border-red-400/50 hover:shadow-lg hover:shadow-red-500/20">Delete</button>
        </div>
      `
    }
  ];

  onMount(async () => {
    const userData = sessionStorage.getItem('user');
    if (!userData) {
      goto('/login');
      return;
    }
    
    user = JSON.parse(userData);
    
    try {
      usersData = await fetchData('/api/users');
    } catch (error) {
      console.error('Error loading data:', error);
    } finally {
      isLoading = false;
    }
  });

  function openUserModal(user = null) {
    editingItem = user;
    if (user) {
      Object.assign(userForm, user);
    } else {
      Object.assign(userForm, {
        user_id: '',
        name: '',
        aadhar: '',
        phone_number: '',
        user_role: 'viewer'
      });
    }
    showUserModal = true;
  }

  function closeModals() {
    showUserModal = false;
    editingItem = null;
  }

  async function saveUser() {
    try {
      const success = editingItem 
        ? await updateItem('/api/users', userForm.user_id, userForm)
        : await createItem('/api/users', userForm);
      
      if (success) {
        usersData = await fetchData('/api/users');
        closeModals();
      }
    } catch (error) {
      console.error('Error saving user:', error);
    }
  }

  async function deleteUser(/** @type {string} */ userId) {
    if (confirm('Are you sure you want to delete this user?')) {
      try {
        const success = await deleteItem('/api/users', userId);
        if (success) {
          usersData = await fetchData('/api/users');
        }
      } catch (error) {
        console.error('Error deleting user:', error);
      }
    }
  }

  // Make functions globally available for inline onclick handlers
  window.editUser = (userId) => {
    const user = usersData.find(u => u.user_id === userId);
    openUserModal(user);
  };
  window.deleteUser = deleteUser;
</script>

<div class="p-4 sm:p-6 lg:p-8">
  <!-- Page Header -->
  <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 mb-6">
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">User Management</h1>
    <button 
      on:click={() => openUserModal()}
      class="bg-gray-800 hover:bg-gray-700 text-white px-6 py-3 rounded-lg font-medium transition-all duration-300 text-sm flex items-center justify-center space-x-2 border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20 active:bg-gray-600 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/30"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
      </svg>
      <span>Add User</span>
    </button>
  </div>

  <!-- Users Table -->
  <DataTable 
    data={usersData} 
    columns={tableColumns}
    searchable={true}
    searchPlaceholder="Search users..."
  />
</div>

<!-- User Modal -->
<Modal isOpen={showUserModal} title={editingItem ? 'Edit User' : 'Add New User'} onClose={closeModals}>
  <form on:submit|preventDefault={saveUser} class="space-y-4">
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      <div>
        <label for="user_id" class="block text-gray-300 text-sm font-medium mb-2">User ID</label>
        <input 
          id="user_id"
          type="text" 
          bind:value={userForm.user_id}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter user ID"
        />
      </div>
      <div>
        <label for="name" class="block text-gray-300 text-sm font-medium mb-2">Name</label>
        <input 
          id="name"
          type="text" 
          bind:value={userForm.name}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter name"
        />
      </div>
      <div>
        <label for="aadhar" class="block text-gray-300 text-sm font-medium mb-2">Aadhar Number</label>
        <input 
          id="aadhar"
          type="text" 
          bind:value={userForm.aadhar}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter Aadhar number"
        />
      </div>
      <div>
        <label for="phone_number" class="block text-gray-300 text-sm font-medium mb-2">Phone Number</label>
        <input 
          id="phone_number"
          type="text" 
          bind:value={userForm.phone_number}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter phone number"
        />
      </div>
      <div>
        <label for="user_role" class="block text-gray-300 text-sm font-medium mb-2">User Role</label>
        <select 
          id="user_role"
          bind:value={userForm.user_role}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
        >
          <option value="viewer">Viewer</option>
          <option value="inspector">Inspector</option>
          <option value="admin">Admin</option>
        </select>
      </div>
    </div>
    
    <div class="flex flex-col sm:flex-row justify-end space-y-2 sm:space-y-0 sm:space-x-4 pt-6">
      <button 
        type="button"
        on:click={closeModals}
        class="w-full sm:w-auto px-6 py-3 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-all duration-300 text-sm border border-gray-600 hover:border-gray-500 hover:shadow-lg hover:shadow-gray-500/10"
      >
        Cancel
      </button>
      <button 
        type="submit"
        class="w-full sm:w-auto px-6 py-3 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-all duration-300 text-sm border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20 active:bg-gray-600 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/30"
      >
        {editingItem ? 'Update User' : 'Add User'}
      </button>
    </div>
  </form>
</Modal>