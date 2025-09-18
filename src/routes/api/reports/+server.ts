import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { inspectionReports } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';

// ✅ GET all reports
export const GET: RequestHandler = async () => {
  try {
    const reports = await db.select().from(inspectionReports);
    return new Response(JSON.stringify(reports), { status: 200 });
  } catch (err) {
    console.error('Error fetching reports:', err);
    return new Response(JSON.stringify({ error: 'Failed to fetch reports' }), { status: 500 });
  }
};

// ✅ POST a new report
export const POST: RequestHandler = async ({ request }) => {
  try {
    const data = await request.json();
    const [newReport] = await db
      .insert(inspectionReports)
      .values({
        batchId: data.batchId,
        inspectorName: data.inspectorName,
        remark: data.remark,
        status: data.status
      })
      .returning();
    return new Response(JSON.stringify(newReport), { status: 201 });
  } catch (err) {
    console.error('Error adding report:', err);
    return new Response(JSON.stringify({ error: 'Failed to add report' }), { status: 500 });
  }
};
