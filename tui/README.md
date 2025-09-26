# Rail Ledger TUI

Purpose: Provide a read-only terminal user interface (TUI) for System Admins to view system insights and records exposed by existing APIs.

## Scope (Phase 1)

- Read-only views only (no create/update/delete)
- Tabs/screens:
  - Overview (stats + recent activity)
  - Batches (list + detail)
  - Vendors (list + detail)
  - Reports (inspection reports list + detail)
  - Users (list + detail)
  - Tickets (mock data list + detail)
  - AI Insights (summary + prebuilt reports)
  - QR Tools (lookup by qr_hash)
  - System (app info)
- Keyboard navigation and filters
- Light caching + periodic refresh

## API Endpoints Used

- Users: `/api/users`, `/api/users/[id]`
- Vendors: `/api/vendors`, `/api/vendors/[id]`
- Batches: `/api/batches`, `/api/batches/[id]`, `/api/batches/qr_hash?qr_hash=...`
- Reports: `/api/reports`, `/api/reports/[id]`
- Tickets: `/api/tickets`, `/api/tickets/[id]`
- AI: `/api/ai/summary`, `/api/ai-reports`, `/api/summary`

## MVP Deliverables

- Compiling ratatui app with tabs and placeholder tables
- HTTP client + typed models
- List views for Batches, Vendors, Reports with pagination
- Detail panels for each entity
- Config (base URL, refresh interval) via env vars

## Stretch (later)

- Export to CSV
- Search across entities
- ASCII sparkline charts
- Auth (if/when required by API)

## How to run

- Requirements: Rust (stable), internet access to API base
- Optional env vars:
  - `RAIL_LEDGER_API_BASE` (default: http://localhost:5173) – set once we wire the API client
  - `REFRESH_INTERVAL_MS` (default: 30000)

Build and run:

```bash
cd tui
cargo run --release
```

Controls:

- q to quit
- Tab / Shift-Tab to switch tabs
- More keys will be added as we implement lists and filters
