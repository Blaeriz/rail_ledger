import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { db } from '$lib/db';
import { inspectionReports, batch_info, vendor_info } from '$lib/server/db/schema';
import { sql, eq } from 'drizzle-orm';

const STATE_TO_ID: Record<string, string> = {
	'Andhra Pradesh': 'AP',
	'Arunachal Pradesh': 'AR',
	'Assam': 'AS',
	'Bihar': 'BR',
	'Chhattisgarh': 'CT',
	'Gujarat': 'GJ',
	'Haryana': 'HR',
	'Himachal Pradesh': 'HP',
	'Jharkhand': 'JH',
	'Karnataka': 'KA',
	'Kerala': 'KL',
	'Madhya Pradesh': 'MP',
	'Maharashtra': 'MH',
	'Odisha': 'OR',
	'Punjab': 'PB',
	'Rajasthan': 'RJ',
	'Tamil Nadu': 'TN',
	'Uttar Pradesh': 'UP',
	'West Bengal': 'WB',
	'Andaman & Nicobar Island': 'AN',
	'Delhi': 'DL',
	'Goa': 'GA',
	'Jammu & Kashmir': 'JK',
	'Ladakh': 'LA',
	'Lakshadweep': 'LD',
	'Manipur': 'MN',
	'Meghalaya': 'ML',
	'Mizoram': 'MZ',
	'Nagaland': 'NL',
	'Puducherry': 'PY',
	'Sikkim': 'SK',
	'Tripura': 'TR',
	'Uttarakhand': 'UT'
};

export const GET: RequestHandler = async () => {
	try {
		const rows = await db
			.select({
				state: vendor_info.state,
				count: sql<number>`COUNT(*)`
			})
			.from(inspectionReports)
			.leftJoin(batch_info, eq(inspectionReports.batchId, batch_info.batch_id))
			.leftJoin(vendor_info, eq(batch_info.vendor_id, vendor_info.vendor_id))
			.where(eq(inspectionReports.status, 1))
			.groupBy(vendor_info.state);

		const data = rows
			.filter(r => r.state)
			.map(r => ({
				id: STATE_TO_ID[r.state as string] ?? r.state,
				state: r.state as string,
				pendingInspections: Number(r.count)
			}))
			.sort((a, b) => b.pendingInspections - a.pendingInspections);

		return json({
			success: true,
			data,
			totalPending: data.reduce((sum, item) => sum + item.pendingInspections, 0),
			lastUpdated: new Date().toISOString()
		});
	} catch (error) {
		console.error('Error fetching heatmap data:', error);
		return json({ success: false, error: 'Failed to fetch heatmap data' }, { status: 500 });
	}
};
