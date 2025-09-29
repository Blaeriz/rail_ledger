# Rail Ledger — Unified QR + AI system for Track Fittings (Detailed)

This document captures the end‑to‑end design of our Smart India Hackathon 2025 solution for identifying, tracing, and assuring quality of Indian Railways’ track fittings (Elastic Rail Clips, Rail Pads, Liners) and sleepers. It combines laser‑etched, tamper‑evident QR codes; secure cryptography; live operational dashboards; analytics (with optional AI prototypes isolated in `ai/`); and planned integration with UDM and TMS.

The content reflects what is implemented in this repo plus tightly scoped plans where integration or hardware wiring is not yet committed to source.


## 1) Problem we solve

- 10+ crore ERCs, 5+ crore liners, and 8.5+ crore pads are procured annually; there’s no unified identity at the item/batch level connected to UDM/TMS.
- Quality and performance exceptions are difficult to localize across lifecycle phases: manufacturing → inspection → supply → fitment → in‑service.
- No single pane of glass for inventory, inspection SLAs, vendor performance, defects and safety risks.

Outcome: A secure identity and analytics layer so every fitting is verifiable on‑site with a phone, and every stakeholder gets timely, actionable insight.


## 2) What the system does (high‑level)

- Hardware: Laser‑etch durable QR code on each fitting (or on sleeper top surface) encoding a compact signed payload.
- Security: HMAC‑SHA256 signature (truncated) over canonical fields; server verifies on scan; revocation and fraud detection rules.
- Mobile/Web Scan: Inspector or store clerk scans QR (Flutter app or PWA); the app verifies signature offline and enriches online (warranty, inspection history, inventory state, location).
- Data Platform: SvelteKit app with Postgres + Drizzle ORM, typed APIs and role‑based dashboards for Admin, Inspector, Viewer.
- Analytics and AI prototypes: Decision support for quality, predictive maintenance, vendor performance, and safety. All AI/ML experimentation is confined to the `ai/` directory (Streamlit), separate from shipped app APIs.
- Observability: Request counters, latency histograms, IP request tracking, and live API activity feed for ops/abuse monitoring.
- TUI: A read‑only Rust terminal UI for control rooms to monitor state from the same APIs.


## 3) Architecture (repo‑backed)

- Frontend + API: SvelteKit (TypeScript, Tailwind). Key modules:
  - `src/routes/api/*` → RESTful endpoints for users, vendors, batches, reports, metrics.
  - `ai/` → Streamlit analysis app and experiments (not exposed via app APIs).
  - `src/lib/eventLog.ts` + `src/routes/api/metrics/live` → lightweight, minute‑window API activity tracking (GET/POST counts per route).
  - `src/lib/metrics.ts` + `src/lib/hooks.server.ts` → prom‑style counters and histograms per request, IP request counter.
- Database: PostgreSQL (local via Docker or managed like Neon). Schema in `src/lib/server/db/schema.ts`:
  - `user_info(user_id, aadhar, phone_number, name, user_role)`
  - `vendor_info(vendor_id, no_of_batches, gst_no, pan_number, city, state, phone_number, email, audit_date)`
  - `batch_info(batch_id, vendor_id, batch_size, date_of_production, qc_status, expiry_date, last_inspection_date, fitment_date, fitment_location, qr_hash)`
  - `inspection_reports(report_id, batch_id, inspector_name, remark, status, created_at)`
- Data seeding: `seed-data.sql` provides realistic sample data for demos.
- AI/analytics demo app: `ai/x.py` (Streamlit) performs geospatial risk clustering, expiry watch, staffing gaps, and dispatch recommender using CSVs. This is a prototype and is not wired as application APIs.
- TUI: `tui` (Rust/ratatui) planned screens for Overview, Batches, Vendors, Reports, Users, Tickets, AI Insights, QR Tools.

### 3.1 Target architecture at scale

