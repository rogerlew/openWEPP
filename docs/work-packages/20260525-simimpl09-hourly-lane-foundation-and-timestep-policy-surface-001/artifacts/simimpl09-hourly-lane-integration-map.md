# simimpl09 hourly lane integration map

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Hourly lane closure chain implemented as typed runner flow:
  1. parse `wepp_ui` requested/effective mode,
  2. build mode-selection provenance (`requested`, `effective`, `selected_lane`),
  3. resolve typed `ExecutionLaneContext` from provenance,
  4. execute scheduler/kernel lifecycle using typed lane context,
  5. publish execution, timestep-policy, and adapter-boundary provenance.
- Lane semantics:
  - `ui_run=0` -> `daily`, `timestep_seconds=86400`.
  - `ui_run=1` -> `hourly`, `timestep_seconds=3600`.
- Sub-hourly is represented as typed scaffold only; not enabled for physics
  execution in SIMIMPL09.

## Ran
- Integration points verified in `crates/openwepp-runner/src/lib.rs` and
  contract-derived manifest assertions in
  `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`.
