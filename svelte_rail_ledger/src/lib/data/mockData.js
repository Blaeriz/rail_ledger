// Mock data for the application
export const mockVendors = [
  {
    vendor_id: 'V001',
    name: 'Steel Works Ltd',
    city: 'Mumbai',
    state: 'Maharashtra',
    audit_date: '2024-01-15',
    gst_no: '29ABCDE1234F1Z5',
    pan_number: 'ABCDE1234F',
    email: 'contact@steelworks.com',
    phone: '+91-9876543210',
    no_of_batches: 15
  },
  {
    vendor_id: 'V002',
    name: 'Metal Solutions Inc',
    city: 'Delhi',
    state: 'Delhi',
    audit_date: '2024-02-10',
    gst_no: '07FGHIJ5678K2L6',
    pan_number: 'FGHIJ5678K',
    email: 'info@metalsolutions.com',
    phone: '+91-9876543211',
    no_of_batches: 8
  }
];

export const mockBatches = [
  {
    batch_id: 'B001',
    vendor_id: 'V001',
    vendor_name: 'Steel Works Ltd',
    batch_size: 100,
    production_date: '2024-01-10',
    qc_status: 'Pass',
    fitment_status: 'Completed',
    expiry_date: '2025-01-10',
    qr_hash: 'qr_hash_001_abc123'
  },
  {
    batch_id: 'B002',
    vendor_id: 'V001',
    vendor_name: 'Steel Works Ltd',
    batch_size: 150,
    production_date: '2024-01-12',
    qc_status: 'Pending Inspection',
    fitment_status: 'Not Started',
    expiry_date: '2025-01-12',
    qr_hash: 'qr_hash_002_def456'
  }
];

export const mockUsers = [
  {
    user_id: 'u001',
    aadhar: '123456789012',
    name: 'Rajesh Kumar',
    phone: '+91-9876543210',
    role: 'Admin'
  },
  {
    user_id: 'u002',
    aadhar: '234567890123',
    name: 'Priya Sharma',
    phone: '+91-9876543211',
    role: 'Inspector'
  }
];

export const mockInspections = [
  {
    batch_id: 'B001',
    inspection_date: '2024-02-10',
    type: 'Quality Check',
    status: 'Pass',
    remarks: 'All good'
  },
  {
    batch_id: 'B002',
    inspection_date: '2024-02-12',
    type: 'Safety Check',
    status: 'Fail',
    remarks: 'Minor issues found'
  }
];

export const mockFitments = [
  {
    batch_id: 'B001',
    fitment_date: '2024-02-15',
    location: 'Platform 1, Mumbai Central',
    photo: 'photo1.jpg'
  }
];
