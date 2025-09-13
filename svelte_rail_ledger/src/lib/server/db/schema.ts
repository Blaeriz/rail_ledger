import { pgTable, text, varchar, integer, date } from 'drizzle-orm/pg-core';

// Users table
export const user_info = pgTable('user_info', {
  user_id: text('user_id').primaryKey(),
  aadhar: varchar('aadhar', { length: 12 }),
  phone_number: varchar('phone_number', { length: 15 }),
  name: text('name'),
  user_role: text('user_role')
});

export const batch_info = pgTable(
  'batch_info',
  {
    batch_id: text('batch_id').primaryKey(),
    vendor_id: text('vendor_id'),
    batch_size: integer('batch_size'),
    date_of_production: date('date_of_production'),
    qc_status: text('qc_status'),
    expiry_date: date('expiry_date'),
    last_inspection_date: date('last_inspection_date'),
    fitment_date: date('fitment_date'),
    fitment_location: text('fitment_location'),
    qr_hash: text('qr_hash'),
  }
);

// Vendors table
export const vendor_info = pgTable('vendor_info', {
  vendor_id: text('vendor_id').primaryKey(),
  no_of_batches: integer('no_of_batches'),
  gst_no: varchar('gst_no', { length: 20 }),
  pan_number: varchar('pan_number', { length: 10 }),
  city: text('city'),
  state: text('state'),
  phone_number: varchar('phone_number', { length: 15 }),
  email: text('email'),
  audit_date: date('audit_date')
});
