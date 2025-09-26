import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Mock ticket data - Comprehensive dataset for all user types
let tickets = [
	// Admin-created tickets
	{
		ticket_id: 'TKT-001',
		title: 'System Security Audit Required',
		description: 'Quarterly security audit needs to be conducted across all railway management systems. This includes penetration testing and vulnerability assessment.',
		priority: 'High',
		status: 'Open',
		category: 'Technical',
		created_by: 'admin1',
		assigned_to: 'admin2',
		created_at: '2024-01-20T09:00:00Z',
		updated_at: '2024-01-20T09:00:00Z',
		resolution_notes: '',
		attachments: []
	},
	{
		ticket_id: 'TKT-002',
		title: 'Vendor Compliance Review',
		description: 'Annual vendor compliance review for all suppliers. Need to verify certifications and quality standards.',
		priority: 'Medium',
		status: 'In Progress',
		category: 'Quality',
		created_by: 'admin1',
		assigned_to: 'admin3',
		created_at: '2024-01-19T14:30:00Z',
		updated_at: '2024-01-20T10:15:00Z',
		resolution_notes: 'Started reviewing vendor documentation. 60% complete.',
		attachments: ['vendor_compliance_checklist.pdf']
	},
	{
		ticket_id: 'TKT-003',
		title: 'Database Backup Strategy Update',
		description: 'Current backup strategy needs enhancement to meet new compliance requirements. Need to implement automated daily backups.',
		priority: 'High',
		status: 'Resolved',
		category: 'Technical',
		created_by: 'admin2',
		assigned_to: 'admin1',
		created_at: '2024-01-18T11:20:00Z',
		updated_at: '2024-01-19T16:45:00Z',
		resolved_at: '2024-01-19T16:45:00Z',
		resolution_notes: 'Automated backup system implemented. Daily backups now running successfully.',
		attachments: ['backup_configuration.pdf']
	},
	
	// Inspector-created tickets
	{
		ticket_id: 'TKT-004',
		title: 'Quality Control Issue - Batch #12345',
		description: 'Found quality issues in batch 12345 during inspection. Multiple components showing defects in rail clips.',
		priority: 'High',
		status: 'Open',
		category: 'Quality',
		created_by: 'inspector1',
		assigned_to: 'admin1',
		created_at: '2024-01-17T10:30:00Z',
		updated_at: '2024-01-17T10:30:00Z',
		resolution_notes: '',
		attachments: ['defect_photos.zip']
	},
	{
		ticket_id: 'TKT-005',
		title: 'Safety Equipment Malfunction',
		description: 'Safety equipment in inspection area is not functioning properly. Emergency stop button not responding.',
		priority: 'Critical',
		status: 'In Progress',
		category: 'Safety',
		created_by: 'inspector2',
		assigned_to: 'admin2',
		created_at: '2024-01-16T14:20:00Z',
		updated_at: '2024-01-17T09:15:00Z',
		resolution_notes: 'Equipment replacement ordered. ETA: 2 days. Temporary safety measures implemented.',
		attachments: ['safety_report.pdf', 'equipment_photos.jpg']
	},
	{
		ticket_id: 'TKT-006',
		title: 'QR Code Scanner Calibration',
		description: 'QR code scanner needs recalibration. Reading accuracy has decreased to 85%.',
		priority: 'Medium',
		status: 'Resolved',
		category: 'Maintenance',
		created_by: 'inspector1',
		assigned_to: 'admin3',
		created_at: '2024-01-15T16:45:00Z',
		updated_at: '2024-01-16T11:30:00Z',
		resolved_at: '2024-01-16T11:30:00Z',
		resolution_notes: 'Scanner calibrated successfully. Reading accuracy restored to 99.2%.',
		attachments: ['calibration_report.pdf']
	},
	{
		ticket_id: 'TKT-007',
		title: 'Inspection Schedule Conflict',
		description: 'Scheduled inspection conflicts with maintenance window. Need to reschedule or coordinate.',
		priority: 'Low',
		status: 'Closed',
		category: 'General',
		created_by: 'inspector2',
		assigned_to: 'admin1',
		created_at: '2024-01-14T08:00:00Z',
		updated_at: '2024-01-14T15:30:00Z',
		resolved_at: '2024-01-14T15:30:00Z',
		resolution_notes: 'Inspection rescheduled to next available slot. Maintenance team notified.',
		attachments: []
	},
	{
		ticket_id: 'TKT-008',
		title: 'New Inspection Protocol Training',
		description: 'Team needs training on new inspection protocols for rail pad quality assessment.',
		priority: 'Medium',
		status: 'Open',
		category: 'General',
		created_by: 'inspector3',
		assigned_to: 'admin2',
		created_at: '2024-01-13T12:15:00Z',
		updated_at: '2024-01-13T12:15:00Z',
		resolution_notes: '',
		attachments: ['new_protocol_guide.pdf']
	},
	
	// Viewer-created tickets
	{
		ticket_id: 'TKT-009',
		title: 'System Performance Issue',
		description: 'Application is running slowly during peak hours. Affecting data viewing and report generation.',
		priority: 'Medium',
		status: 'Resolved',
		category: 'Technical',
		created_by: 'viewer1',
		assigned_to: 'admin1',
		created_at: '2024-01-12T16:45:00Z',
		updated_at: '2024-01-13T11:30:00Z',
		resolved_at: '2024-01-13T11:30:00Z',
		resolution_notes: 'Server resources optimized. Performance improved by 40%. Cache implemented.',
		attachments: ['performance_report.pdf']
	},
	{
		ticket_id: 'TKT-010',
		title: 'Report Export Functionality',
		description: 'Unable to export reports in PDF format. Error message appears when trying to download.',
		priority: 'Low',
		status: 'In Progress',
		category: 'Technical',
		created_by: 'viewer2',
		assigned_to: 'admin3',
		created_at: '2024-01-11T14:20:00Z',
		updated_at: '2024-01-12T10:15:00Z',
		resolution_notes: 'Identified issue with PDF generation library. Working on fix.',
		attachments: ['error_screenshot.png']
	},
	{
		ticket_id: 'TKT-011',
		title: 'Data Access Request',
		description: 'Need access to historical inspection data for analysis. Current permissions are limited.',
		priority: 'Low',
		status: 'Open',
		category: 'General',
		created_by: 'viewer3',
		assigned_to: 'admin1',
		created_at: '2024-01-10T11:30:00Z',
		updated_at: '2024-01-10T11:30:00Z',
		resolution_notes: '',
		attachments: ['access_request_form.pdf']
	},
	{
		ticket_id: 'TKT-012',
		title: 'Dashboard Customization',
		description: 'Would like to customize dashboard layout to show specific metrics relevant to my role.',
		priority: 'Low',
		status: 'Closed',
		category: 'General',
		created_by: 'viewer1',
		assigned_to: 'admin2',
		created_at: '2024-01-09T09:15:00Z',
		updated_at: '2024-01-10T14:30:00Z',
		resolved_at: '2024-01-10T14:30:00Z',
		resolution_notes: 'Dashboard customization feature implemented. User can now configure their view.',
		attachments: ['customization_guide.pdf']
	},
	{
		ticket_id: 'TKT-013',
		title: 'Mobile App Login Issue',
		description: 'Cannot login to mobile app. Getting authentication error even with correct credentials.',
		priority: 'Medium',
		status: 'Resolved',
		category: 'Technical',
		created_by: 'viewer2',
		assigned_to: 'admin3',
		created_at: '2024-01-08T13:45:00Z',
		updated_at: '2024-01-09T16:20:00Z',
		resolved_at: '2024-01-09T16:20:00Z',
		resolution_notes: 'Mobile app authentication bug fixed. Update available in app store.',
		attachments: []
	},
	{
		ticket_id: 'TKT-014',
		title: 'Notification Settings',
		description: 'Email notifications are not being received for ticket updates. Checked spam folder.',
		priority: 'Low',
		status: 'In Progress',
		category: 'Technical',
		created_by: 'viewer3',
		assigned_to: 'admin1',
		created_at: '2024-01-07T15:30:00Z',
		updated_at: '2024-01-08T09:45:00Z',
		resolution_notes: 'Email server configuration issue identified. Working with IT team to resolve.',
		attachments: ['email_logs.txt']
	},
	{
		ticket_id: 'TKT-015',
		title: 'Batch Data Inconsistency',
		description: 'Noticed inconsistency in batch data display. Some batches show different information in different views.',
		priority: 'High',
		status: 'Open',
		category: 'Quality',
		created_by: 'viewer1',
		assigned_to: 'admin2',
		created_at: '2024-01-06T10:20:00Z',
		updated_at: '2024-01-06T10:20:00Z',
		resolution_notes: '',
		attachments: ['data_inconsistency_report.pdf']
	},
	{
		ticket_id: 'TKT-016',
		title: 'User Manual Update Request',
		description: 'Current user manual is outdated. Need updated documentation for new features.',
		priority: 'Low',
		status: 'Closed',
		category: 'General',
		created_by: 'viewer2',
		assigned_to: 'admin3',
		created_at: '2024-01-05T12:00:00Z',
		updated_at: '2024-01-06T14:30:00Z',
		resolved_at: '2024-01-06T14:30:00Z',
		resolution_notes: 'Updated user manual published. Available in documentation section.',
		attachments: ['updated_manual_v2.1.pdf']
	},
	{
		ticket_id: 'TKT-017',
		title: 'System Maintenance Window',
		description: 'Request for scheduled maintenance window to perform system updates without affecting operations.',
		priority: 'Medium',
		status: 'Open',
		category: 'Maintenance',
		created_by: 'admin3',
		assigned_to: 'admin1',
		created_at: '2024-01-04T16:00:00Z',
		updated_at: '2024-01-04T16:00:00Z',
		resolution_notes: '',
		attachments: ['maintenance_plan.pdf']
	},
	{
		ticket_id: 'TKT-018',
		title: 'Vendor Portal Integration',
		description: 'Need to integrate new vendor portal with existing system. This will streamline vendor management.',
		priority: 'High',
		status: 'In Progress',
		category: 'Technical',
		created_by: 'admin2',
		assigned_to: 'admin3',
		created_at: '2024-01-03T11:15:00Z',
		updated_at: '2024-01-04T13:45:00Z',
		resolution_notes: 'API integration 70% complete. Testing phase initiated.',
		attachments: ['integration_specs.pdf', 'test_results.pdf']
	},
	{
		ticket_id: 'TKT-019',
		title: 'Data Backup Verification',
		description: 'Need to verify that all critical data is being backed up properly. Last verification was 3 months ago.',
		priority: 'Medium',
		status: 'Resolved',
		category: 'Technical',
		created_by: 'admin1',
		assigned_to: 'admin2',
		created_at: '2024-01-02T09:30:00Z',
		updated_at: '2024-01-03T15:20:00Z',
		resolved_at: '2024-01-03T15:20:00Z',
		resolution_notes: 'Backup verification completed. All systems operational. Recovery test successful.',
		attachments: ['backup_verification_report.pdf']
	},
	{
		ticket_id: 'TKT-020',
		title: 'Training Material Request',
		description: 'Need comprehensive training materials for new team members joining the inspection department.',
		priority: 'Low',
		status: 'Open',
		category: 'General',
		created_by: 'inspector1',
		assigned_to: 'admin2',
		created_at: '2024-01-01T14:00:00Z',
		updated_at: '2024-01-01T14:00:00Z',
		resolution_notes: '',
		attachments: ['training_requirements.pdf']
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
