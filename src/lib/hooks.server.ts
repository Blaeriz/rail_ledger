// src/hooks.server.ts
import type { Handle } from '@sveltejs/kit';
import { apiCallCounter, apiLatencyHistogram, ipRequestCounter } from '$lib/metrics';

export const handle: Handle = async ({ event, resolve }) => {
  const start = Date.now();
  const route = event.url.pathname;
  const method = event.request.method;

  // ✅ Track IP
  const ip =
    (event.request.headers.get('x-forwarded-for') as string) ||
    event.getClientAddress() ||
    'unknown';
  ipRequestCounter.inc({ ip });

  // ✅ Proceed with the request
  const response = await resolve(event);
  const duration = (Date.now() - start) / 1000;
  const statusCode = response.status;
  const isError = statusCode >= 400;

  apiCallCounter.inc({
    method,
    route,
    statusCode: statusCode.toString(),
    type: isError ? 'error' : 'success'
  });

  apiLatencyHistogram.observe({ method, route }, duration);

  return response;
};
