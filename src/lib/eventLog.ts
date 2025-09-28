// src/lib/eventLog.ts

export type ApiStatus = 'success' | 'error';
export type ApiRoute = '/api/ai/summary' | '/api/batches' | '/api/users' | '/api/vendors';

interface ApiEvent {
  route: ApiRoute;
  status: ApiStatus;
  time: number;
}

// Store all events temporarily
const apiEvents: ApiEvent[] = [];

/**
 * Logs an API event
 */
export function logEvent(route: ApiRoute, status: ApiStatus) {
  apiEvents.push({ route, status, time: Date.now() });
  cleanupOldEvents(60); // remove events older than 60 minutes
}

/**
 * Converts Date.now() timestamp to your custom format: DDMMYYHHMM
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
  const log: Record<string, { method: 'POST'; timestamps: string[] }> = {};

  for (const event of apiEvents) {
    if (!log[event.route]) log[event.route] = { method: 'POST', timestamps: [] };
    log[event.route].timestamps.push(formatTimestamp(event.time));
  }

  return log;
}

/**
 * Remove events older than `maxMinutes`
 */
function cleanupOldEvents(maxMinutes: number) {
  const cutoff = Date.now() - maxMinutes * 60_000;
  while (apiEvents.length && apiEvents[0].time < cutoff) {
    apiEvents.shift();
  }
}
