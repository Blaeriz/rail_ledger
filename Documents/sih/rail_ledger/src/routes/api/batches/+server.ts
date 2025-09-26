import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { batch_info } from '$lib/server/db/schema';

// GET all batches
export const GET: RequestHandler = async () => {
	try {
		const result = await db.select().from(batch_info);
		return new Response(JSON.stringify(result), { status: 200 });
	} catch (err) {
		console.error('Error fetching batches:', err);
		return new Response(JSON.stringify({ error: 'Failed to fetch batches' }), { status: 500 });
	}
};

// POST a new batch
export const POST: RequestHandler = async ({ request }) => {
	try {
		const data = await request.json();
		const [inserted] = await db
			.insert(batch_info)
			.values({
				batch_id: data.batch_id,
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
			.returning();
		return new Response(JSON.stringify(inserted), { status: 201 });
	} catch (err) {
		console.error('Error adding batch:', err);
		return new Response(JSON.stringify({ error: 'Failed to add batch' }), { status: 500 });
	}
};
