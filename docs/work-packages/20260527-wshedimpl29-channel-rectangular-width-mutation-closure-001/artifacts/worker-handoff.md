# WSHEDIMPL29 Worker Handoff

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Completed in WSHEDIMPL29
- Canonical contracts/index updated for WS29 scope:
  - `SC-ROUTE-001` v31
  - `SC-SED-001` v30
  - `SC-SYSTEM-001` v52
  - `science-contracts/index.md` row summaries
- Runtime WS20 detachment lane now projects `dcap` geometry outcomes (`werod`)
  and applies rectangular `widb(i-1)` mutation semantics when `werod > wfu`.
- Mutated `widb` points are now projected into node-state writeback symbols:
  - `ws10_channel_{id}_widb_{point:04}`
- WS11 vector added/passing for rectangular `widb` mutation/writeback behavior.

## Immediate Next Actions
1. Stage the next `chnero/chnrt/detach` closure for shape-transition continuity
   in erodible lanes (`ishape=3` pathways), including authoritative
   `depa/depb`-driven rectangular fallback mapping and explicit `SC-*` row
   traceability.
2. Add targeted WS11 vectors for the selected shape-transition seam before
   runtime edits, including fail-closed coverage for any newly required symbol
   families.
3. Continue migration with explicit HOLD posture until
   `GAP-ROUTE-009`/`GAP-SED-006`/`GAP-SYSTEM-008` closure criteria are met.

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass
