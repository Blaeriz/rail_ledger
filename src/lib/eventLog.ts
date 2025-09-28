// src/lib/eventLog.ts

export type ApiStatus = 'success' | 'error';
export type ApiMethod = 'GET' | 'POST';
export type ApiRoute = '/api/ai/summary' | '/api/batches' | '/api/users' | '/api/vendors';

interface ApiEvent {
  route: ApiRoute;
  method: ApiMethod;
  status: ApiStatus;
  time: number;
}

// Temporary in-memory store
const apiEvents: ApiEvent[] = [];

/**
 * Logs an API event
 * Backward-compatible:
 * 1. logEvent(route, status) → defaults method to POST
 * 2. logEvent(route, method, status)
 */
export function logEvent(route: ApiRoute, arg2: ApiMethod | ApiStatus, arg3?: ApiStatus) {
  let method: ApiMethod;
  let status: ApiStatus;

  if (arg3 === undefined) {
    method = 'POST';
    status = arg2 as ApiStatus;
  } else {
    method = arg2 as ApiMethod;
    status = arg3;
  }

  // Only track GET and POST
  if (method === 'GET' || method === 'POST') {
    apiEvents.push({ route, method, status, time: Date.now() });
    cleanupOldEvents(60); // remove events older than 60 minutes
  }
}

/**
 * Converts timestamp to DDMMYYHHMM
 */
function formatTimestamp(ts: number) {
  const d = new Date(ts);
  const day = String(d.getDate()).padStart(2, '0');
  const month = String(d.getMonth() + 1).padStart(2, '0');
  const year = String(d.getFullYear()).slice(-2);
  const hours = String(d.getHours()).padStart(2, '0');
  const minutes = String(d.getMinutes()).padStart(2, '0');
  return `${day}${month}${year}${hours}${minutes}`;
}

/**
 * Returns events grouped by route and method
 * Only GET and POST methods are included
 */
export function getEventLog() {
  const log: Record<string, Record<ApiMethod, string[]>> = {};

  const routes: ApiRoute[] = ['/api/ai/summary', '/api/batches', '/api/users', '/api/vendors'];
  for (const route of routes) {
    log[route] = { GET: [], POST: [] };
  }

  for (const event of apiEvents) {
    log[event.route][event.method].push(formatTimestamp(event.time));
  }

  return log;
}

/**
 * Removes events older than maxMinutes
 */
function cleanupOldEvents(maxMinutes: number) {
  const cutoff = Date.now() - maxMinutes * 60_000;
  while (apiEvents.length && apiEvents[0].time < cutoff) {
    apiEvents.shift();
  }
}
