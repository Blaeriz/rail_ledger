import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { batch_info } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';

// GET /api/batches/qr_hash?qr_hash=...
export const GET: RequestHandler = async ({ url }) => {
  try {
    const qrHash = url.searchParams.get('qr_hash');
    if (!qrHash) {
      return new Response(JSON.stringify({ error: 'qr_hash is required' }), { status: 400 });
    }

    const rows = await db.select().from(batch_info).where(eq(batch_info.qr_hash, qrHash)).limit(1);
    if (!rows.length) {
      return new Response(JSON.stringify({ error: 'Batch not found' }), { status: 404 });
    }
    return new Response(JSON.stringify(rows[0]), { status: 200 });
  } catch (err) {
    console.error('Error fetching batch by qr_hash:', err);
    return new Response(JSON.stringify({ error: 'Failed to fetch batch by qr_hash' }), { status: 500 });
  }
};
