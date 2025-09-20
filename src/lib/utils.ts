// Shared utility functions for the Rail Ledger application

import type { User, Batch, Vendor, Report, SummaryStats } from './types';

// API fetch functions
export async function fetchData<T>(url: string): Promise<T[]> {
  try {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
    return await response.json();
  } catch (error) {
    console.error(`Error fetching data from ${url}:`, error);
    return [];
  }
}

// CRUD operations
export async function createItem<T>(url: string, data: Partial<T>): Promise<boolean> {
  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data)
    });
    return response.ok;
  } catch (error) {
    console.error('Error creating item:', error);
    return false;
  }
}

export async function updateItem<T>(url: string, id: string, data: Partial<T>): Promise<boolean> {
  try {
    const response = await fetch(`${url}/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data)
    });
    return response.ok;
  } catch (error) {
    console.error('Error updating item:', error);
    return false;
  }
}

export async function deleteItem(url: string, id: string): Promise<boolean> {
  try {
    const response = await fetch(`${url}/${id}`, {
      method: 'DELETE'
    });
    return response.ok;
  } catch (error) {
    console.error('Error deleting item:', error);
    return false;
  }
}

// Data processing functions
export function calculateSummaryStats(
  vendors: Vendor[],
  batches: Batch[],
  reports: Report[]
): SummaryStats {
  const totalVendors = vendors.length;
  const totalBatches = batches.length;
  const pendingInspections = batches.filter(b => 
    b.qc_status === 'Pending Inspection' || b.qc_status === 'Pending' || b.qc_status === 'PENDING' || b.qc_status === 'PENDING INSPECTION' ||
    b.status === 'Pending Inspection' || b.status === 'Pending' || b.status === 'PENDING' || b.status === 'PENDING INSPECTION' ||
    b.inspection_status === 'Pending Inspection' || b.inspection_status === 'Pending' || b.inspection_status === 'PENDING' || b.inspection_status === 'PENDING INSPECTION'
  ).length;
  const failedBatches = reports.filter(r => r.status === 0).length;
  
  return {
    totalVendors,
    totalBatches,
    pendingInspections,
    failedBatches,
    completedToday: 0, // This would be calculated based on today's date
    urgentInspections: 0, // This would be calculated based on priority
    recentActivity: reports.slice(0, 5).map(r => ({
      reportId: r.reportId,
      batchId: r.batchId,
      createdAt: r.createdAt,
      status: r.status
    }))
  };
}

// Chart configuration helpers
export function getChartColors(type: 'primary' | 'secondary' | 'success' | 'warning' | 'danger'): string[] {
  const colorMap = {
    primary: ['#8B5CF6', '#A78BFA', '#C4B5FD'],
    secondary: ['#6B7280', '#9CA3AF', '#D1D5DB'],
    success: ['#10B981', '#34D399', '#6EE7B7'],
    warning: ['#F59E0B', '#FBBF24', '#FCD34D'],
    danger: ['#EF4444', '#F87171', '#FCA5A5']
  };
  return colorMap[type] || colorMap.primary;
}

// Form validation
export function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

export function validatePhone(phone: string): boolean {
  const phoneRegex = /^[\+]?[1-9][\d]{0,15}$/;
  return phoneRegex.test(phone.replace(/\s/g, ''));
}

export function validateAadhar(aadhar: string): boolean {
  const aadharRegex = /^\d{12}$/;
  return aadharRegex.test(aadhar.replace(/\s/g, ''));
}

// Status helpers
export function getStatusColor(status: string): string {
  const statusMap: Record<string, string> = {
    'Pass': 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10',
    'PASS': 'border-green-600 text-white bg-green-500/5 shadow-lg shadow-green-500/10',
    'Fail': 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10',
    'FAIL': 'border-red-600 text-white bg-red-500/5 shadow-lg shadow-red-500/10',
    'Pending': 'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10',
    'PENDING': 'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10',
    'PENDING INSPECTION': 'border-yellow-600 text-white bg-yellow-500/5 shadow-lg shadow-yellow-500/10'
  };
  return statusMap[status] || 'border-gray-600 text-white bg-gray-500/5 shadow-lg shadow-gray-500/10';
}

export function getRoleColor(role: string): string {
  const roleMap: Record<string, string> = {
    'admin': 'border-purple-400 text-white bg-purple-500/3 shadow-lg shadow-purple-500/5',
    'inspector': 'border-blue-400 text-white bg-blue-500/3 shadow-lg shadow-blue-500/5',
    'viewer': 'border-green-400 text-white bg-green-500/3 shadow-lg shadow-green-500/5'
  };
  return roleMap[role.toLowerCase()] || 'border-gray-500 text-white bg-gray-500/3 shadow-lg shadow-gray-500/5';
}
