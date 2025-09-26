import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { batch_info } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';

// GET a single batch
export const GET: RequestHandler = async ({ params }) => {
	try {
		const id = params.id;
		if (!id) {
			return new Response(JSON.stringify({ error: 'Batch ID is required' }), { status: 400 });
		}
		const [batch] = await db.select().from(batch_info).where(eq(batch_info.batch_id, id));
		if (!batch) {
			return new Response(JSON.stringify({ error: 'Batch not found' }), { status: 404 });
		}
		return new Response(JSON.stringify(batch), { status: 200 });
	} catch (err) {
		console.error('Error fetching batch:', err);
		return new Response(JSON.stringify({ error: 'Failed to fetch batch. ..' }), { status: 500 });
	}
};

// UPDATE a batch
export const PUT: RequestHandler = async ({ params, request }) => {
	try {
		const id = params.id;
		if (!id) {
			return new Response(JSON.stringify({ error: 'Batch ID is required' }), { status: 400 });
		}
		const data = await request.json();
		const [updated] = await db
			.update(batch_info)
			.set({
				vendor_id: data.vendor_id,
				batch_size: data.batch_size,
				date_of_production: data.date_of_production,
				qc_status: data.qc_status,
				expiry_date: data.expiry_date,
				last_inspection_date: data.last_inspection_date,
				fitment_date: data.fitment_date,
				fitment_location: data.fitment_location,
				qr_hash: data.qr_hash
			})
			.where(eq(batch_info.batch_id, id))
			.returning();

		if (!updated) {
			return new Response(JSON.stringify({ error: 'Batch not found' }), { status: 404 });
		}

		return new Response(JSON.stringify(updated), { status: 200 });
	} catch (err) {
		console.error('Error updating batch:', err);
		return new Response(JSON.stringify({ error: 'Failed to update batch' }), { status: 500 });
	}
};

// DELETE a batch
export const DELETE: RequestHandler = async ({ params }) => {
	try {
		const id = params.id;
		if (!id) {
			return new Response(JSON.stringify({ error: 'Batch ID is required' }), { status: 400 });
		}
		const [deleted] = await db.delete(batch_info).where(eq(batch_info.batch_id, id)).returning();
		if (!deleted) {
			return new Response(JSON.stringify({ error: 'Batch not found' }), { status: 404 });
		}
		return new Response(JSON.stringify({ message: 'Batch deleted successfully' }), { status: 200 });
	} catch (err) {
		console.error('Error deleting batch:', err);
		return new Response(JSON.stringify({ error: 'Failed to delete batch' }), { status: 500 });
	}
};
