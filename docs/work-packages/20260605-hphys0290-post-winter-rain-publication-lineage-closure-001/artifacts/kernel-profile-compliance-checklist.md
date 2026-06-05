# Kernel Profile Compliance Checklist

Status: complete
Evidence mode: Static + Ran

## Contract-First Sequencing

Static:

- Canonical contract amendments were made in `SC-WATBAL-001`,
  `SC-RUNOFFPART-001`, and `SC-SNOWFREEZE-001` before production code edits.
- Contract-derived tests were added in
  `tests/integration/hphys0290_post_winter_rain_publication_contract.rs` and
  `tests/integration/sim_contract_boundary_unit_registry.rs`.
- Pre-implementation contract-gate failures are recorded in
  `artifacts/pre-implementation-contract-gate.md`.

## Canonical Authority

Static:

- `SC-WATBAL-001#INV-WATBAL-065` defines WB13 `RM` as explicit
  `snow.post_winter_rain_m + snow.routed_melt_m + Irr`.
- `SC-RUNOFFPART-001#INV-RUNOFFPART-020` defines runoff reconciliation
  publication of post-winter direct rain.
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-023` defines the post-winter rain
  snow/winter producer surface.
- Baseline citations are pinned to
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Typed Guards

Static:

- `snow.post_winter_rain_m` is registered as typed-required,
  non-negative finite depth in `crates/openwepp-sim-contract/src/units.rs`.
- WB13 requires the value from the flux surface and rejects missing,
  non-finite, and negative values before publication.
- The final implementation does not use a state default or reset value to
  satisfy the post-winter rain publication requirement.

## Required Gates

Ran:

- `cargo fmt --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `cargo test --workspace` -> pass.
- `cargo deny check` -> pass.
- `bash tools/release/check_authority_suite_antievasion.sh` -> pass.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` -> pass.
- Gate log: `/tmp/hphys0290_final_gates_20260605T013019Z_after_nan/status.tsv`.

Disposition: kernel-process profile requirements for the scoped WB13
publication seam are satisfied. Package remains `executed-hold` only because
full semantic parity remains open.
