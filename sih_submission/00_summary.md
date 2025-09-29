# Rail Ledger — Smart India Hackathon 2025 Summary

Rail Ledger is a unified identification and intelligence system for Indian Railways' track fittings and sleepers. We laser-etch secure QR codes on each item (or batch) and connect scans to a modern data stack that powers quality, inventory, and safety workflows across manufacturing → supply → fitment → in-service.

Authoritative QR spec (payload, signing, canonicalization, SVG guidance): see `sih_submission/qr_spec.md`.

## What it enables

- Secure identity: Compact payload signed with HMAC-SHA256 (truncated) prevents tampering/counterfeits. Works offline for field verification.
- Scan-to-insight: Mobile/PWA scan shows vendor, warranty, inspections, fitment history, and flags exceptions instantly.
- End-to-end traceability: Batch-to-location lineage, QC status, and inspection SLAs tied to UDM/TMS records.
- Actionable analytics: Quality, predictive maintenance, vendor performance, and safety risk insights. Note: all AI/ML experimentation lives in the `ai/` directory (Streamlit prototypes) and is not served via app APIs.
- Operations visibility: Live metrics, request rates, and event timelines; Rust TUI for control rooms; export/import shims for system integration.

## Built today (in repo)

- SvelteKit app with Postgres + Drizzle ORM; APIs for users, vendors, batches, reports.
- AI/ML prototypes confined to `ai/` (Streamlit). No AI endpoints are exposed in the app scope for this submission.
- Admin/Inspector/Viewer dashboards with charts, tables, and India map.
- Observability: Prom-style metrics and a live API activity feed.
- Streamlit analytics prototype (under `ai/`) and Rust TUI skeleton using the same APIs.

## Why it matters

- Safety: Detects QC leakage and high-risk assets before they fail in service.
- Quality: Vendor benchmarking and exception tracking drive accountability.
- Efficiency: Faster receipt and inspections; fewer manual reconciliations; better maintenance planning.

## Next integrations

- UDM (ireps.gov.in): batch manifests and receipts; vendor lot numbers linked to QR.
- TMS (irecept.gov.in): fitment and inspection sync per track section; maintenance windows.
- Mobile scanner app (Flutter) with offline verify and online enrichment.

Vendor QR provisioning (out of repo): A dedicated vendor application will call our secure QR provisioning APIs to obtain pre‑signed payloads and downloadable `.svg` QR images suitable for direct input to laser engravers.

## Open choices to finalize

- Truncation length for HMAC (12 or 16 bytes), per-batch vs per-piece QR, secret rotation policy, and engraving controller protocol.

In short, Rail Ledger brings cryptographic identity and analytics-driven insight to every fitting—scannable, verifiable, and actionable in the field.