At scale, we separate concerns across five layers while keeping interfaces simple and auditable:

- Clients
  - Vendor app for batch creation and QR provisioning
  - Inspector app for installation and inspections
  - Public app for quick scans with minimal disclosure
  - Railway web dashboard for analytics
  - Admin TUI for real-time infrastructure health

- API gateway + auth
  - OAuth2/JWT and API keys where needed
  - RBAC for vendor/inspector/admin/public roles
  - Rate limiting, validation, and request logging

- Backend services
  - Identity: UID/QR generation, signing, verification
  - Vendor: master data and batch manifests
  - Inventory: batches, stock, shipments, lifecycle state
  - Installation: fitment events with GPS/section metadata
  - Inspection: scheduling and results capture
  - Public scan: low-latency minimal lookups
  - Analytics: aggregations and anomaly detection (AI optional)
  - Integrations: UDM/TMS synchronization

- Eventing
  - Kafka (or similar) to ingest install/inspect events, enable async processors and retries, and decouple mobile writes from DB writes

- Data layer
  - PostgreSQL → Citus for relational scale-out; ClickHouse for OLAP
  - Object storage (S3) for images and certificates; WORM audit log for compliance

- Observability
  - Prometheus/Grafana for metrics, ELK/OpenSearch for logs, Jaeger/Tempo for tracing, plus the Admin TUI as a CLI view

MVP → target mapping (what we have vs where it lands):
- API layer: SvelteKit endpoints today → fronted by API Gateway with OAuth2/JWT and rate limits.
- Services: Monolith modules (users/vendors/batches/reports) → split into Vendor/Inventory/Inspection/Public Scan services.
- Identity/QR: Current QR signing/verification logic → dedicated Identity Service; vendor app uses secure provisioning APIs.
- Analytics/AI: Prototype in `ai/` today → scales into Analytics service; AI remains optional and off the API path.
- Eventing: Direct DB writes today → future Kafka topic for install/inspect events with async processors.
- Data: Postgres today → scale to Citus for horizontal partitioning; add ClickHouse for OLAP; S3 for media; WORM audit trail.
- Observability: prom-client + event log today → Prometheus/Grafana, ELK, and tracing.


## 4) QR data, crypto and engraving standard

- Purpose: Make every field audit‑ready and fraud‑resistant while remaining laser‑etchable and scannable on small surfaces.
  - Full normative spec with canonicalization rules, test vectors, and SVG guidance: see `sih_submission/qr_spec.md`.
- Data fields (canonical order for signing):
  - `v`: format version (1 byte)
  - `t`: item type (ERC|PAD|LINER|SLEEPER → 2–3 byte code)
  - `vid`: vendor_id (hash‑reduced or code)
  - `bid`: batch_id (short UUID or sequence)
  - `dp`: date_of_production (YYMMDD)
  - `bs`: batch_size (optional, 2–3 bytes if encoded)
  - `n`: serial (required per‑piece; unique within batch)
- Integrity: `sig = Trunc(HMAC‑SHA256(secret_k, canonical_bytes), L)`
  - We use truncated HMAC‑SHA256 with L = 12 bytes (96 bits), balancing security and engravable area on ERCs.
  - Secret rotation: `kid` (key id) encoded in payload so server can verify with proper secret.
  - Encoding: Base32 Crockford or Base58 to avoid visually ambiguous characters; QR ECC level M/H depending on etching area.
- Payload layout examples:
  - Compact TLV or URL‑safe querystring. Example (Base32):
    - `RL1:PAD:V2:B48:250101:S8:K1:7ZX4H8F3Q9D5` where the last token is the truncated MAC.
  - For vendor laser controllers, we output both SVG and raw string; controller renders QR with specified ECC and quiet zone.

### 4.1 QR provisioning via Vendor Application (out of repo)

