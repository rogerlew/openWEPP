# CQR28 Behavior Equivalence

Static: production edits are private helper extraction in
`hydrology_phase_plant_percolation.rs`; no public API or crate-visible kernel
entry point was changed.

Preserved behavior surfaces:

- `Wb11HydrologyKernel::run_percolation` remains the WB18 entry point.
- Runtime symbols and writebacks are unchanged, including
  `wb11_soil_water`, `wb12_infiltration`,
  `wb12_infiltration_same_pass_lineage`, per-layer `theta`, per-layer WB18
  percolation fluxes, `wb11_perc_loss_d`, and `wb11_perc_recharge_pe`.
- Stable status ID `HKERNEL-WB11-PERC-OK-001` is unchanged.
- Typed guard constructors, boundary symbols, and error IDs remain on the same
  validation paths.
- Daily/hourly lane selection, lane substeps, restrictive layer harmonic
  conductivity, bedrock thickness weighting, same-pass infiltration cadence,
  deep-percolation roundoff canonicalization, scalar ledger preservation, and
  layer storage rebalance are preserved.
- WB18 formulas retain their operand order inside extracted helpers, including
  `bi`, `sci`, `fc_ul_ratio`, `st`, `fx`, conductivity adjustment, `pei`, and
  lower-layer saturation damping.

Ran:

- `cargo test --test wb18_percolation_physics_kernel_contract`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`

Conclusion: no intentional behavior change was introduced.
