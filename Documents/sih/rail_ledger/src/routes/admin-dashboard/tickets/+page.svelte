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
				return `<span class="${color}">${item.priority}</span>`;
			}
		},
		{
			key: 'status',
			label: 'STATUS',
			render: (/** @type {any} */ item) => {
				const color = getTicketStatusColor(item.status);
				return `<span class="${color}">${item.status}</span>`;
			}
		},
		{
			key: 'category',
			label: 'CATEGORY',
			render: (/** @type {any} */ item) => {
				const color = getCategoryColor(item.category);
				return `<span class="${color}">${item.category}</span>`;
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
          <button onclick="editTicket('${item.ticket_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1.5 rounded-lg border border-purple-500/40 hover:border-purple-400/60 hover:shadow-lg hover:shadow-purple-500/25 font-medium">Edit</button>
          <button onclick="deleteTicket('${item.ticket_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1.5 rounded-lg border border-red-500/40 hover:border-red-400/60 hover:shadow-lg hover:shadow-red-500/25 font-medium">Delete</button>
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
			const ticket = ticketsData.find((t) => t.ticket_id === ticketId);
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
					ticketsData = ticketsData.filter((t) => t.ticket_id !== ticketId);
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
				const index = ticketsData.findIndex((t) => t.ticket_id === editingTicket.ticket_id);
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
		<h1 class="mb-2 text-2xl font-bold text-white sm:text-3xl lg:text-4xl">Ticket Management</h1>
		<p class="text-gray-400">Manage and track all system tickets and issues</p>
	</div>

	<!-- Action Buttons -->
	<div class="mb-6 sm:mb-8">
		<button
			on:click={openTicketModal}
			class="group animate-slide-in-left interactive relative flex items-center justify-center space-x-2 overflow-hidden rounded-lg border border-purple-500/30 bg-gray-800 px-6 py-3 text-sm font-medium text-white transition-all duration-500 hover:border-purple-400/50 hover:bg-gray-700 hover:shadow-2xl hover:shadow-purple-500/30 active:border-purple-400/60 active:bg-gray-600 active:shadow-xl active:shadow-purple-500/40"
		>
			<div
				class="absolute inset-0 bg-gradient-to-r from-purple-500/10 to-blue-500/10 opacity-0 transition-opacity duration-500 group-hover:opacity-100"
			></div>
			<div class="relative z-10 flex items-center space-x-2">
				<svg
					class="h-4 w-4 transition-transform duration-300 group-hover:scale-110"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 6v6m0 0v6m0-6h6m-6 0H6"
					></path>
				</svg>
				<span class="transition-transform duration-300 group-hover:translate-x-1"
					>Create Ticket</span
				>
			</div>
		</button>
	</div>

	<!-- Tickets Table -->
	<div class="mb-6">
		<div class="mb-4 flex flex-col items-start justify-between gap-4 sm:flex-row sm:items-center">
			<span class="text-sm text-gray-400">{ticketsData.length} tickets</span>
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
	<div class="mx-4 w-full max-w-2xl rounded-xl bg-gray-800/95 backdrop-blur-sm border border-gray-700/50 shadow-2xl p-6">
		<h2 class="mb-6 text-xl font-bold text-white">
			{editingTicket ? 'Edit Ticket' : 'Create New Ticket'}
		</h2>

		<form on:submit|preventDefault={handleSubmit} class="space-y-4">
			<div>
				<label for="ticket-title" class="mb-2 block text-sm font-medium text-gray-300">Title</label>
				<input
					id="ticket-title"
					type="text"
					bind:value={ticketForm.title}
					required
					class="w-full rounded-lg border border-gray-600/50 bg-gray-700/50 backdrop-blur-sm px-3 py-2 text-white focus:border-purple-500/60 focus:outline-none focus:ring-2 focus:ring-purple-500/20 transition-all duration-200"
					placeholder="Enter ticket title"
				/>
			</div>

			<div>
				<label for="ticket-description" class="mb-2 block text-sm font-medium text-gray-300"
					>Description</label
				>
				<textarea
					id="ticket-description"
					bind:value={ticketForm.description}
					required
					rows="4"
					class="w-full rounded-lg border border-gray-600/50 bg-gray-700/50 backdrop-blur-sm px-3 py-2 text-white focus:border-purple-500/60 focus:outline-none focus:ring-2 focus:ring-purple-500/20 transition-all duration-200"
					placeholder="Describe the issue or request"
				></textarea>
			</div>

			<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
				<div>
					<label for="ticket-priority" class="mb-2 block text-sm font-medium text-gray-300"
						>Priority</label
					>
					<select
						id="ticket-priority"
						bind:value={ticketForm.priority}
						class="w-full rounded-lg border border-gray-600/50 bg-gray-700/50 backdrop-blur-sm px-3 py-2 text-white focus:border-purple-500/60 focus:outline-none focus:ring-2 focus:ring-purple-500/20 transition-all duration-200"
					>
						<option value="Low">Low</option>
						<option value="Medium">Medium</option>
						<option value="High">High</option>
						<option value="Critical">Critical</option>
					</select>
				</div>

				<div>
					<label for="ticket-status" class="mb-2 block text-sm font-medium text-gray-300"
						>Status</label
					>
					<select
						id="ticket-status"
						bind:value={ticketForm.status}
						class="w-full rounded-lg border border-gray-600/50 bg-gray-700/50 backdrop-blur-sm px-3 py-2 text-white focus:border-purple-500/60 focus:outline-none focus:ring-2 focus:ring-purple-500/20 transition-all duration-200"
					>
						<option value="Open">Open</option>
						<option value="In Progress">In Progress</option>
						<option value="Resolved">Resolved</option>
						<option value="Closed">Closed</option>
					</select>
				</div>
			</div>

			<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
				<div>
					<label for="ticket-category" class="mb-2 block text-sm font-medium text-gray-300"
						>Category</label
					>
					<select
						id="ticket-category"
						bind:value={ticketForm.category}
						class="w-full rounded-lg border border-gray-600/50 bg-gray-700/50 backdrop-blur-sm px-3 py-2 text-white focus:border-purple-500/60 focus:outline-none focus:ring-2 focus:ring-purple-500/20 transition-all duration-200"
					>
						<option value="Technical">Technical</option>
						<option value="Quality">Quality</option>
						<option value="Safety">Safety</option>
						<option value="General">General</option>
						<option value="Maintenance">Maintenance</option>
					</select>
				</div>

				<div>
					<label for="ticket-assigned" class="mb-2 block text-sm font-medium text-gray-300"
						>Assign To</label
					>
					<input
						id="ticket-assigned"
						type="text"
						bind:value={ticketForm.assigned_to}
						class="w-full rounded-lg border border-gray-600/50 bg-gray-700/50 backdrop-blur-sm px-3 py-2 text-white focus:border-purple-500/60 focus:outline-none focus:ring-2 focus:ring-purple-500/20 transition-all duration-200"
						placeholder="Username (optional)"
					/>
				</div>
			</div>

			<div class="flex justify-end space-x-3 pt-4">
				<button
					type="button"
					on:click={closeTicketModal}
					class="rounded-lg bg-gray-600/80 backdrop-blur-sm border border-gray-500/50 px-4 py-2 text-white transition-all duration-200 hover:bg-gray-500/80 hover:border-gray-400/60 hover:shadow-lg hover:shadow-gray-500/20"
				>
					Cancel
				</button>
				<button
					type="submit"
					class="rounded-lg bg-purple-600/80 backdrop-blur-sm border border-purple-500/50 px-4 py-2 text-white transition-all duration-200 hover:bg-purple-500/80 hover:border-purple-400/60 hover:shadow-lg hover:shadow-purple-500/25"
				>
					{editingTicket ? 'Update Ticket' : 'Create Ticket'}
				</button>
			</div>
		</form>
	</div>
</Modal>
