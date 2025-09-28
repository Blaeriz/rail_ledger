// src/lib/eventLog.ts

export type ApiStatus = 'success' | 'error';
export type ApiMethod = 'GET' | 'POST';
export type ApiRoute = '/api/ai/summary' | '/api/batches' | '/api/users' | '/api/vendors';

interface ApiEvent {
  route: ApiRoute;
  status: ApiStatus;
  method: ApiMethod;
  time: number;
}

// Store all events temporarily
const apiEvents: ApiEvent[] = [];

/**
 * Logs an API event with route, status, and method (GET/POST)
 */
export function logEvent(route: ApiRoute, status: ApiStatus, method: ApiMethod = 'POST') {
  apiEvents.push({ route, status, method, time: Date.now() });
  cleanupOldEvents(60); // remove events older than 60 minutes
}

/**
 * Converts Date.now() timestamp to custom format: DDMMYYHHMM
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
 * Returns events grouped by route, method, and timestamps
 */
export function getEventLog() {
  const log: Record<string, { method: ApiMethod; timestamps: string[] }> = {};

  for (const event of apiEvents) {
    if (!log[event.route]) log[event.route] = { method: event.method, timestamps: [] };
    log[event.route].timestamps.push(formatTimestamp(event.time));
  }

  return log;
}

/**
 * Count events for a route and optional status in last X minutes
 */
export function countEvents(route: ApiRoute, minutes: number, status?: ApiStatus) {
  const cutoff = Date.now() - minutes * 60_000;
  return apiEvents.filter(
    (e) =>
      e.route === route &&
      e.time >= cutoff &&
      (!status || e.status === status)
  ).length;
}

/**
 * Remove events older than `maxMinutes` to prevent memory bloat
 */
function cleanupOldEvents(maxMinutes: number) {
  const cutoff = Date.now() - maxMinutes * 60_000;
  while (apiEvents.length && apiEvents[0].time < cutoff) {
    apiEvents.shift();
  }
}

/**
 * Optional: Get all events (for debugging or UI)
 */
export function getAllEvents(): ApiEvent[] {
  return [...apiEvents];
}