- A separate Vendor Application (not included in this repository) integrates with our secure QR Provisioning APIs.
- Flow:
  1) Vendor operator authenticates in the Vendor App and requests QR(s) for a batch or per‑piece range providing: `vendor_id`, `batch_id`, `item_type`, `date_of_production`, and a required serial range (per‑piece mode).
  2) Our API computes the canonical payload and HMAC‑SHA256 (truncated), assigns/validates `kid`, and persists or returns the `qr_hash`.
  3) Response includes:
     - Canonical payload string (for audit),
     - `qr_hash` (DB/back‑reference),
     - An SVG QR (ready for laser engravers) and recommended ECC/size settings.
  4) The Vendor App downloads the `.svg` and feeds it directly to the laser engraver software, or embeds into their controller workflow.
- Safety: AuthN/AuthZ per vendor, rate‑limits, audit logs, and key rotation (`kid`) enforced in the provisioning API. No secrets are exposed to vendors; signing happens server‑side.
- On‑device verification:
  - The mobile app carries the signing key set as a JWKS‑like bundle with `kid` map for offline verification.
  - If offline, app shows “cryptographically valid” with cached vendor/batch meta; online adds warranty/inspection from server.
- Server‑side anti‑fraud:
  - Repeated scans with large geo separation in short windows flag as potential counterfeit.
  - Batches marked as `qc_status != Pass` but scanned at fitment raise “QC leakage”.
  - Rate‑limit by IP, device, and qr_hash; persist anomalies for audit.


## 5) Core user journeys

1) Manufacture and Engrave
- Vendor receives batch_id and QR payload from Rail Ledger.
- Controller engraves QR; vendor ships lot and uploads ASN to UDM.

2) Receipt and Inspection
- Store clerk/inspector scans item; app verifies signature offline; server attaches vendor, warranty, and pending inspections.
- Inspection report is added via `/api/reports` with status + remark. Optional AI summaries are generated only within the `ai/` prototype for analysis.

3) Fitment and In‑service Monitoring
- On fitment scan, location is recorded; later scans compare for movement/counterfeit detection.
- SLA alerts for overdue inspection; predictive maintenance flags high‑risk items based on risk score.


## 6) Implemented features in this repo

- CRUD APIs: `/api/vendors`, `/api/users`, `/api/batches`, `/api/reports` with Drizzle over Postgres.
- AI endpoints: Not exposed in the app scope for this submission. All AI work is limited to the `ai/` directory (Streamlit prototypes).
- Dashboards (Svelte): Admin/Inspector/Viewer pages with tables, charts, India map, modals.
- Observability:
  - `prom-client` counters/histograms plus request timing in `hooks.server.ts`.
  - `eventLog` and `/api/metrics/live` expose sliding‑window timestamps grouped by route and method (GET/POST).
- TUI (Rust) skeleton with tabs and API plan; compiles and runs.
- AI/Analytics (Python Streamlit) notebook‑style analytics over CSVs: clustering, risk tiering, expiry watch, dispatch recommender (in `ai/`).


## 7) Integration with UDM and TMS (planned — post‑MVP)

This will be addressed after MVP once official interfaces are clarified. We will not scrape portals; instead, integrate via official APIs or sanctioned CSV gateways when available. Roadmap stance:

- Define target contracts for:
  - UDM (ireps.gov.in): ASN/manifests push and GRN/depot data pull
  - TMS (irecept.gov.in): fitment and inspection sync per track section
- Until specs are provided, keep import/export shims and internal abstractions to swap in official clients without breaking app code.

Until official APIs are available, we keep import/export shims:
- Export CSV/JSON from `/api/*` to a predefined spec.
- Import job to upsert vendor/batch/inspection data into our Postgres.


## 8) Analytics and AI approach (prototype)

- Any AI/ML usage is constrained to the Streamlit prototype. It explores hotspot clustering (DBSCAN), expiry/inspection SLA, staffing gaps, dispatch recommender, and summarization experiments on local data. Production APIs do not depend on AI.


