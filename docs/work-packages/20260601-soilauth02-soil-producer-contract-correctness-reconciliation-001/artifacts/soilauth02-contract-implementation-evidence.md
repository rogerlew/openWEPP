# SOILAUTH02 Contract Implementation Evidence

Status: complete  
Evidence mode: Static

## Scope
Contract-first authority updates completed before final parser remediation
closure.

## Updated Authority Surfaces
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
  - ratified canonical quoted-header `avke` omission normalization note,
  - ratified canonical `9002/9003/9005` policy-first row ordering note,
  - ratified canonical per-OFE restrictive-row placement note.
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
  - datver matrix updated: quoted headers and per-OFE restrictive rows accepted
    in strict+compat envelopes,
  - grammar updated: policy-first/header-first accepted for policy datvers,
  - compatibility + strict policy sections updated,
  - guard map updated for quoted-token and restrictive-row invariants,
  - revision history entry added (`0.1.9`).

## Producer Ownership Note
- No producer-source edits were applied in `/workdir/wepppy` during SOILAUTH02.
- Reconciliation path selected: openWEPP contract/parser alignment to canonical
  producer envelopes with explicit provenance (no hidden fallback behavior).
