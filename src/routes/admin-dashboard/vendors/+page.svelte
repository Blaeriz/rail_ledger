<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fetchData, createItem, updateItem, deleteItem } from '$lib/utils';
  import DataTable from '$lib/components/DataTable.svelte';
  import Modal from '$lib/components/Modal.svelte';

  let user = null;
  let vendorData = /** @type {any[]} */ ([]);
  let isLoading = true;
  let showVendorModal = false;
  let editingItem = /** @type {any} */ (null);

  const vendorForm = {
    vendor_id: '',
    city: '',
    state: '',
    no_of_batches: '',
    email: ''
  };

  const tableColumns = [
    { key: 'vendor_id', label: 'VENDOR ID' },
    { key: 'city', label: 'CITY' },
    { key: 'state', label: 'STATE' },
    { key: 'no_of_batches', label: 'BATCHES' },
    { key: 'email', label: 'EMAIL' },
    { 
      key: 'actions', 
      label: 'ACTIONS',
      render: (/** @type {any} */ item) => `
        <div class="flex space-x-2">
          <button onclick="editVendor('${item.vendor_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20">Edit</button>
          <button onclick="deleteVendor('${item.vendor_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-red-500/30 hover:border-red-400/50 hover:shadow-lg hover:shadow-red-500/20">Delete</button>
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
      vendorData = await fetchData('/api/vendors');
    } catch (error) {
      console.error('Error loading data:', error);
    } finally {
      isLoading = false;
    }
  });

  function openVendorModal(vendor = null) {
    editingItem = vendor;
    if (vendor) {
      Object.assign(vendorForm, vendor);
    } else {
      Object.assign(vendorForm, {
        vendor_id: '',
        city: '',
        state: '',
        no_of_batches: '',
        email: ''
      });
    }
    showVendorModal = true;
  }

  function closeModals() {
    showVendorModal = false;
    editingItem = null;
  }

  async function saveVendor() {
    try {
      const success = editingItem 
        ? await updateItem('/api/vendors', vendorForm.vendor_id, vendorForm)
        : await createItem('/api/vendors', vendorForm);
      
      if (success) {
        vendorData = await fetchData('/api/vendors');
        closeModals();
      }
    } catch (error) {
      console.error('Error saving vendor:', error);
    }
  }

  async function deleteVendor(/** @type {string} */ vendorId) {
    if (confirm('Are you sure you want to delete this vendor?')) {
      try {
        const success = await deleteItem('/api/vendors', vendorId);
        if (success) {
          vendorData = await fetchData('/api/vendors');
        }
      } catch (error) {
        console.error('Error deleting vendor:', error);
      }
    }
  }

  // Make functions globally available for inline onclick handlers
  window.editVendor = (vendorId) => {
    const vendor = vendorData.find(v => v.vendor_id === vendorId);
    openVendorModal(vendor);
  };
  window.deleteVendor = deleteVendor;
</script>

<div class="p-4 sm:p-6 lg:p-8">
  <!-- Page Header -->
  <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 mb-6">
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">Vendor Management</h1>
    <button 
      on:click={() => openVendorModal()}
      class="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 sm:px-6 sm:py-3 rounded-lg font-medium transition-all duration-300 text-sm sm:text-base"
    >
      + Add Vendor
    </button>
  </div>

  <!-- Vendor Table -->
  <DataTable 
    data={vendorData} 
    columns={tableColumns}
    searchable={true}
    searchPlaceholder="Search vendors..."
  />
</div>

<!-- Vendor Modal -->
<Modal isOpen={showVendorModal} title={editingItem ? 'Edit Vendor' : 'Add New Vendor'} onClose={closeModals}>
  <form on:submit|preventDefault={saveVendor} class="space-y-4">
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      <div>
        <label for="vendor_id" class="block text-gray-300 text-sm font-medium mb-2">Vendor ID</label>
        <input 
          id="vendor_id"
          type="text" 
          bind:value={vendorForm.vendor_id}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter vendor ID"
        />
      </div>
      <div>
        <label for="city" class="block text-gray-300 text-sm font-medium mb-2">City</label>
        <input 
          id="city"
          type="text" 
          bind:value={vendorForm.city}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter city"
        />
      </div>
      <div>
        <label for="state" class="block text-gray-300 text-sm font-medium mb-2">State</label>
        <input 
          id="state"
          type="text" 
          bind:value={vendorForm.state}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter state"
        />
      </div>
      <div>
        <label for="no_of_batches" class="block text-gray-300 text-sm font-medium mb-2">Number of Batches</label>
        <input 
          id="no_of_batches"
          type="number" 
          bind:value={vendorForm.no_of_batches}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter number of batches"
        />
      </div>
      <div class="sm:col-span-2">
        <label for="email" class="block text-gray-300 text-sm font-medium mb-2">Email</label>
        <input 
          id="email"
          type="email" 
          bind:value={vendorForm.email}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter email address"
        />
      </div>
    </div>
    
    <div class="flex flex-col sm:flex-row justify-end space-y-2 sm:space-y-0 sm:space-x-4 pt-6">
      <button 
        type="button" 
        on:click={closeModals}
        class="w-full sm:w-auto px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-all duration-300 text-sm"
      >
        Cancel
      </button>
      <button 
        type="submit"
        class="w-full sm:w-auto px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg transition-all duration-300 text-sm"
      >
        {editingItem ? 'Update Vendor' : 'Add Vendor'}
      </button>
    </div>
  </form>
</Modal>