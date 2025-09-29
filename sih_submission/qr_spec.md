# QR Specification (Rail Ledger)

Authoritative specification for generating, signing, encoding, and delivering QR codes for track fittings and sleepers.

This spec reflects the locked MVP decisions:
- HMAC truncation: 12 bytes (96 bits)
- Identity granularity: per‑piece (every ERC/liner/pad has a serial `n` unique within its batch)
- Secrets: per‑vendor keys derived via HKDF from a platform master; quarterly rotation; payload includes `kid`
- Mobile stack: Flutter apps for field scanning; no PWA for phones
- Warranty start: manufacture date
- Vendor engraving outputs: scalable SVG by default; PNG and CSV payloads available on request


## 1. Data model (signed payload)

Canonical field order (all fields are ASCII, uppercase tokens; no spaces):
1. `v`: QR format version. Fixed: `1`
2. `t`: item type code. One of: `ERC`, `PAD`, `LINER`, `SLEEPER`
3. `vid`: vendor_id short code (see 2.1)
4. `bid`: batch_id short code (see 2.2)
5. `dp`: date_of_production as `YYMMDD`
6. `n`: per‑piece serial, zero‑padded to width 6 (e.g., `000123`)
7. `kid`: key identifier for vendor key rotation (e.g., `K1`, `2025Q1`)

Signature field (not part of the canonical bytes):
8. `sig`: HMAC‑SHA256(payload_bytes, vendor_key)[0:12] (12‑byte truncation). Encoded as Base32 Crockford (uppercase, no padding) in the QR text.

### 1.1 Canonical string format

Compact token sequence with separators to reduce length and parsing ambiguity:

```
RL1:<T>:V<VID>:B<BID>:<DP>:N<N6>:K<KID>:<SIG>
```

Where:
- `RL1` → literal prefix for Rail Ledger format v1
- `<T>` ∈ {`ERC`,`PAD`,`LINER`,`SLEEPER`}
- `<VID>` → vendor short code (Base32Crockford, 6 chars)
- `<BID>` → batch short code (Base32Crockford, 6 chars)
- `<DP>` → `YYMMDD`
- `<N6>` → 6‑digit serial
- `<KID>` → key id (opaque string ≤ 8 chars, uppercase A–Z0–9, dash allowed)
- `<SIG>` → Base32Crockford of 12‑byte HMAC

Example (illustrative):
```
RL1:ERC:V3M9QX:B2K7PH:250101:N000123:K2025Q1:7ZX4H8F3Q9D5
```
Note: the signature shown above is an example placeholder.


## 2. Short code derivation

To keep QR payload short and visually robust, full identifiers are reduced to short codes.

### 2.1 Vendor short code `VID`
- Input: `vendor_id` (string)
- Code: `VID = Base32Crockford(SHA256(vendor_id))[0:6]`

### 2.2 Batch short code `BID`
- Input: `batch_id` (string)
- Code: `BID = Base32Crockford(SHA256(batch_id))[0:6]`

Rationale: short, collision‑resistant locally (collisions improbable; server verification uses full ids). If a collision is detected during provisioning, the server re‑issues with an alternate shorting strategy or increases length.


## 3. Signing

### 3.1 Key hierarchy
- Platform master secret `K_master` kept in HSM/secret manager.
- Per‑vendor key derived with HKDF (SHA‑256):
  - `K_vendor = HKDF_SHA256(salt = vendor_id, ikm = K_master, info = "rail-ledger-vendor", L = 32)`
- Quarterly rotation:
  - `K_vendor,q = HKDF_SHA256(salt = vendor_id || quarter_tag, ikm = K_vendor, info = "rail-ledger-rot", L = 32)`
  - `kid = quarter_tag` (e.g., `2025Q1`)

Keys are server‑side only. The Vendor App never sees secrets.

### 3.2 Canonical bytes
- Construct the canonical string WITHOUT the signature segment: `RL1:<T>:V<VID>:B<BID>:<DP>:N<N6>:K<KID>`
- Convert to bytes as ASCII (UTF‑8, no BOM)

### 3.3 HMAC
- Compute `H = HMAC_SHA256(key = K_vendor,q, msg = canonical_bytes)`
- Truncate: `H12 = H[0:12]` (12 bytes)
- Encode `SIG = Base32Crockford(H12)` (uppercase, no padding)


## 4. Encoding & QR parameters

- Character set: ASCII uppercase, digits, colon `:`, letters `V`, `B`, `N`, `K` as markers
- Recommended QR version and ECC:
  - ERC/liner/pad (small surface): Version 3 (29×29) at ECC M; if space permits, ECC H
  - Sleeper (large surface): Version 4+ at ECC H
- Quiet zone: minimum 4 modules; recommended 6 on metal
- Module size (laser etch):
  - Stainless/spring steel ERCs: 0.45–0.60 mm/module
  - Rubber pads/liners: 0.55–0.75 mm/module
  - Concrete sleeper: 0.60–0.90 mm/module
- Physical sizes (guidance):
  - ERC/liner/pad: 16–22 mm square
  - Sleeper: 24–32 mm square


## 5. Provisioning API (sketch)

A separate Vendor Application (not in this repo) calls these APIs.

### 5.1 Request (batch or range)
`POST /vendor/qr/provision`

Body:
```json
{
  "vendor_id": "VENDOR001",
  "batch_id": "BATCH001",
  "item_type": "ERC",
  "date_of_production": "2025-01-01",
  "serial_start": 1,
  "serial_end": 500,
  "kid": "2025Q1",
  "format": "svg",         // svg | png | csv
  "ecc": "M",              // M | H
  "size_mm": 20             // physical target size of QR square
}
```

### 5.2 Response (truncated)
```json
{
  "count": 500,
  "entries": [
    {
      "serial": 1,
      "payload": "RL1:ERC:V3M9QX:B2K7PH:250101:N000001:K2025Q1:7ZX4H8F3Q9D5",
      "qr_hash": "b3c49f2e6c2f...",       
      "kid": "2025Q1",
      "signature_alg": "HMAC-SHA256-12",
      "svg": "<svg ...>...</svg>"
    }
    // ...
  ]
}
```
Notes:
- `qr_hash` is a server lookup key (implementation-defined; may be hex of HMAC or DB id)
- SVG uses the requested size and ECC; quiet zone included; viewBox set so the engraver can scale proportionally


## 6. SVG output requirements

- Must embed quiet zone ≥ 4 modules
- viewBox: square (e.g., `viewBox="0 0 29 29"` for v3)
- Physical size: set via `width/height` in mm; engravers may scale uniformly
- Path complexity: avoid tiny features < 0.12 mm to reduce charring/bleed
- Optional label line (tiny text) is omitted by default to save space


## 7. Validation and verification

- Offline (Flutter):
  - Parse payload, verify Base32, recompute `SIG` using cached vendor JWKS‑like metadata (public pseudo‑keys or keyed hash params)
  - If compute not possible offline (to keep secrets server‑side), validate signature shape and check basic constraints, then perform online verify when connectivity returns
- Online (server):
  - Recompute HMAC with `K_vendor,q` (resolved via `kid`), match `SIG`
  - Anti‑fraud checks: duplicate serials, geo anomaly, QC leakage


## 8. Test guidance

To produce a deterministic test vector:
1. Fix inputs: `vendor_id="VENDOR001"`, `batch_id="BATCH001"`, `item_type="ERC"`, `dp=250101`, `n=000123`, `kid=2025Q1`
2. Derive `VID`/`BID` using SHA256 + Base32 Crockford (first 6 chars)
3. Build canonical: `RL1:ERC:V<VID>:B<BID>:250101:N000123:K2025Q1`
4. Use a known test key for dev (e.g., `K_vendor,q = 32 bytes of 0x11`) to compute HMAC‑SHA256
5. Truncate to 12 bytes and Base32 encode for `SIG`
6. The full QR text should be ≤ ~48 chars and fit in QR Version 3 ECC M comfortably


## 9. Warranty computation

- Warranty start = manufacture date
- Remaining coverage at scan = `expiry_date - now`
- Fitment date is recorded for context and analytics but does not shift warranty clock (policy may evolve later)


## 10. Geolocation capture

- Inspector and fitment scans attempt to attach device geolocation automatically (user OS prompts)
- Store timestamp, lat/lon, accuracy; redact to policy precision for public surfaces
- Use geofence checks to detect out‑of‑route movement and counterfeit risk


## 11. Change control

- Any field addition bumps `v` (format version)
- Payloads with unknown `v` must be rejected or handled in compatibility mode with explicit logging

