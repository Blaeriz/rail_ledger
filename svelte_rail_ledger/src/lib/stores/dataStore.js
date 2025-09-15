import { writable } from 'svelte/store';

// Centralized data store
export const dataStore = writable({
  vendors: [],
  batches: [],
  users: [],
  isLoading: false,
  lastFetch: null,
  error: null
});

// Cache duration: 5 minutes
const CACHE_DURATION = 5 * 60 * 1000;

// Optimized data fetching with caching
export async function fetchAllData() {
  const now = Date.now();
  
  // Check if data is cached and still valid
  dataStore.update(store => {
    if (store.lastFetch && (now - store.lastFetch) < CACHE_DURATION && store.vendors.length > 0) {
      return store; // Return cached data
    }
    return { ...store, isLoading: true, error: null };
  });

  try {
    // Try to fetch from APIs first
    try {
      const [vendorsResponse, batchesResponse, usersResponse] = await Promise.all([
        fetch('/api/vendors'),
        fetch('/api/batches'),
        fetch('/api/users')
      ]);

      if (vendorsResponse.ok && batchesResponse.ok && usersResponse.ok) {
        const [vendors, batches, users] = await Promise.all([
          vendorsResponse.json(),
          batchesResponse.json(),
          usersResponse.json()
        ]);

        // Update store with fresh data
        dataStore.set({
          vendors,
          batches,
          users,
          isLoading: false,
          lastFetch: now,
          error: null
        });

        return { vendors, batches, users };
      }
    } catch (apiError) {
      console.warn('API not available, using mock data:', apiError.message);
    }

    // Fallback to mock data if APIs are not available
    const mockData = generateMockData();
    
    dataStore.set({
      vendors: mockData.vendors,
      batches: mockData.batches,
      users: mockData.users,
      isLoading: false,
      lastFetch: now,
      error: null
    });

    return mockData;
  } catch (error) {
    console.error('Error fetching data:', error);
    dataStore.update(store => ({
      ...store,
      isLoading: false,
      error: error.message
    }));
    throw error;
  }
}

// Generate mock data for development
function generateMockData() {
  const vendors = [
    {
      vendor_id: 'V001',
      no_of_batches: 15,
      gst_no: '29ABCDE1234F1Z5',
      pan_number: 'ABCDE1234F',
      city: 'Mumbai',
      state: 'Maharashtra',
      phone_number: '+91-9876543210',
      email: 'vendor1@example.com',
      audit_date: '2024-01-15'
    },
    {
      vendor_id: 'V002',
      no_of_batches: 8,
      gst_no: '07FGHIJ5678K2L6',
      pan_number: 'FGHIJ5678K',
      city: 'Delhi',
      state: 'Delhi',
      phone_number: '+91-9876543211',
      email: 'vendor2@example.com',
      audit_date: '2024-02-20'
    },
    {
      vendor_id: 'V003',
      no_of_batches: 12,
      gst_no: '33KLMNO9012P3M7',
      pan_number: 'KLMNO9012P',
      city: 'Bangalore',
      state: 'Karnataka',
      phone_number: '+91-9876543212',
      email: 'vendor3@example.com',
      audit_date: '2024-01-30'
    }
  ];

  const batches = [
    {
      batch_id: 'B001',
      vendor_id: 'V001',
      batch_size: 100,
      date_of_production: '2024-01-10',
      qc_status: 'pending',
      expiry_date: '2024-02-10',
      last_inspection_date: null,
      fitment_date: null,
      fitment_location: 'Zone A',
      qr_hash: 'abc123def456'
    },
    {
      batch_id: 'B002',
      vendor_id: 'V001',
      batch_size: 150,
      date_of_production: '2024-01-15',
      qc_status: 'completed',
      expiry_date: '2024-02-15',
      last_inspection_date: '2024-01-20',
      fitment_date: '2024-01-25',
      fitment_location: 'Zone B',
      qr_hash: 'def456ghi789'
    },
    {
      batch_id: 'B003',
      vendor_id: 'V002',
      batch_size: 200,
      date_of_production: '2024-01-20',
      qc_status: 'failed',
      expiry_date: '2024-02-20',
      last_inspection_date: '2024-01-25',
      fitment_date: null,
      fitment_location: 'Zone C',
      qr_hash: 'ghi789jkl012'
    },
    {
      batch_id: 'B004',
      vendor_id: 'V003',
      batch_size: 75,
      date_of_production: '2024-01-25',
      qc_status: 'pending',
      expiry_date: '2024-02-25',
      last_inspection_date: null,
      fitment_date: null,
      fitment_location: 'Zone A',
      qr_hash: 'jkl012mno345'
    },
    {
      batch_id: 'B005',
      vendor_id: 'V003',
      batch_size: 120,
      date_of_production: '2024-01-30',
      qc_status: 'completed',
      expiry_date: '2024-03-01',
      last_inspection_date: '2024-02-05',
      fitment_date: '2024-02-10',
      fitment_location: 'Zone D',
      qr_hash: 'mno345pqr678'
    }
  ];

  const users = [
    {
      user_id: 'U001',
      aadhar: '123456789012',
      phone_number: '+91-9876543210',
      name: 'Admin User',
      user_role: 'Admin'
    },
    {
      user_id: 'U002',
      aadhar: '234567890123',
      phone_number: '+91-9876543211',
      name: 'Inspector User',
      user_role: 'Inspector'
    },
    {
      user_id: 'U003',
      aadhar: '345678901234',
      phone_number: '+91-9876543212',
      name: 'Viewer User',
      user_role: 'Viewer'
    }
  ];

  return { vendors, batches, users };
}

// Utility functions for data processing
export function getVendorById(vendorId, vendors) {
  return vendors.find(v => v.vendor_id === vendorId);
}

export function getBatchesByStatus(status, batches) {
  return batches.filter(batch => batch.qc_status === status);
}

export function getBatchesByVendor(vendorId, batches) {
  return batches.filter(batch => batch.vendor_id === vendorId);
}

export function calculateSuccessRate(batches) {
  if (batches.length === 0) return 0;
  const completed = batches.filter(batch => batch.qc_status === 'Pass' || batch.qc_status === 'completed').length;
  return Math.round((completed / batches.length) * 100);
}

export function getRecentBatches(batches, limit = 10) {
  return batches
    .sort((a, b) => new Date(b.date_of_production || 0) - new Date(a.date_of_production || 0))
    .slice(0, limit);
}

export function getOverdueBatches(batches) {
  const now = new Date();
  return batches.filter(batch => 
    batch.qc_status === 'pending' && 
    batch.expiry_date && 
    new Date(batch.expiry_date) < now
  );
}
