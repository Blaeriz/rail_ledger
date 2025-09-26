// Shared types for the Rail Ledger application

export interface User {
	username: string;
	role: 'admin' | 'inspector' | 'viewer';
	user_id?: string;
	name?: string;
	aadhar?: string;
	phone_number?: string;
	user_role?: string;
}

export interface Vendor {
	vendor_id: string;
	city: string;
	state: string;
	no_of_batches: number;
	email: string;
}

export interface Batch {
	batch_id: string;
	vendor_id: string;
	batch_size: number;
	qc_status: 'Pass' | 'Fail' | 'Pending';
	date_of_production: string;
	expiry_date: string;
	fitment_location?: string;
	last_inspection_date?: string;
}

export interface Report {
	reportId: string;
	batchId: string;
	inspectorName: string;
	status: 0 | 1; // 0 = Fail, 1 = Pass
	createdAt: string;
}

export interface Ticket {
	ticket_id: string;
	title: string;
	description: string;
	priority: 'Low' | 'Medium' | 'High' | 'Critical';
	status: 'Open' | 'In Progress' | 'Resolved' | 'Closed';
	category: 'Technical' | 'Quality' | 'Safety' | 'General' | 'Maintenance';
	created_by: string;
	assigned_to?: string;
	created_at: string;
	updated_at: string;
	resolved_at?: string;
	resolution_notes?: string;
	attachments?: string[];
}

export interface SummaryStats {
	totalVendors: number;
	totalBatches: number;
	pendingInspections: number;
	failedBatches: number;
	completedToday?: number;
	urgentInspections?: number;
	recentActivity?: Array<{
		reportId: string;
		batchId: string;
		createdAt: string;
		status: number;
	}>;
}

export interface ChartData {
	labels: string[];
	datasets: Array<{
		label: string;
		data: number[];
		backgroundColor?: string | string[];
		borderColor?: string | string[];
		borderWidth?: number;
	}>;
}

// Utility function to format dates
export function formatDate(dateString: string): string {
	return new Date(dateString).toLocaleDateString();
}

// Utility function to get days since production
export function getDaysSinceProduction(productionDate: string): number {
	const production = new Date(productionDate);
	const now = new Date();
	return Math.floor((now.getTime() - production.getTime()) / (1000 * 60 * 60 * 24));
}

// Utility function to get priority level
export function getPriority(daysSinceProduction: number): { level: string; color: string } {
	if (daysSinceProduction > 30) {
		return {
			level: 'HIGH',
			color: 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10'
		};
	} else if (daysSinceProduction > 14) {
		return {
			level: 'MEDIUM',
			color: 'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10'
		};
	} else {
		return {
			level: 'LOW',
			color: 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10'
		};
	}
}

// Status helpers
export function getStatusColor(status: string): string {
	const statusMap: Record<string, string> = {
		Pass: 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10',
		PASS: 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10',
		Fail: 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10',
		FAIL: 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10',
		Pending: 'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10',
		PENDING: 'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10',
		'PENDING INSPECTION':
			'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10'
	};
	return (
		statusMap[status] || 'border-gray-600 text-white bg-gray-500/5 shadow-lg shadow-gray-500/10'
	);
}

export function getRoleColor(role: string): string {
	const roleMap: Record<string, string> = {
		admin: 'border-purple-400 text-white bg-purple-500/3 shadow-lg shadow-purple-500/5',
		inspector: 'border-blue-400 text-white bg-blue-500/3 shadow-lg shadow-blue-500/5',
		viewer: 'border-green-400 text-white bg-green-500/3 shadow-lg shadow-green-500/5'
	};
	return (
		roleMap[role.toLowerCase()] ||
		'border-gray-500 text-white bg-gray-500/3 shadow-lg shadow-gray-500/5'
	);
}

// Ticket utility functions with modern glow effects
export function getPriorityColor(priority: string): string {
	const priorityMap: Record<string, string> = {
		Low: 'border-2 border-green-500/60 text-white bg-green-500/20 shadow-lg shadow-green-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		Medium: 'border-2 border-yellow-500/60 text-white bg-yellow-500/20 shadow-lg shadow-yellow-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		High: 'border-2 border-orange-500/60 text-white bg-orange-500/20 shadow-lg shadow-orange-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		Critical: 'border-2 border-red-500/60 text-white bg-red-500/20 shadow-lg shadow-red-500/30 rounded-full px-3 py-1 font-semibold text-xs'
	};
	return (
		priorityMap[priority] ||
		'border-2 border-gray-500/60 text-white bg-gray-500/20 shadow-lg shadow-gray-500/30 rounded-full px-3 py-1 font-semibold text-xs'
	);
}

export function getTicketStatusColor(status: string): string {
	const statusMap: Record<string, string> = {
		Open: 'border-2 border-blue-500/60 text-white bg-blue-500/20 shadow-lg shadow-blue-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		'In Progress': 'border-2 border-orange-500/60 text-white bg-orange-500/20 shadow-lg shadow-orange-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		Resolved: 'border-2 border-green-500/60 text-white bg-green-500/20 shadow-lg shadow-green-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		Closed: 'border-2 border-gray-500/60 text-white bg-gray-500/20 shadow-lg shadow-gray-500/30 rounded-full px-3 py-1 font-semibold text-xs'
	};
	return (
		statusMap[status] || 'border-2 border-gray-500/60 text-white bg-gray-500/20 shadow-lg shadow-gray-500/30 rounded-full px-3 py-1 font-semibold text-xs'
	);
}

export function getCategoryColor(category: string): string {
	const categoryMap: Record<string, string> = {
		Technical: 'border-2 border-purple-500/60 text-white bg-purple-500/20 shadow-lg shadow-purple-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		Quality: 'border-2 border-green-500/60 text-white bg-green-500/20 shadow-lg shadow-green-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		Safety: 'border-2 border-red-500/60 text-white bg-red-500/20 shadow-lg shadow-red-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		General: 'border-2 border-blue-500/60 text-white bg-blue-500/20 shadow-lg shadow-blue-500/30 rounded-full px-3 py-1 font-semibold text-xs',
		Maintenance: 'border-2 border-orange-500/60 text-white bg-orange-500/20 shadow-lg shadow-orange-500/30 rounded-full px-3 py-1 font-semibold text-xs'
	};
	return (
		categoryMap[category] ||
		'border-2 border-gray-500/60 text-white bg-gray-500/20 shadow-lg shadow-gray-500/30 rounded-full px-3 py-1 font-semibold text-xs'
	);
}

// Global window interface extensions
declare global {
	interface Window {
		editVendor?: (vendorId: string) => void;
		deleteVendor?: (vendorId: string) => void;
		editBatch?: (batchId: string) => void;
		deleteBatch?: (batchId: string) => void;
		editUser?: (userId: string) => void;
		deleteUser?: (userId: string) => void;
		editReport?: (reportId: string) => void;
		deleteReport?: (reportId: string) => void;
		editTicket?: (ticketId: string) => void;
		deleteTicket?: (ticketId: string) => void;
		viewTicket?: (ticketId: string) => void;
	}
}
