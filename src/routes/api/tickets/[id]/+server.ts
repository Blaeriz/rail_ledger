import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Mock ticket data (same as in +server.ts)
let tickets = [
	{
		ticket_id: 'TKT-001',
		title: 'Quality Control Issue - Batch #12345',
		description:
			'Found quality issues in batch 12345 during inspection. Multiple components showing defects.',
		priority: 'High',
		status: 'Open',
		category: 'Quality',
		created_by: 'inspector1',
		assigned_to: 'admin1',
		created_at: '2024-01-15T10:30:00Z',
		updated_at: '2024-01-15T10:30:00Z',
		resolution_notes: '',
		attachments: []
	},
	{
		ticket_id: 'TKT-002',
		title: 'Safety Equipment Malfunction',
		description:
			'Safety equipment in inspection area is not functioning properly. Needs immediate attention.',
		priority: 'Critical',
		status: 'In Progress',
		category: 'Safety',
		created_by: 'inspector2',
		assigned_to: 'admin1',
		created_at: '2024-01-14T14:20:00Z',
		updated_at: '2024-01-15T09:15:00Z',
		resolution_notes: 'Equipment replacement ordered. ETA: 2 days.',
		attachments: ['safety_report.pdf']
	},
	{
		ticket_id: 'TKT-003',
		title: 'System Performance Issue',
		description: 'Application is running slowly during peak hours. Affecting inspection workflow.',
		priority: 'Medium',
		status: 'Resolved',
		category: 'Technical',
		created_by: 'viewer1',
		assigned_to: 'admin1',
		created_at: '2024-01-13T16:45:00Z',
		updated_at: '2024-01-14T11:30:00Z',
		resolved_at: '2024-01-14T11:30:00Z',
		resolution_notes: 'Server resources optimized. Performance improved by 40%.',
		attachments: []
	},
	{
		ticket_id: 'TKT-004',
		title: 'Maintenance Request - Equipment',
		description:
			'Regular maintenance required for inspection equipment. Scheduled maintenance overdue.',
		priority: 'Low',
		status: 'Open',
		category: 'Maintenance',
		created_by: 'inspector1',
		assigned_to: '',
		created_at: '2024-01-12T08:00:00Z',
		updated_at: '2024-01-12T08:00:00Z',
		resolution_notes: '',
		attachments: []
	},
	{
		ticket_id: 'TKT-005',
		title: 'General Inquiry - Process',
		description: 'Need clarification on new inspection procedures. Documentation unclear.',
		priority: 'Low',
		status: 'Closed',
		category: 'General',
		created_by: 'viewer2',
		assigned_to: 'admin1',
		created_at: '2024-01-11T12:15:00Z',
		updated_at: '2024-01-12T15:45:00Z',
		resolved_at: '2024-01-12T15:45:00Z',
		resolution_notes: 'Process documentation updated. Training session scheduled.',
		attachments: ['process_guide.pdf']
	}
];

export const GET: RequestHandler = async ({ params }) => {
	const ticket = tickets.find((t) => t.ticket_id === params.id);

	if (!ticket) {
		return json({ error: 'Ticket not found' }, { status: 404 });
	}

	return json(ticket);
};

export const PUT: RequestHandler = async ({ params, request }) => {
	const updatedTicket = await request.json();
	const index = tickets.findIndex((t) => t.ticket_id === params.id);

	if (index === -1) {
		return json({ error: 'Ticket not found' }, { status: 404 });
	}

	tickets[index] = {
		...tickets[index],
		...updatedTicket,
		updated_at: new Date().toISOString()
	};

	return json(tickets[index]);
};

export const DELETE: RequestHandler = async ({ params }) => {
	const index = tickets.findIndex((t) => t.ticket_id === params.id);

	if (index === -1) {
		return json({ error: 'Ticket not found' }, { status: 404 });
	}

	tickets.splice(index, 1);
	return json({ success: true });
};
