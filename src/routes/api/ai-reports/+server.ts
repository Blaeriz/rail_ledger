import { json } from '@sveltejs/kit';
import { aiService, type AIReportData } from '$lib/services/aiService.js';

export async function POST({ request }) {
  try {
    const { type, batchId, vendorId } = await request.json();

    let report: AIReportData;

    switch (type) {
      case 'quality':
        report = await aiService.generateQualityAnalysisReport(batchId);
        break;
      case 'maintenance':
        report = await aiService.generatePredictiveMaintenanceReport(batchId);
        break;
      case 'vendor':
        report = await aiService.generateVendorPerformanceReport(vendorId);
        break;
      case 'safety':
        report = await aiService.generateSafetyRiskReport(batchId);
        break;
      default:
        return json({ error: 'Invalid report type' }, { status: 400 });
    }

    return json({ report });
  } catch (error) {
    console.error('Error generating AI report:', error);
    return json({ error: 'Failed to generate AI report' }, { status: 500 });
  }
}

export async function GET() {
  try {
    // Generate recent AI reports based on real data
    const qualityReport = await aiService.generateQualityAnalysisReport();
    const maintenanceReport = await aiService.generatePredictiveMaintenanceReport();
    const vendorReport = await aiService.generateVendorPerformanceReport();
    const safetyReport = await aiService.generateSafetyRiskReport();

    const reports = [qualityReport, maintenanceReport, vendorReport, safetyReport];

    return json({ reports });
  } catch (error) {
    console.error('Error fetching AI reports:', error);
    return json({ error: 'Failed to fetch AI reports' }, { status: 500 });
  }
}
