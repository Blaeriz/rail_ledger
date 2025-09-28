import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { vendor_info } from '$lib/server/db/schema';
import { logEvent } from '$lib/eventLog';

// ✅ GET all vendors
export const GET: RequestHandler = async () => {
  try {
    const result = await db.select().from(vendor_info);
    logEvent('/api/vendors', 'GET', 'success'); // Track successful fetch
    return new Response(JSON.stringify(result), { status: 200 });
  } catch (err) {
    console.error('Error fetching vendors:', err);
    logEvent('/api/vendors', 'GET', 'error'); // Track failure
    return new Response(JSON.stringify({ error: 'Failed to fetch vendors' }), { status: 500 });
  }
};

// ✅ POST a new vendor
export const POST: RequestHandler = async ({ request }) => {
  try {
    const data = await request.json();
    await db.insert(vendor_info).values({
      vendor_id: data.vendor_id,
      no_of_batches: data.no_of_batches,
      gst_no: data.gst_no,
      pan_number: data.pan_number,
      city: data.city,
      state: data.state,
      phone_number: data.phone_number,
      email: data.email,
      audit_date: data.audit_date
    });
    logEvent('/api/vendors', 'POST', 'success'); // Track successful insert
    return new Response(JSON.stringify({ message: 'Vendor added successfully' }), { status: 201 });
  } catch (err) {
    console.error('Error adding vendor:', err);
    logEvent('/api/vendors', 'POST', 'error'); // Track failure
    return new Response(JSON.stringify({ error: 'Failed to add vendor' }), { status: 500 });
  }
};
