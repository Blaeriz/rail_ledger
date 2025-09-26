-- Sample data for testing AI reports
-- Run this SQL to populate the database with test data

-- Insert sample users
INSERT INTO user_info (user_id, aadhar, phone_number, name, user_role) VALUES
('admin1', '123456789012', '9876543210', 'Admin User', 'admin'),
('inspector1', '123456789013', '9876543211', 'John Inspector', 'inspector'),
('viewer1', '123456789014', '9876543212', 'Jane Viewer', 'viewer');

-- Insert sample vendors
INSERT INTO vendor_info (vendor_id, no_of_batches, gst_no, pan_number, city, state, phone_number, email, audit_date) VALUES
('VENDOR001', 15, '29ABCDE1234F1Z5', 'ABCDE1234F', 'Mumbai', 'Maharashtra', '9876543210', 'vendor1@example.com', '2024-01-01'),
('VENDOR002', 8, '29FGHIJ5678K2L6', 'FGHIJ5678K', 'Delhi', 'Delhi', '9876543211', 'vendor2@example.com', '2024-01-15'),
('VENDOR003', 12, '29MNOPQ9012R3S7', 'MNOPQ9012R', 'Bangalore', 'Karnataka', '9876543212', 'vendor3@example.com', '2024-02-01');

-- Insert sample batches
INSERT INTO batch_info (batch_id, vendor_id, batch_size, date_of_production, qc_status, expiry_date, last_inspection_date, fitment_date, fitment_location, qr_hash) VALUES
('BATCH001', 'VENDOR001', 100, '2024-01-01', 'Passed', '2025-01-01', '2024-01-15', '2024-01-20', 'Track Section A-1', 'QR001'),
('BATCH002', 'VENDOR001', 150, '2024-01-05', 'Failed', '2025-01-05', '2024-01-18', NULL, NULL, 'QR002'),
('BATCH003', 'VENDOR002', 75, '2024-01-10', 'Passed', '2025-01-10', '2024-01-20', '2024-01-25', 'Track Section B-2', 'QR003'),
('BATCH004', 'VENDOR002', 200, '2024-01-15', 'Pending', '2025-01-15', '2024-01-22', NULL, NULL, 'QR004'),
('BATCH005', 'VENDOR003', 120, '2024-01-20', 'Passed', '2025-01-20', '2024-01-25', '2024-01-30', 'Track Section C-3', 'QR005');

-- Insert sample inspection reports
INSERT INTO inspection_reports (batch_id, inspector_name, remark, status, created_at) VALUES
('BATCH001', 'John Inspector', 'All components passed quality inspection', 0, '2024-01-15T10:30:00Z'),
('BATCH001', 'John Inspector', 'Follow-up inspection completed successfully', 0, '2024-01-16T14:20:00Z'),
('BATCH002', 'John Inspector', 'Critical defects found in welding joints', 1, '2024-01-18T09:15:00Z'),
('BATCH002', 'John Inspector', 'Safety hazard identified in component structure', 1, '2024-01-19T11:45:00Z'),
('BATCH003', 'John Inspector', 'Quality standards met, minor surface imperfections noted', 0, '2024-01-20T13:30:00Z'),
('BATCH004', 'John Inspector', 'Inspection in progress, initial findings positive', 0, '2024-01-22T08:00:00Z'),
('BATCH005', 'John Inspector', 'Excellent quality, all metrics exceeded expectations', 0, '2024-01-25T16:45:00Z'),
('BATCH005', 'John Inspector', 'Final inspection completed, ready for fitment', 0, '2024-01-26T10:15:00Z');
