# WSHEDIMPL28 Worker Handoff

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Completed in WSHEDIMPL28
- Canonical contracts/index updated for WS28 closure scope:
  - `SC-ROUTE-001` v30
  - `SC-SED-001` v29
  - `SC-SYSTEM-001` v51
  - `science-contracts/index.md` row summaries
- Runtime WS20 segment routing now preserves baseline `chnrt.for` boundary
  width semantics (`widb(i-1)` upper boundary, `wida(i)` lower boundary).
- WS11 vector added/passing for lower-boundary `wida` sensitivity in WS20/WS21
  routing lanes.
- Required gate suite executed and passing.

## Immediate Next Actions
1. Enumerate and stage the next remaining baseline-authoritative
   `chnero/chnrt/detach` closures still open after WSHEDIMPL28 with explicit
   `SC-*` row mapping, prioritizing boundary-geometry mutation semantics
   (`widb` updates from `werb`/`dcap` lanes and related shape transitions).
2. Add targeted WS11 vectors for the selected next branch-family scope before
   runtime edits, preserving contract-first ordering and explicit hydraulic-lane
   forcing where needed.
3. Continue migration and keep disposition at `HOLD` until
   `GAP-ROUTE-009`/`GAP-SED-006`/`GAP-SYSTEM-008` closure criteria are met.

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass
