import { defineConfig } from 'drizzle-kit';
import 'dotenv/config'; // make sure env vars load from .env

export default defineConfig({
  schema: './src/lib/server/db/schema.ts',
  dialect: 'postgresql',
  dbCredentials: {
    url: process.env.DATABASE_URL!, // 👈 only use env, don’t hardcode
  },
  verbose: true,
  strict: true,
});
