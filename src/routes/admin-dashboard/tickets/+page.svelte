<!-- @ts-nocheck -->
<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fetchData, createItem, updateItem, deleteItem } from '$lib/utils';
  import { getPriorityColor, getTicketStatusColor, getCategoryColor } from '$lib/types';
  import DataTable from '$lib/components/DataTable.svelte';
  import Modal from '$lib/components/Modal.svelte';

  let user = /** @type {any} */ (null);
  let ticketsData = /** @type {any[]} */ ([]);
  let isLoading = true;
  let showTicketModal = false;
  let editingTicket = /** @type {any} */ (null);

  const ticketForm = {
    title: '',
    description: '',
    priority: 'Medium',
    status: 'Open',
    category: 'General',
    assigned_to: ''
  };

  const tableColumns = [
    { key: 'ticket_id', label: 'TICKET ID' },
    { key: 'title', label: 'TITLE' },
    { 
      key: 'priority', 
      label: 'PRIORITY',
      render: (/** @type {any} */ item) => {
        const color = getPriorityColor(item.priority);
        return `<span class="px-3 py-1 rounded-full text-xs font-semibold border-2 ${color}">${item.priority}</span>`;
      }
    },
    { 
      key: 'status', 
      label: 'STATUS',
      render: (/** @type {any} */ item) => {
        const color = getTicketStatusColor(item.status);
        return `<span class="px-3 py-1 rounded-full text-xs font-semibold border-2 ${color}">${item.status}</span>`;
      }
    },
    { 
      key: 'category', 
      label: 'CATEGORY',
      render: (/** @type {any} */ item) => {
        const color = getCategoryColor(item.category);
        return `<span class="px-3 py-1 rounded-full text-xs font-semibold border-2 ${color}">${item.category}</span>`;
      }
    },
    { key: 'created_by', label: 'CREATED BY' },
    { key: 'assigned_to', label: 'ASSIGNED TO' },
    { key: 'created_at', label: 'CREATED' },
    { 
      key: 'actions', 
      label: 'ACTIONS',
      render: (/** @type {any} */ item) => `
        <div class="flex space-x-2">
          <button onclick="editTicket('${item.ticket_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-purple-500/30 hover:border-purple-400/50 hover:shadow-lg hover:shadow-purple-500/20">Edit</button>
          <button onclick="deleteTicket('${item.ticket_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-red-500/30 hover:border-red-400/50 hover:shadow-lg hover:shadow-red-500/20">Delete</button>
        </div>
      `
    }
  ];

  onMount(async () => {
    const userData = sessionStorage.getItem('user');
    if (!userData) return goto('/login');
    
    user = JSON.parse(userData);
    
    try {
      ticketsData = await fetchData('/api/tickets');
    } catch (error) {
      console.error('Error loading tickets:', error);
    }
    
    isLoading = false;

    // Global functions for table actions
    window.editTicket = (/** @type {string} */ ticketId) => {
      const ticket = ticketsData.find(t => t.ticket_id === ticketId);
      if (ticket) {
        editingTicket = { ...ticket };
        ticketForm.title = ticket.title;
        ticketForm.description = ticket.description;
        ticketForm.priority = ticket.priority;
        ticketForm.status = ticket.status;
        ticketForm.category = ticket.category;
        ticketForm.assigned_to = ticket.assigned_to || '';
        showTicketModal = true;
      }
    };

    window.deleteTicket = async (/** @type {string} */ ticketId) => {
      if (confirm('Are you sure you want to delete this ticket?')) {
        try {
          await deleteItem('/api/tickets', ticketId);
          ticketsData = ticketsData.filter(t => t.ticket_id !== ticketId);
        } catch (error) {
          console.error('Error deleting ticket:', error);
        }
      }
    };
  });

  function openTicketModal() {
    editingTicket = null;
    ticketForm.title = '';
    ticketForm.description = '';
    ticketForm.priority = 'Medium';
    ticketForm.status = 'Open';
    ticketForm.category = 'General';
    ticketForm.assigned_to = '';
    showTicketModal = true;
  }

  function closeTicketModal() {
    showTicketModal = false;
    editingTicket = null;
  }

  async function handleSubmit() {
    try {
      if (editingTicket) {
        await updateItem('/api/tickets', editingTicket.ticket_id, ticketForm);
        const index = ticketsData.findIndex(t => t.ticket_id === editingTicket.ticket_id);
        if (index !== -1) {
          ticketsData[index] = { ...ticketsData[index], ...ticketForm };
        }
      } else {
        const newTicket = await createItem('/api/tickets', {
          ...ticketForm,
          created_by: user.username
        });
        ticketsData = [newTicket, ...ticketsData];
      }
      closeTicketModal();
    } catch (error) {
      console.error('Error saving ticket:', error);
    }
  }
</script>

<div class="p-4 sm:p-6 lg:p-8">
  <!-- Page Header -->
  <div class="mb-6 sm:mb-8">
    <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-white mb-2">Ticket Management</h1>
    <p class="text-gray-400">Manage and track all system tickets and issues</p>
  </div>

  <!-- Action Buttons -->
  <div class="mb-6 sm:mb-8">
    <button 
      on:click={openTicketModal}
      class="bg-gray-800 hover:bg-gray-700 text-white px-6 py-3 rounded-lg font-medium transition-all duration-500 text-sm flex items-center justify-center space-x-2 border border-purple-500/30 hover:border-purple-400/50 hover:shadow-2xl hover:shadow-purple-500/30 active:bg-gray-600 active:border-purple-400/60 active:shadow-xl active:shadow-purple-500/40 group animate-slide-in-left interactive relative overflow-hidden"
    >
      <div class="absolute inset-0 bg-gradient-to-r from-purple-500/10 to-blue-500/10 opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      <div class="relative z-10 flex items-center space-x-2">
        <svg class="w-4 h-4 group-hover:scale-110 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
        </svg>
        <span class="group-hover:translate-x-1 transition-transform duration-300">Create Ticket</span>
      </div>
    </button>
  </div>

  <!-- Tickets Table -->
  <div class="mb-6">
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-4">
      <span class="text-gray-400 text-sm">{ticketsData.length} tickets</span>
    </div>
    
    <DataTable 
      data={ticketsData} 
      columns={tableColumns}
      searchable={true}
      searchPlaceholder="Search tickets..."
      showPagination={true}
      pageSize={10}
    />
  </div>
</div>

<!-- Ticket Modal -->
<Modal isOpen={showTicketModal} onClose={closeTicketModal}>
  <div class="bg-gray-800 p-6 rounded-lg max-w-2xl w-full mx-4">
    <h2 class="text-xl font-bold text-white mb-6">
      {editingTicket ? 'Edit Ticket' : 'Create New Ticket'}
    </h2>
    
    <form on:submit|preventDefault={handleSubmit} class="space-y-4">
      <div>
        <label for="ticket-title" class="block text-sm font-medium text-gray-300 mb-2">Title</label>
        <input 
          id="ticket-title"
          type="text" 
          bind:value={ticketForm.title}
          required
          class="w-full bg-gray-700 text-white px-3 py-2 rounded-lg border border-gray-600 focus:border-purple-500 focus:outline-none"
          placeholder="Enter ticket title"
        />
      </div>
      
      <div>
        <label for="ticket-description" class="block text-sm font-medium text-gray-300 mb-2">Description</label>
        <textarea 
          id="ticket-description"
          bind:value={ticketForm.description}
          required
          rows="4"
          class="w-full bg-gray-700 text-white px-3 py-2 rounded-lg border border-gray-600 focus:border-purple-500 focus:outline-none"
          placeholder="Describe the issue or request"
        ></textarea>
      </div>
      
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div>
          <label for="ticket-priority" class="block text-sm font-medium text-gray-300 mb-2">Priority</label>
          <select 
            id="ticket-priority"
            bind:value={ticketForm.priority}
            class="w-full bg-gray-700 text-white px-3 py-2 rounded-lg border border-gray-600 focus:border-purple-500 focus:outline-none"
          >
            <option value="Low">Low</option>
            <option value="Medium">Medium</option>
            <option value="High">High</option>
            <option value="Critical">Critical</option>
          </select>
        </div>
        
        <div>
          <label for="ticket-status" class="block text-sm font-medium text-gray-300 mb-2">Status</label>
          <select 
            id="ticket-status"
            bind:value={ticketForm.status}
            class="w-full bg-gray-700 text-white px-3 py-2 rounded-lg border border-gray-600 focus:border-purple-500 focus:outline-none"
          >
            <option value="Open">Open</option>
            <option value="In Progress">In Progress</option>
            <option value="Resolved">Resolved</option>
            <option value="Closed">Closed</option>
          </select>
        </div>
      </div>
      
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div>
          <label for="ticket-category" class="block text-sm font-medium text-gray-300 mb-2">Category</label>
          <select 
            id="ticket-category"
            bind:value={ticketForm.category}
            class="w-full bg-gray-700 text-white px-3 py-2 rounded-lg border border-gray-600 focus:border-purple-500 focus:outline-none"
          >
            <option value="Technical">Technical</option>
            <option value="Quality">Quality</option>
            <option value="Safety">Safety</option>
            <option value="General">General</option>
            <option value="Maintenance">Maintenance</option>
          </select>
        </div>
        
        <div>
          <label for="ticket-assigned" class="block text-sm font-medium text-gray-300 mb-2">Assign To</label>
          <input 
            id="ticket-assigned"
            type="text" 
            bind:value={ticketForm.assigned_to}
            class="w-full bg-gray-700 text-white px-3 py-2 rounded-lg border border-gray-600 focus:border-purple-500 focus:outline-none"
            placeholder="Username (optional)"
          />
        </div>
      </div>
      
      <div class="flex justify-end space-x-3 pt-4">
        <button 
          type="button" 
          on:click={closeTicketModal}
          class="px-4 py-2 bg-gray-600 hover:bg-gray-500 text-white rounded-lg transition-colors duration-200"
        >
          Cancel
        </button>
        <button 
          type="submit"
          class="px-4 py-2 bg-purple-600 hover:bg-purple-500 text-white rounded-lg transition-colors duration-200"
        >
          {editingTicket ? 'Update Ticket' : 'Create Ticket'}
        </button>
      </div>
    </form>
  </div>
</Modal>
