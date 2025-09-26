import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Sample data for Indian states with pending inspections
const sampleHeatmapData = [
	{ id: 'MH', state: 'Maharashtra', pendingInspections: 45 },
	{ id: 'UP', state: 'Uttar Pradesh', pendingInspections: 38 },
	{ id: 'WB', state: 'West Bengal', pendingInspections: 32 },
	{ id: 'TN', state: 'Tamil Nadu', pendingInspections: 28 },
	{ id: 'GJ', state: 'Gujarat', pendingInspections: 25 },
	{ id: 'KA', state: 'Karnataka', pendingInspections: 22 },
	{ id: 'RJ', state: 'Rajasthan', pendingInspections: 20 },
	{ id: 'MP', state: 'Madhya Pradesh', pendingInspections: 18 },
	{ id: 'AP', state: 'Andhra Pradesh', pendingInspections: 16 },
	{ id: 'KL', state: 'Kerala', pendingInspections: 14 },
	{ id: 'PB', state: 'Punjab', pendingInspections: 12 },
	{ id: 'HR', state: 'Haryana', pendingInspections: 10 },
	{ id: 'BR', state: 'Bihar', pendingInspections: 8 },
	{ id: 'OR', state: 'Odisha', pendingInspections: 6 },
	{ id: 'AS', state: 'Assam', pendingInspections: 4 },
	{ id: 'JH', state: 'Jharkhand', pendingInspections: 3 },
	{ id: 'CT', state: 'Chhattisgarh', pendingInspections: 2 },
	{ id: 'HP', state: 'Himachal Pradesh', pendingInspections: 1 }
];

export const GET: RequestHandler = async ({ url }) => {
	try {
		// In a real application, you would fetch this data from your database
		// For now, we'll return sample data with some randomization to simulate real data
		const randomizedData = sampleHeatmapData.map(item => ({
			...item,
			pendingInspections: Math.max(0, item.pendingInspections + Math.floor(Math.random() * 10) - 5)
		}));

		// Sort by pending inspections (descending)
		randomizedData.sort((a, b) => b.pendingInspections - a.pendingInspections);

		return json({
			success: true,
			data: randomizedData,
			totalPending: randomizedData.reduce((sum, item) => sum + item.pendingInspections, 0),
			lastUpdated: new Date().toISOString()
		});
	} catch (error) {
		console.error('Error fetching heatmap data:', error);
		return json(
			{
				success: false,
				error: 'Failed to fetch heatmap data',
				data: sampleHeatmapData
			},
			{ status: 500 }
		);
	}
};
