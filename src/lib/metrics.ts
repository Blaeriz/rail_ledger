// src/lib/metrics.ts
import {
  Counter,
  Histogram,
  collectDefaultMetrics,
  Registry
} from 'prom-client';

const register = new Registry();

// ✅ Collect default Node.js metrics
collectDefaultMetrics({ register });

// ✅ Count total API requests (success + errors)
export const apiCallCounter = new Counter({
  name: 'api_request_count',
  help: 'Total number of API requests',
  labelNames: ['method', 'route', 'statusCode', 'type'] as const,
});
register.registerMetric(apiCallCounter);

// ✅ Track request duration
export const apiLatencyHistogram = new Histogram({
  name: 'api_request_duration_seconds',
  help: 'Duration of API requests in seconds',
  labelNames: ['method', 'route'] as const,
});
register.registerMetric(apiLatencyHistogram);

// ✅ Track requests per IP (for attack monitoring)
export const ipRequestCounter = new Counter({
  name: 'ip_request_count',
  help: 'Number of requests by client IP',
  labelNames: ['ip'] as const,
});
register.registerMetric(ipRequestCounter);

export { register };
