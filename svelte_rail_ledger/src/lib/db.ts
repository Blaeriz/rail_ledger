import pkg from 'pg';
import { drizzle } from 'drizzle-orm/node-postgres';
const { Pool } = pkg;

// Hardcoded database IP
const DB_HOST = 'localhost'; 

export const pool = new Pool({
  user: process.env.DB_USER || 'archit',
  host: DB_HOST,
  database: process.env.DB_NAME || 'rail_ledger',
  password: process.env.DB_PASS || '1234',
  port: Number(process.env.DB_PORT) || 5432,
});

// Drizzle ORM client
export const db = drizzle(pool);

console.log('✅ Connected to Postgres via Drizzle');
