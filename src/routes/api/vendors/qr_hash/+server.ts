import type { RequestHandler } from '@sveltejs/kit';
import crypto from 'crypto';
import { db } from '$lib/db';
import { vendor_info } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';

export const POST: RequestHandler = async ({ request }) => {
  try {
    const { vendorId, batchId } = await request.json();

    if (!vendorId || !batchId) {
      return new Response(
        JSON.stringify({ error: 'vendorId and batchId are required' }),
        { status: 400 }
      );
    }

    // Check if vendor exists
    const [vendor] = await db.select().from(vendor_info).where(eq(vendor_info.vendor_id, vendorId));
    if (!vendor) {
      return new Response(
        JSON.stringify({ error: 'Vendor does not exist' }),
        { status: 404 }
      );
    }

    // Create HMAC hash using a fixed secret
    const secret = "avni"; 
    const hmac = crypto.createHmac('sha256', secret);
    hmac.update(vendorId + batchId);
    const hash = hmac.digest('hex').slice(0, 16);

    return new Response(JSON.stringify({ qr_hash: hash }), { status: 200 });
  } catch (err) {
    console.error('Error generating QR hash:', err);
    return new Response(JSON.stringify({ error: 'Failed to generate QR hash' }), { status: 500 });
  }
};
