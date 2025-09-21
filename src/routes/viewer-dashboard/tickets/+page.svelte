<!-- @ts-nocheck -->
<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { fetchData } from '$lib/utils';
	import { getPriorityColor, getTicketStatusColor, getCategoryColor } from '$lib/types';
	import DataTable from '$lib/components/DataTable.svelte';
	import Modal from '$lib/components/Modal.svelte';

	let user = /** @type {any} */ (null);
	let ticketsData = /** @type {any[]} */ ([]);
	let isLoading = true;
	let showTicketModal = false;
	let selectedTicket = /** @type {any} */ (null);

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
          <button onclick="viewTicket('${item.ticket_id}')" class="text-white bg-gray-800 hover:bg-gray-700 text-xs sm:text-sm transition-all duration-300 px-3 py-1 rounded border border-blue-500/30 hover:border-blue-400/50 hover:shadow-lg hover:shadow-blue-500/20">View</button>
        </div>
      `
		}
	];

	onMount(async () => {
		const userData = sessionStorage.getItem('user');
		if (!userData) return goto('/login');

		user = JSON.parse(userData);

		try {
			// Get all tickets for viewer (read-only access)
			ticketsData = await fetchData('/api/tickets');
		} catch (error) {
			console.error('Error loading tickets:', error);
		}

		isLoading = false;

		// Global function for viewing ticket details
		window.viewTicket = (/** @type {string} */ ticketId) => {
			const ticket = ticketsData.find((t) => t.ticket_id === ticketId);
			if (ticket) {
				selectedTicket = ticket;
				showTicketModal = true;
			}
		};
	});

	function closeTicketModal() {
		showTicketModal = false;
		selectedTicket = null;
	}
</script>

<div class="p-4 sm:p-6 lg:p-8">
	<!-- Page Header -->
	<div class="mb-6 sm:mb-8">
		<h1 class="mb-2 text-2xl font-bold text-white sm:text-3xl lg:text-4xl">Tickets</h1>
		<p class="text-gray-400">View all system tickets and their current status</p>
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

<!-- Ticket Details Modal -->
<Modal isOpen={showTicketModal} onClose={closeTicketModal}>
	<div class="mx-4 w-full max-w-4xl rounded-lg bg-gray-800 p-6">
		<h2 class="mb-6 text-xl font-bold text-white">Ticket Details</h2>

		{#if selectedTicket}
			<div class="space-y-6">
				<!-- Header Info -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
					<div>
						<h3 class="mb-2 text-lg font-semibold text-white">{selectedTicket.title}</h3>
						<p class="text-sm text-gray-400">Ticket ID: {selectedTicket.ticket_id}</p>
					</div>
					<div class="flex flex-wrap gap-2">
						<span
							class="rounded-full border-2 px-3 py-1 text-xs font-semibold {getPriorityColor(
								selectedTicket.priority
							)}"
						>
							{selectedTicket.priority}
						</span>
						<span
							class="rounded-full border-2 px-3 py-1 text-xs font-semibold {getTicketStatusColor(
								selectedTicket.status
							)}"
						>
							{selectedTicket.status}
						</span>
						<span
							class="rounded-full border-2 px-3 py-1 text-xs font-semibold {getCategoryColor(
								selectedTicket.category
							)}"
						>
							{selectedTicket.category}
						</span>
					</div>
				</div>

				<!-- Description -->
				<div>
					<h4 class="mb-2 text-sm font-medium text-gray-300">Description</h4>
					<p class="rounded-lg bg-gray-700 p-3 text-white">{selectedTicket.description}</p>
				</div>

				<!-- Details Grid -->
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
					<div>
						<h4 class="mb-2 text-sm font-medium text-gray-300">Created By</h4>
						<p class="text-white">{selectedTicket.created_by}</p>
					</div>
					<div>
						<h4 class="mb-2 text-sm font-medium text-gray-300">Assigned To</h4>
						<p class="text-white">{selectedTicket.assigned_to || 'Unassigned'}</p>
					</div>
					<div>
						<h4 class="mb-2 text-sm font-medium text-gray-300">Created</h4>
						<p class="text-white">{new Date(selectedTicket.created_at).toLocaleString()}</p>
					</div>
					<div>
						<h4 class="mb-2 text-sm font-medium text-gray-300">Last Updated</h4>
						<p class="text-white">{new Date(selectedTicket.updated_at).toLocaleString()}</p>
					</div>
				</div>

				<!-- Resolution Notes -->
				{#if selectedTicket.resolution_notes}
					<div>
						<h4 class="mb-2 text-sm font-medium text-gray-300">Resolution Notes</h4>
						<p class="rounded-lg bg-gray-700 p-3 text-white">{selectedTicket.resolution_notes}</p>
					</div>
				{/if}

				<!-- Attachments -->
				{#if selectedTicket.attachments && selectedTicket.attachments.length > 0}
					<div>
						<h4 class="mb-2 text-sm font-medium text-gray-300">Attachments</h4>
						<div class="space-y-2">
							{#each selectedTicket.attachments as attachment}
								<div class="flex items-center space-x-2 text-blue-400 hover:text-blue-300">
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"
										></path>
									</svg>
									<span class="text-sm">{attachment}</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{/if}

		<div class="flex justify-end pt-6">
			<button
				on:click={closeTicketModal}
				class="rounded-lg bg-gray-600 px-4 py-2 text-white transition-colors duration-200 hover:bg-gray-500"
			>
				Close
			</button>
		</div>
	</div>
</Modal>
