// src/routes/api/metrics/live/+server.ts
import type { RequestHandler } from '@sveltejs/kit';
import { getEventLog } from '$lib/eventLog';

export const GET: RequestHandler = async ({ url }) => {
  const minutes = Number(url.searchParams.get('minutes') ?? 5);
  const cutoff = Date.now() - minutes * 60_000;

  // Get the grouped log
  const fullLog = getEventLog();

  // Filter timestamps for GET and POST only
  const filteredLog: Record<string, { GET: string[]; POST: string[] }> = {};

  for (const route in fullLog) {
    const methods = fullLog[route];

    filteredLog[route] = { GET: [], POST: [] };

    // Only handle GET and POST
    ['GET', 'POST'].forEach((method) => {
      const timestamps = methods[method as 'GET' | 'POST'];
      const filteredTimestamps = timestamps.filter(ts => {
        const day = Number(ts.slice(0, 2));
        const month = Number(ts.slice(2, 4)) - 1;
        const year = 2000 + Number(ts.slice(4, 6));
        const hour = Number(ts.slice(6, 8));
        const minute = Number(ts.slice(8, 10));
        const tsDate = new Date(year, month, day, hour, minute).getTime();
        return tsDate >= cutoff;
      });
      filteredLog[route][method as 'GET' | 'POST'] = filteredTimestamps;
    });
  }

  return new Response(JSON.stringify(filteredLog), {
    headers: { 'Content-Type': 'application/json' }
  });
};
