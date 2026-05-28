# WSHEDIMPL30 Worker Handoff

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Completed in WSHEDIMPL30
- Canonical contracts/index updated for WS30 scope:
  - `SC-ROUTE-001` v32
  - `SC-SED-001` v31
  - `SC-SYSTEM-001` v53
  - `science-contracts/index.md` notes refreshed
- WS10 runtime now supports erodible-lane `ishape=3` execution.
- WS20/WS21 routing lanes now apply baseline-authoritative fallback mapping:
  - upper boundary rectangular fallback from `depb(i-1) <= 1e-4`,
  - lower boundary rectangular fallback from `depa(i) <= 1e-4`.
- WS11 vectors added/passing for WS30 seam behavior.

## Immediate Next Actions
1. Migrate baseline `detach.for` lower-boundary width mutation semantics
   (`if flagc=2 and wera(i)>wfl then wida(i)=wera(i)`) into WS23/WS24 closure
   pathways with explicit writeback projection strategy.
2. Reconcile parser/runtime shape-code lineage so naturally eroded class mapping
   from watershed channel input authority is explicit and unambiguous across
   parser projection and WS10 kernel consumption.
3. Continue channel sediment migration with explicit HOLD posture until
   `GAP-ROUTE-009`/`GAP-SED-006`/`GAP-SYSTEM-008` closure criteria are met.

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass
