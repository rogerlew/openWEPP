# CQR15 CRAP Before

Status: complete.

Ran:

```bash
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001/artifacts/lcov_before.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001/artifacts/crap_before.json
```

Ran: LCOV completed and wrote `artifacts/lcov_before.info`.

Ran: `cargo crap` completed and wrote `artifacts/crap_before.json`. It emitted
the recurring workspace warning that 125 source files had no matching LCOV
entry; this warning also appeared in after capture and did not block CRAP JSON
emission.

Static: target-file line count before production edit was `2124`.

Static: suppression census before production edit:

- Line 1: `#[allow(clippy::too_many_lines)]` on
  `seed_wb11_runtime_surface_inputs`, the CQR15 target.
- Line 1441: `#[allow(clippy::too_many_lines)]` on an out-of-scope later
  function in the same file.

Ran: target-file coverage before:

- Lines: `1220/1819`, `67.07%`
- Functions: `52/81`, `64.20%`

Ran: highest live rows before:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `seed_wb11_runtime_surface_inputs` | 2 | 94.0 | 61.95426195426196 | 580.6018405181356 |
| `produce_wb16_ealpha_from_runtime_surface` | 660 | 58.0 | 57.446808510638306 | 317.2103869084884 |
| `execute_scheduler_kernel_lifecycle` | 1442 | 13.0 | 37.2972972972973 | 54.66251538901941 |
| `pl_runtime_has_active_crop_for_scheduler_day` | 1929 | 23.0 | 64.63414634146342 | 46.39958249299923 |
| `refresh_wb18_frozen_depth_from_fine_frost_state` | 557 | 16.0 | 57.692307692307686 | 35.38643604915795 |
| `pl_crop_slot_is_active_for_day` | 2026 | 14.0 | 53.96825396825397 | 33.11738193219675 |

Static: the CQR15 scoped target is `seed_wb11_runtime_surface_inputs`; other
rows above 30 are out of scope for this package.
