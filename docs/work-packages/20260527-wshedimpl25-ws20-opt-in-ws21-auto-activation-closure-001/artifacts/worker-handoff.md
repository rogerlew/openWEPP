# WSHEDIMPL25 Worker Handoff

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Completed in WSHEDIMPL25
- Canonical contracts/index updated for WS25 closure scope:
  - `SC-ROUTE-001` v27
  - `SC-SED-001` v26
  - `SC-SYSTEM-001` v48
  - `science-contracts/index.md` row summaries
- WS11 vectors added for WS20-only opt-in seam behavior:
  - fail-closed missing-`crfrac` expectation,
  - routed success with `crfrac` and zero WS20 unresolved fallback counter.
- Runtime WS10 channel branch control updated so WS20 opt-in auto-activates
  WS21 migration lane:
  - `ws21_case34_enabled = ws20_case12_enabled || ws21_case34_opt_in`.
- Full required gate suite executed and passing.

## Immediate Next Actions
1. Enumerate remaining baseline-authoritative `chnero/chnrt/detach` branches
   still not migrated (beyond WS20/WS21/WS24/WS25 seam closures) and map each
   to explicit `SC-*` gap sub-obligations.
2. Add targeted WS11 vectors for each remaining branch family before runtime
   migration edits, preserving contract-first sequence.
3. Continue migration with no surrogate/heuristic substitutions and keep
   disposition at `HOLD` until `GAP-ROUTE-009`/`GAP-SED-006`/`GAP-SYSTEM-008`
   closure criteria are met.
