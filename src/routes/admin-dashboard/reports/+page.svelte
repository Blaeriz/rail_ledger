<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fetchData, createItem, updateItem, deleteItem } from '$lib/utils';
  import { formatDate, getStatusColor } from '$lib/types';
  import DataTable from '$lib/components/DataTable.svelte';
  import Modal from '$lib/components/Modal.svelte';

  let user = null;
  let batchData = /** @type {any[]} */ ([]);
  let reportData = /** @type {any[]} */ ([]);
  let isLoading = true;
  let showReportModal = false;
  let editingItem = /** @type {any} */ (null);

  const reportForm = {
    reportId: '',
    batchId: '',
    inspectorName: '',
    status: 1,
    createdAt: ''
  };

  const tableColumns = [
    { key: 'reportId', label: 'REPORT ID' },
    { key: 'batchId', label: 'BATCH ID' },
    { key: 'inspectorName', label: 'INSPECTOR' },
    { 
      key: 'status', 
      label: 'STATUS',
      render: (/** @type {any} */ item) => {
        const status = item.status === 1 ? 'PASS' : 'FAIL';
        const color = item.status === 1 ? 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10' : 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10';
        return `<span class="px-3 py-1 rounded-full text-xs font-semibold border-2 ${color}">${status}</span>`;
      }
    },
    { key: 'createdAt', label: 'CREATED', render: (/** @type {any} */ item) => formatDate(item.createdAt) },
    { 
      key: 'actions', 
      label: 'ACTIONS',
      render: (/** @type {any} */ item) => `
        <div class="flex space-x-2">
          <button onclick="editReport('${item.reportId}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20">Edit</button>
          <button onclick="deleteReport('${item.reportId}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-red-500/30 hover:border-red-400/50 hover:shadow-lg hover:shadow-red-500/20">Delete</button>
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
      const [batches, reports] = await Promise.all([
        fetchData('/api/batches'),
        fetchData('/api/reports')
      ]);
      
      batchData = batches;
      reportData = reports;
    } catch (error) {
      console.error('Error loading data:', error);
    } finally {
      isLoading = false;
    }
  });

  function openReportModal(report = null) {
    editingItem = report;
    if (report) {
      Object.assign(reportForm, report);
    } else {
      Object.assign(reportForm, {
        reportId: '',
        batchId: '',
        inspectorName: '',
        status: 1,
        createdAt: new Date().toISOString().split('T')[0]
      });
    }
    showReportModal = true;
  }

  function closeModals() {
    showReportModal = false;
    editingItem = null;
  }

  async function saveReport() {
    try {
      const success = editingItem 
        ? await updateItem('/api/reports', reportForm.reportId, reportForm)
        : await createItem('/api/reports', reportForm);
      
      if (success) {
        reportData = await fetchData('/api/reports');
        closeModals();
      }
    } catch (error) {
      console.error('Error saving report:', error);
    }
  }

  async function deleteReport(/** @type {string} */ reportId) {
    if (confirm('Are you sure you want to delete this report?')) {
      try {
        const success = await deleteItem('/api/reports', reportId);
        if (success) {
          reportData = await fetchData('/api/reports');
        }
      } catch (error) {
        console.error('Error deleting report:', error);
      }
    }
  }

  // Make functions globally available for inline onclick handlers
  window.editReport = (reportId) => {
    const report = reportData.find(r => r.reportId === reportId);
    openReportModal(report);
  };
  window.deleteReport = deleteReport;
</script>

<div class="p-4 sm:p-6 lg:p-8">
  <!-- Page Header -->
  <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 mb-6">
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white">Inspection Reports</h1>
    <button 
      on:click={() => openReportModal()}
      class="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 sm:px-6 sm:py-3 rounded-lg font-medium transition-all duration-300 text-sm sm:text-base"
    >
      + Add Report
    </button>
  </div>

  <!-- Reports Table -->
  <DataTable 
    data={reportData} 
    columns={tableColumns}
    searchable={true}
    searchPlaceholder="Search reports..."
  />
</div>

<!-- Report Modal -->
<Modal isOpen={showReportModal} title={editingItem ? 'Edit Report' : 'Add New Report'} onClose={closeModals}>
  <form on:submit|preventDefault={saveReport} class="space-y-4">
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      <div>
        <label for="reportId" class="block text-gray-300 text-sm font-medium mb-2">Report ID</label>
        <input 
          id="reportId"
          type="text" 
          bind:value={reportForm.reportId}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter report ID"
        />
      </div>
      <div>
        <label for="batchId" class="block text-gray-300 text-sm font-medium mb-2">Batch ID</label>
        <select 
          id="batchId"
          bind:value={reportForm.batchId}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
        >
          <option value="">Select batch</option>
          {#each batchData as batch}
            <option value={batch.batch_id}>{batch.batch_id} - {batch.vendor_id}</option>
          {/each}
        </select>
      </div>
      <div>
        <label for="inspectorName" class="block text-gray-300 text-sm font-medium mb-2">Inspector Name</label>
        <input 
          id="inspectorName"
          type="text" 
          bind:value={reportForm.inspectorName}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
          placeholder="Enter inspector name"
        />
      </div>
      <div>
        <label for="status" class="block text-gray-300 text-sm font-medium mb-2">Status</label>
        <select 
          id="status"
          bind:value={reportForm.status}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
        >
          <option value={1}>Pass</option>
          <option value={0}>Fail</option>
        </select>
      </div>
      <div>
        <label for="createdAt" class="block text-gray-300 text-sm font-medium mb-2">Created Date</label>
        <input 
          id="createdAt"
          type="date" 
          bind:value={reportForm.createdAt}
          required
          class="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white focus:outline-none focus:border-purple-500 focus:ring-1 focus:ring-purple-500 text-sm"
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
        {editingItem ? 'Update Report' : 'Add Report'}
      </button>
    </div>
  </form>
</Modal>