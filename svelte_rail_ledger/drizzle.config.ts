import { defineConfig } from 'drizzle-kit';

export default defineConfig({
  schema: './src/lib/server/db/schema.ts',
  dialect: 'postgresql',
  dbCredentials: {
    url: process.env.DATABASE_URL || 'postgres://archit:1234@10.12.41.5:5432/rail_ledger', // hardcoded DB
  },
  verbose: true,
  strict: true,
});
