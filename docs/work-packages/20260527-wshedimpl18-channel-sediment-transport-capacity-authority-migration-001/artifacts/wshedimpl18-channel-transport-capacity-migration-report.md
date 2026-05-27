# WSHEDIMPL18 Channel Transport-Capacity Migration Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WS10 channel sediment publication path no longer sets `tc` with surrogate
  identity coupling (`tc = qsed`).
- Transport-capacity branch now consumes:
  - class-aware hillslope payload aggregation (`mass_kg`, fractions,
    diameters),
  - WS17 terminal segment slope/width scaffold (`nslpts` lookup),
  - WS15/WS16 control and conversion lineage (`ishape`, `ctlz`, `chnz`,
    `chnnbr`, `chntcr`) to drive hydraulic coupling.
- Migrated baseline-lineage helper families are active in watershed kernel path:
  - `shield` critical-shear interpolation,
  - `hydchn`-style width/shear coupling,
  - `trncap` iterative class-capacity partitioning.
- WS11 vectors now verify:
  - `qsed` remains mass-conserved from payload ingress,
  - `tc` is finite/non-negative and not collapsed to `qsed`,
  - `tc` changes under class-diameter perturbation while `qsed` remains stable.
- Residual blockers remain open and non-promotable:
  - `GAP-SYSTEM-008`
  - `GAP-ROUTE-009`
  - `GAP-SED-006`

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only)
