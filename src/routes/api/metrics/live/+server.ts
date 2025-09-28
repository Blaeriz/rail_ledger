// src/routes/api/metrics/live/+server.ts
import type { RequestHandler } from '@sveltejs/kit';
import { getEventLog } from '$lib/eventLog';

export const GET: RequestHandler = async ({ url }) => {
  const minutes = Number(url.searchParams.get('minutes') ?? 5);
  const cutoff = Date.now() - minutes * 60_000;

  const fullLog = getEventLog();

  const filteredLog: Record<
    string,
    { method: 'GET' | 'POST'; timestamps: string[] }
  > = {};

  for (const route in fullLog) {
    const { method, timestamps } = fullLog[route];

    const filteredTimestamps = timestamps.filter(ts => {
      // Convert DDMMYYHHMM back to Date
      const day = Number(ts.slice(0, 2));
      const month = Number(ts.slice(2, 4)) - 1; // JS months 0-11
      const year = 2000 + Number(ts.slice(4, 6));
      const hour = Number(ts.slice(6, 8));
      const minute = Number(ts.slice(8, 10));

      const tsDate = new Date(year, month, day, hour, minute).getTime();
      return tsDate >= cutoff;
    });

    if (filteredTimestamps.length > 0) {
      filteredLog[route] = { method, timestamps: filteredTimestamps };
    }
  }

  return new Response(JSON.stringify(filteredLog), {
    headers: { 'Content-Type': 'application/json' }
  });
};
