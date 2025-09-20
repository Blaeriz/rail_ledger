import pkg from 'pg';
import { drizzle } from 'drizzle-orm/node-postgres';
const { Pool } = pkg;

export const pool = new Pool({
  user: process.env.DB_USER || 'neondb_owner',
  host: process.env.DB_HOST || 'ep-spring-poetry-a18e7nwj-pooler.ap-southeast-1.aws.neon.tech',
  database: process.env.DB_NAME || 'neondb',
  password: process.env.DB_PASS || 'npg_lxS5YBJREOf3',
  port: Number(process.env.DB_PORT) || 5432,
  ssl: { rejectUnauthorized: false },
});

export const db = drizzle(pool);

