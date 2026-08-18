# Required Reading Map

Status: `Static intake complete at 546bf150ad179e7ed3175b575805e9f874c14a75`.

Applicable instructions:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

Binding process and validation authority:

- `docs/standards/testing-and-gate-strategy.md`
- `SC-VEGETATION-001@13`
- `SC-VEGETATIONTRANSACTION-001`
- `SC-LANDSURFACEENERGY-001@3`
- `SC-SURFACELIQUID-001`
- `SC-WATBAL-001`
- `SC-BIOGEOCHEM-001`

Terminal prerequisite evidence:

- Child-3 `final-disposition.md`, `gate-results.md`,
  `terminal-diff-reconciliation.md`, and `implementation-and-test-evidence.md`
- V9 reconciliation `final-disposition.md`, `gate-results.md`, authority
  decision, exact definition/vector/runtime descriptor and dual-verifier
  evidence
- campaign `current-owner-and-state-map.md`,
  `current-scheduler-and-cadence-map.md`,
  `real-hydrology-path-trace.md`, and `production-selector-exclusion-map.md`

Runtime source map:

- `direct_runtime/03_executor.rs`: actual day scheduler and frozen hook after
  `apply_publication_day_input`, before `run_day_spans_hydrology`
- `direct_runtime/00_core_frames.rs`: persistent `DirectRunFrame`, production
  lanes, and default-off surface-liquid owner
- `land_surface_energy_shadow/strict_v8_endpoint.rs`: sole strict physical
  endpoint and sealed uncommitted complete-owner result
- `openwepp-vegetation/src/v9_state.rs`: reviewed exact V8/V9 identity-only
  successor and migration authority

The exact implementation write paths must rerun
`tools/agents/find-agents --for ...` before edits; no nested instruction beyond
the root and `crates/AGENTS.md`/`tests/AGENTS.md` currently applies.
