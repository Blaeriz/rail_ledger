# Rail Ledger TUI Plan

This document outlines the initial plan before implementation.

## Goals
- Build a read-only TUI for system admins to view operational insights and records.
- Use existing REST APIs from the SvelteKit app.
- Fast, keyboard-first navigation.

## Non-goals (Phase 1)
- No create/update/delete actions.
- No authentication flow unless required by APIs.
- No persistence beyond in-memory cache.

## Architecture
- Language: Rust
- Libraries: ratatui (UI), crossterm (terminal), reqwest (HTTP), serde (JSON), tokio (async), anyhow/thiserror (errors), serde_json, url
- App layers:
  - config: read API base url, refresh interval
  - models: Rust structs matching API payloads
  - api: reqwest client, typed functions (list/detail)
  - ui: screens and widgets per tab
  - app: state machine (current tab, filters, data cache)

## Screens & Data
- Overview
  - Pull vendors, batches, reports
  - Compute: totalVendors, totalBatches, pendingInspections, failures
  - Recent reports list (5-10 items)
- Batches
  - List: id, vendor_id, qc_status, production/expiry dates
  - Filters: status, vendor, search by id
  - Detail: fitment, last_inspection_date, location, qr_hash
- Vendors
  - List: id, city/state, email/phone, no_of_batches, audit_date
  - Detail: derived metrics from batches (pass/fail counts)
- Reports
  - List: reportId, batchId, inspectorName, status, createdAt
  - Detail: remark, metadata
- Users
  - List + detail (basic info)
- Tickets
  - List + detail (from mock API)
- AI Insights
  - GET /api/ai-reports for quality/maintenance/vendor/safety
  - Show summary/insights/recommendations
- QR Tools
  - Input qr_hash, GET /api/batches/qr_hash
- System
  - Show base URL, refresh interval, version

## Keyboard
- Global: q to quit, Tab/Shift-Tab to move tabs, g+key to jump, / to search, Esc to clear
- Lists: arrows/j/k to move, PageUp/PageDown, Enter for detail, f to open filter menu

## Networking
- Base URL from env: RAIL_LEDGER_API_BASE (default http://localhost:5173)
- Timeouts, retries; refresh = 30s default

## Error Handling
- Non-blocking errors rendered in a status bar and logs

## Milestones
1. Scaffold project + compile a static tabs UI
2. Add models and API client; wire Batches list (happy path)
3. Add Vendors, Reports; Overview stats
4. Add Users, Tickets
5. Add AI Insights, QR lookup
6. Polish: filters, pagination, status bar, config
