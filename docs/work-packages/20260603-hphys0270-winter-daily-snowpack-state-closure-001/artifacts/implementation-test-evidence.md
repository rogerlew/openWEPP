# Implementation Test Evidence

Status: completed/HOLD
Evidence mode: ran

Static:

- `crates/openwepp-runner/src/hillslope/mod.rs` now publishes HPHYS trace schema `v9`.
- The trace row includes day-begin snow state: `snow_runtime_swe_before_m`, `snow_runtime_depth_before_m`, `snow_runtime_density_before_kg_m3`, and `snow_runtime_settle_day_count_before`.
- The trace row includes post-minus-pre deltas: `snow_runtime_swe_delta_m`, `snow_runtime_depth_delta_m`, `snow_runtime_density_delta_kg_m3`, and `snow_runtime_settle_day_count_delta`.
- Scheduler lifecycle rows carry the day-begin state through `post_seed`, `post_phase`, `post_scheduler`, and `post_wb13` trace rows for traced days.
- `hphys0270_diagnostics.py` produces required H1/H7/H39 classification reports using the new fields.

Ran:

- `cargo fmt --check` returned `0`.
- `cargo clippy --workspace --all-targets -- -D warnings` returned `0`.
- `cargo test -p openwepp-runner hphys02 --lib -- --nocapture` returned `0`.
- `cargo test --workspace` returned `101` due two pre-existing SIMIMPL18 fixture failures in `pl14s_tier_a_candidate_emission_and_replay_contract`; the failures report `HKERNEL-WB11-ET-E-003` during evapotranspiration on fixture day 1 and are not introduced by the HPHYS0270 trace-only changes.
- `cargo deny check` returned `0` with existing duplicate-crate and unmatched-license-allowance warnings.
