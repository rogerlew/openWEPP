# WSHEDIMPL02 Pre-Migration Failure Baseline

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Objective: confirm WSHED03 expected-failure vectors fail on the current
  partial watershed runtime path before WSHED04+ production migration.

## Ran
- `cargo test -p openwepp --test ws11_channel_routing_physics_equivalence_contract -- --ignored --nocapture`
  - Result: failed as expected (`0 passed; 2 failed`).
  - Observed failures:
    - missing `ws10_channel_1_q1` for KW/MC lineage vector.
    - missing `ws10_channel_1_qsed` for channel-sediment lineage vector.
- `cargo test -p openwepp --test ws12_impoundment_physics_equivalence_contract -- --ignored --nocapture`
  - Result: failed as expected (`0 passed; 2 failed`).
  - Observed failures:
    - parser-projected coefficient vector fails with
      `WKERNEL-WS10-IMPOUNDMENT-E-001` (missing required payload after manual
      seed removal).
    - RK4/regime-transition vector fails timestep-stability assertion
      (`fine=0.7023321033706627`, `coarse=0.7233210337066273`).
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --ignored --nocapture`
  - Result: failed as expected (`0 passed; 1 failed`).
  - Observed failure:
    - watershed CLI non-stub emission vector fails with
      `CLIWAT-E-034` wrapping `OWSOUT-E-004`.
