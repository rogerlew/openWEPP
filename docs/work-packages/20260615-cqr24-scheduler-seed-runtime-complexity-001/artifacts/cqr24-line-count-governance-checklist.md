# CQR24 Line-Count Governance Checklist

Status: complete.

Static: before line counts:

- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`: `2371`
- `docs/work-packages/README.md`: `625`
- `docs/work-packages/cqr-burndown-execplan.md`: `710`

Static: after line counts:

- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`: `2584`
- `docs/work-packages/README.md`: `625`
- `docs/work-packages/cqr-burndown-execplan.md`: `710`

Static: touched `.rs` file remains below the `3000` line governance cap.

Static: suppression census before CQR24:

- Line `906`: target `#[allow(clippy::too_many_lines, clippy::similar_names)]`
- Line `1688`: pre-existing `#[allow(clippy::too_many_lines)]` on
  `execute_scheduler_kernel_lifecycle`

Static: suppression census after CQR24:

- Line `1901`: pre-existing `#[allow(clippy::too_many_lines)]` on
  `execute_scheduler_kernel_lifecycle`

Static: CQR24 removed the target broad suppression and added no new
suppression.
