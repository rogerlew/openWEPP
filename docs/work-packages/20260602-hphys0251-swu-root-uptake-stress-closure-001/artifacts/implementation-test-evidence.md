# Implementation and Test Evidence

Status: complete

Evidence mode: static + ran

Static: production changes implement only baseline-authoritative `swu.for`
lineage encoded in `SC-EVAP-001#INV-EVAP-017` and
`SC-WATBAL-001#INV-WATBAL-039`.

## Implementation

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`
  projects crop `residue_line[3]` (`pltol`) into PL growth slot and primary
  aliases while preserving non-finite hard failures.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  applies the baseline `swu.for` `pltol` normalization branch, separates layer
  potential `UPi_####` from actual `Ui_####`, publishes aggregate `UPi`/`Ui`
  from layer sums, and derives final `Ep`/`Ws` from actual layer uptake.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  adds a WB17 indexed layer-flux symbol helper.

## Tests and Gates

Ran:

- `cargo test -p openwepp-hillslope-orchestrator hphys0251_management_projection_preserves_crop_pltol -- --nocapture`
- `cargo test --test wb17_et_physics_kernel_contract hphys0251_ -- --nocapture`
- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Logs

- `artifacts/gate-logs/post_impl_targeted_hphys0251.log`
- `artifacts/gate-logs/post_impl_wb17_integration_full.log`
- `artifacts/gate-logs/cargo_fmt_clippy_workspace_all_targets_final.log`
- `artifacts/gate-logs/cargo_test_workspace.log`
- `artifacts/gate-logs/cargo_deny_check.log`

## Gate Notes

- `cargo clippy --workspace --all-targets -- -D warnings` initially found one
  HPHYS0251 manual-clamp issue and exact float comparisons in the touched
  runtime test seam. Both were corrected before final clippy rerun.
- `cargo deny check` exited `0`; it reported existing duplicate/unmatched
  license warnings only.
