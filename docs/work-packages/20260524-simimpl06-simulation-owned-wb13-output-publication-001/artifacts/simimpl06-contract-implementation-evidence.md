# simimpl06 contract implementation evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- SIMIMPL03 authority references verified:
  - `INV-WATBAL-020` (simulation-owned WB13 provenance closure)
  - `INV-SYSTEM-020` (simulation-owned replay-surface publication closure)
  - `G-WUI-009` / `WUI-E-005` mapping context from `SC-INFILE-WEPPUI-001`
- Production runner manifest authority now includes explicit WB13 provenance
  subtree:
  - `wb13_publication.source = "simulation-owned"`
  - `wb13_publication.projection_fallback_used = false`
  - `wb13_publication.guard_id = "HS-SIMOUT-E-001"`
  - `wb13_publication.replay_candidate_surfaces = ["interchange/H.wat.parquet", "interchange/H.pass.parquet"]`
- Projection-first WB13 assembly helper (`build_first_day_wat_projection`) was
  removed from production WB13/H.wat publication flow.
- WB13/H.wat publication now assembles from executed scheduler/kernel
  writeback-surface state via `build_simulation_owned_wb13_row(...)`.

## Ran
- Code changes were implemented in:
  - `crates/openwepp-runner/src/lib.rs`
  - `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
