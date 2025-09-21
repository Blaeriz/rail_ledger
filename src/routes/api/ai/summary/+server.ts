import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// POST /api/ai/summary
// Body: { reports: Array<any>, context?: string }
// Returns: { summary: string, model: string }
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json().catch(() => ({}));
		const reports = Array.isArray(body?.reports) ? body.reports : [];
		const extraContext = typeof body?.context === 'string' ? body.context : '';

		if (!reports.length) {
			return new Response(JSON.stringify({ error: 'No reports provided to summarize' }), {
				status: 400
			});
		}

		// Accept common env names to be forgiving
		const apiKey = env.GEMINI_API_KEY || env.OPENAI_API_KEY || env.GOOGLE_API_KEY;
		const model = env.GEMINI_MODEL || env.OPENAI_MODEL || 'gemini-1.5-flash';

		if (!apiKey) {
			return new Response(
				JSON.stringify({
					error: 'Missing GEMINI_API_KEY (or OPENAI_API_KEY/GOOGLE_API_KEY) on server'
				}),
				{ status: 500 }
			);
		}

		// Compute grounded facts to avoid hallucinated counts
		const normStatus = (s: any) => {
			if (s === 1 || s === '1' || String(s).toUpperCase() === 'PASS') return 1;
			if (s === 0 || s === '0' || String(s).toUpperCase() === 'FAIL') return 0;
			return null;
		};
		const totalReports = reports.length;
		let passCount = 0;
		let failCount = 0;
		const issueBins: Record<string, RegExp[]> = {
			Packaging: [/packag/i, /carton/i, /seal/i, /strap/i],
			Labeling: [/label/i, /serial/i, /misalign/i],
			Rust: [/rust/i, /corrosion/i],
			Cracks: [/crack/i, /fracture/i],
			Moisture: [/moist/i, /wet/i, /damp/i]
		};
		const issueCounts: Record<string, number> = Object.fromEntries(Object.keys(issueBins).map(k => [k, 0]));

		for (const r of reports) {
			const s = normStatus((r as any).status);
			if (s === 1) passCount++;
			else if (s === 0) failCount++;
			const remark = String((r as any).remark ?? '');
			for (const [k, patterns] of Object.entries(issueBins)) {
				if (patterns.some(re => re.test(remark))) issueCounts[k]++;
			}
		}
		const passRate = totalReports ? Math.round((passCount / totalReports) * 100) : 0;
		const failRate = totalReports ? 100 - passRate : 0;

		// Keep payload compact to avoid token overrun (sample only for context)
		const compactReports = reports.slice(0, 50).map((r: any) => ({
			reportId: r.reportId ?? r.report_id ?? undefined,
			batchId: r.batchId ?? r.batch_id ?? undefined,
			inspectorName: r.inspectorName ?? r.inspector_name ?? undefined,
			status: r.status,
			remark: r.remark,
			createdAt: r.createdAt ?? r.created_at ?? undefined
		}));

		const systemPrompt = `You are a senior QA analyst for railway manufacturing. \nProvide an executive summary of inspection reports with:\n- Overall pass rate and failure rate.\n- Top failure reasons or patterns.\n- Notable vendors/batches if issues cluster.\n- Any urgent risks or trends.\nBe concise and action-oriented for management.`;

		const facts = {
			totalReports,
			passCount,
			failCount,
			passRate,
			failRate,
			issueCounts
		};

	const userPrompt = `Facts (authoritative, use ONLY these for numbers):\n${JSON.stringify(facts, null, 2)}\n\nSample report snippets (for color, NOT for counts): first ${compactReports.length} items\n${JSON.stringify(compactReports)}\n\n${extraContext ? `Additional context: ${extraContext}\n\n` : ''}Instructions:\n- Use only the provided facts for all metrics and counts.\n- Do not infer totals or sample sizes beyond 'facts.totalReports'.\n- If vendor/batch clustering cannot be determined from facts, avoid speculating.\n- Write concise markdown (≤180 words) with bullets and a short conclusion.`;


		const combinedText = `${systemPrompt}\n\n${userPrompt}`;

		const url = `https://generativelanguage.googleapis.com/v1beta/models/${model}:generateContent?key=${apiKey}`;
		const resp = await fetch(url, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				contents: [
					{
						role: 'user',
						parts: [{ text: combinedText }]
					}
				],
				generationConfig: { temperature: 0.3, maxOutputTokens: 500 }
			})
		});

		if (!resp.ok) {
			const errText = await resp.text();
			console.error('Gemini API error:', errText);
			return new Response(JSON.stringify({ error: 'AI service failed', details: errText }), {
				status: 502
			});
		}

		const data = await resp.json();
		const parts = data?.candidates?.[0]?.content?.parts;
		const content = Array.isArray(parts) && parts.length > 0 && parts[0]?.text ? parts[0].text : '';

		return new Response(JSON.stringify({ summary: content, model }), { status: 200 });
	} catch (err) {
		console.error('AI summary error:', err);
		return new Response(JSON.stringify({ error: 'Failed to generate AI summary' }), {
			status: 500
		});
	}
};