## 9) Security, privacy, and safety

- QR signing: HMAC‑SHA256 truncated; include `kid` for rotation and allow per‑vendor secrets; verify client+server side.
- PII: User records include Aadhaar and phone; mask in UIs and logs; protect at rest with column‑level encryption if required.
- API hardening: Rate‑limit by IP/device; audit logs via `eventLog`; expose Prom metrics for WAF/IDS correlation.
- Secrets: Move hard‑coded DB and AI API keys to environment variables before production. `.env`/secrets manager to be used.


## 10) Deployment and ops (current and recommended)

- Local: `docker-compose.yml` starts Postgres; Drizzle migrations via `drizzle-kit`; SvelteKit dev via Vite.
- Cloud: Postgres (Neon/RDS), SvelteKit on Node server or serverless adapter; Prometheus scraping of metrics (add `/metrics` endpoint wiring to `register.metrics()`).
- Artifact interfaces for engraving: Export SVG/PNG + payload CSV for laser machines; minimal REST hook for engraving triggers. Vendors may request alternative formats; outputs are scalable by size to match engraver capabilities.


## 11) Roadmap (near‑term)

- [ ] Finalize QR payload spec (exact fields, encoding, truncation length L, and `kid` policy) and publish engraving SOP.
- [ ] Mobile scanner: Flutter app with offline verify + online enrichment; enroll devices and sign scan events.
- [ ] UDM/TMS adapters with retry + reconciliation.
- [ ] Counterfeit detection rules surfaced in dashboard; QR hash lookup APIs (`/api/*/qr_hash`).
- [ ] Prometheus `/metrics` endpoint and Grafana dashboards; anomaly alerts.
- [ ] Replace hard‑coded DB/API secrets with envs across code; add auth (JWT/OIDC) and RBAC enforcement in APIs.
- [ ] Expand schema for per‑piece IDs if required (today is batch‑centric).


## 12) Quick reference of APIs and schema

- Users: `GET/POST /api/users`, `GET/PUT/DELETE /api/users/[id]` (pattern used for others)
- Vendors: `GET/POST /api/vendors`, `GET/PUT/DELETE /api/vendors/[id]`, lookup by `qr_hash` (route scaffolded)
- Batches: `GET/POST /api/batches`, `GET/PUT/DELETE /api/batches/[id]`, lookup by `qr_hash`
- Reports: `GET/POST /api/reports`
- (No AI endpoints exposed.)
- Metrics: `GET /api/metrics/live?minutes=5`

Database tables: see Section 3 and `src/lib/server/db/schema.ts`.

## 13) Decisions (locked for MVP)

1) HMAC truncation length: 12 bytes (96 bits). Chosen to fit ERC etching constraints while maintaining strong integrity.
2) Identity granularity: Per‑piece. Every ERC/liner/pad carries a unique serial within its batch; QR encodes this serial.
3) Secrets strategy: Per‑vendor keys derived from a platform master via HKDF; rotate vendor keys quarterly and encode a `kid` in payloads. This balances compromise blast radius and operational simplicity. Server‑side signing only; keys never leave control plane.
4) Mobile stack: Flutter only for field apps (offline-first). No SvelteKit PWA for phones.
5) UDM/TMS: Treated as post‑MVP. We will define adapters when official specs are available; until then, rely on import/export shims.
6) Warranty policy: Warranty starts at manufacture date; remaining coverage computed as `expiry_date - now` with fitment for context only.
7) Laser engraver outputs: Vendor provisioning API returns scalable SVG by default and can also supply PNG and CSV payloads; engraver format is vendor-selectable.
8) Geo capture: Inspector/fitment scans automatically attach device geolocation (with OS prompts). Precision is kept to what the OS provides; retention follows policy once finalized.

See `sih_submission/qr_spec.md` for the authoritative QR payload, signing, and SVG output specification.
