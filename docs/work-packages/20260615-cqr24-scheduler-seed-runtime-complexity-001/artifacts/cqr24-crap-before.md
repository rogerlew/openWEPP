# CQR24 CRAP Before

Status: complete.

Ran: `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr24-scheduler-seed-runtime-complexity-001/artifacts/lcov_before.info`.

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr24-scheduler-seed-runtime-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr24-scheduler-seed-runtime-complexity-001/artifacts/crap_before.json`.

Static: CQR24 live target identity from `crap_before.json`:

- Function: `produce_wb16_ealpha_from_runtime_surface`
- File: `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
- Line: `907`
- Cyclomatic complexity: `58.0`
- Coverage: `57.446808510638306`
- CRAP: `317.2103869084884`

Static: target-file LCOV before summary:

- Lines: `1424/2011`, `70.81%`
- Functions: `79/108`, `73.15%`

Static: same-file non-target CRAP rows above closure threshold before refactor:

- `execute_scheduler_kernel_lifecycle`: CRAP `54.66251538901941`
- `pl_runtime_has_active_crop_for_scheduler_day`: CRAP `46.39958249299923`
- `refresh_wb18_frozen_depth_from_fine_frost_state`: CRAP `35.38643604915795`
- `pl_crop_slot_is_active_for_day`: CRAP `33.11738193219675`

Static: `cargo crap` warned that `126` source files had no matching LCOV entry.
The target file did have matching LCOV data.
