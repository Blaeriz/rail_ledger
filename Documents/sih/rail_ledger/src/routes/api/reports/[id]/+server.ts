import type { RequestHandler } from '@sveltejs/kit';
import { db } from '$lib/db';
import { inspectionReports } from '$lib/server/db/schema';
import { eq } from 'drizzle-orm';

// ✅ GET single report
export const GET: RequestHandler = async ({ params }) => {
	try {
		const id = Number(params.id);
		const [report] = await db
			.select()
			.from(inspectionReports)
			.where(eq(inspectionReports.reportId, id));
		if (!report)
			return new Response(JSON.stringify({ error: 'Report not found' }), { status: 404 });
		return new Response(JSON.stringify(report), { status: 200 });
	} catch (err) {
		console.error('Error fetching report:', err);
		return new Response(JSON.stringify({ error: 'Failed to fetch report' }), { status: 500 });
	}
};

// ✅ PUT update report
export const PUT: RequestHandler = async ({ params, request }) => {
	try {
		const id = Number(params.id);
		const data = await request.json();
		const [updated] = await db
			.update(inspectionReports)
			.set({
				batchId: data.batchId,
				inspectorName: data.inspectorName,
				remark: data.remark,
				status: data.status
			})
			.where(eq(inspectionReports.reportId, id))
			.returning();
		if (!updated)
			return new Response(JSON.stringify({ error: 'Report not found' }), { status: 404 });
		return new Response(JSON.stringify(updated), { status: 200 });
	} catch (err) {
		console.error('Error updating report:', err);
		return new Response(JSON.stringify({ error: 'Failed to update report' }), { status: 500 });
	}
};

// ✅ DELETE report
export const DELETE: RequestHandler = async ({ params }) => {
	try {
		const id = Number(params.id);
		const [deleted] = await db
			.delete(inspectionReports)
			.where(eq(inspectionReports.reportId, id))
			.returning();
		if (!deleted)
			return new Response(JSON.stringify({ error: 'Report not found' }), { status: 404 });
		return new Response(JSON.stringify({ message: 'Report deleted successfully' }), {
			status: 200
		});
	} catch (err) {
		console.error('Error deleting report:', err);
		return new Response(JSON.stringify({ error: 'Failed to delete report' }), { status: 500 });
	}
};
