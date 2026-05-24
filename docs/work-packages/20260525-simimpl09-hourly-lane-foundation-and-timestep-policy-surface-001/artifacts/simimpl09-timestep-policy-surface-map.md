# simimpl09 timestep policy surface map

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Typed policy surfaces implemented in runner:
  - `ExecutionLane::{Daily,Hourly}`
  - `TimestepPolicy::{Daily,Hourly,SubHourly{timestep_seconds}}`
  - `ExecutionLaneContext { lane, requested_mode, effective_mode, timestep_policy }`
- Manifest policy publication (`/timestep_policy`):
  - `scheduler_mode`
  - `requested_mode`
  - `effective_mode`
  - `selected_lane`
  - `policy`
  - `timestep_seconds`
  - `physics_enabled`
  - `subhourly_scaffold_available`
  - `guard_id = HS-SIMMODE-E-001`
- Guard semantics:
  - unsupported lane symbols hard-fail at `timestep_policy` boundary,
  - lane/effective-mode mismatch hard-fails at `timestep_policy` boundary.

## Ran
- Validated published policy surface through active contract test:
  - `cargo test -p openwepp-runner --test simimpl04_wepp_ui_mode_closure_contract`
