# Implementation And Test Evidence

Status: complete
Evidence mode: Static + Ran

Static:

- Added `SnowDensityModel::PhysicsBulkShallowGuardV1` and selector id
  `physics_bulk_shallow_guard_v1`.
- Added `snow_density_shallow_guard_v1_constants()` with the authority-derived
  `0.25 m` shallow-depth threshold and otherwise unchanged
  `physics_bulk_density_compaction_v1` constants.
- Added a shallow-depth compaction factor applied to the existing dry and wet
  density increments; deep packs at or above `0.25 m` remain identical to
  `physics_bulk_density_compaction_v1`.
- Added the opt-in direct-production env selector while keeping absent/empty
  selectors on the activated default and preserving `legacy_wepp` rollback.
- Added `tools/snowfreeze_observed/shallow_pack_compaction_guard.py` for real
  coupled direct-production WAT and trace evidence.

Ran:

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: passed.
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/shallow_pack_compaction_guard.py`: passed.
- `.venv/bin/python tools/snowfreeze_observed/shallow_pack_compaction_guard.py`:
  wrote `shallow-pack-compaction-guard.json` and `.md`.
- `cargo test --test snowdensity10_3_17_shallow_pack_compaction_guard -- --nocapture`:
  passed, `4 passed`.

Diagnostic result:

- Disposition: `NON-PROMOTION-SHALLOW-GUARD-GATE-NOT-MET`.
- Candidate trace proof: passed.
- Induced under-persistence: `177 -> 176`.
- `harvard_hardwood` induced under-persistence: `73 -> 73`, recovered `0`.
- Over-persistence: `264 -> 267`; new over from non-over: `3`.
- Snow-control failures: `498 -> 500`.
- SWE-depth-density identity closed locally (`1.110e-16 m` max residual), but
  downstream mass-term invariance failed (`3.342e-03 m` max mass-term delta).
