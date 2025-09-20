import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { vendor_info } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';

// GET a single vendor
export const GET: RequestHandler = async ({ params }) => {
  try {
    const id = params.id; // vendor_id is text
    if (!id) {
      return new Response(JSON.stringify({ error: 'Vendor ID is required' }), { status: 400 });
    }
    const [vendor] = await db.select().from(vendor_info).where(eq(vendor_info.vendor_id, id));
    if (!vendor) {
      return new Response(JSON.stringify({ error: 'Vendor not found' }), { status: 404 });
    }
    return new Response(JSON.stringify(vendor), { status: 200 });
  } catch (err) {
    console.error('Error fetching vendor:', err);
    return new Response(JSON.stringify({ error: 'Failed to fetch vendor' }), { status: 500 });
  }
};

// UPDATE a vendor
export const PUT: RequestHandler = async ({ params, request }) => {
  try {
    const id = params.id;
    if (!id) {
      return new Response(JSON.stringify({ error: 'Vendor ID is required' }), { status: 400 });
    }
    const data = await request.json();
    const [updated] = await db
      .update(vendor_info)
      .set({
        no_of_batches: data.no_of_batches,
        gst_no: data.gst_no,
        pan_number: data.pan_number,
        city: data.city,
        state: data.state,
        phone_number: data.phone_number,
        email: data.email,
        audit_date: data.audit_date
      })
      .where(eq(vendor_info.vendor_id, id))
      .returning();

    if (!updated) {
      return new Response(JSON.stringify({ error: 'Vendor not found' }), { status: 404 });
    }
    return new Response(JSON.stringify(updated), { status: 200 });
  } catch (err) {
    console.error('Error updating vendor:', err);
    return new Response(JSON.stringify({ error: 'Failed to update vendor' }), { status: 500 });
  }
};

// DELETE a vendor
export const DELETE: RequestHandler = async ({ params }) => {
  try {
    const id = params.id;
    if (!id) {
      return new Response(JSON.stringify({ error: 'Vendor ID is required' }), { status: 400 });
    }
    const [deleted] = await db.delete(vendor_info).where(eq(vendor_info.vendor_id, id)).returning();
    if (!deleted) {
      return new Response(JSON.stringify({ error: 'Vendor not found' }), { status: 404 });
    }
    return new Response(JSON.stringify({ message: 'Vendor deleted successfully' }), { status: 200 });
  } catch (err) {
    console.error('Error deleting vendor:', err);
    return new Response(JSON.stringify({ error: 'Failed to delete vendor' }), { status: 500 });
  }
};
