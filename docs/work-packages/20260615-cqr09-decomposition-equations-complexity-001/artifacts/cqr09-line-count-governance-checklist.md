# CQR09 Line-Count Governance Checklist

Ran: `wc -l
crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/phase.rs
docs/work-packages/README.md` reported:

- `07_decomposition_equations.rs`: `1664` lines.
- `phase.rs`: `857` lines.
- `docs/work-packages/README.md`: `494` lines.

Static: no touched `.rs` file is at or above the `2000` line WARN threshold.

Static: no touched non-exempt `.rs` file is at or above the `3000` line closure
blocker threshold.

Ran: suppression census after refactor with `rg -n "allow\(clippy"
crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`
reported:

- line `11`: existing `too_many_lines` for
  `compute_equation_decomposition_seed_surface` outside CQR09 scope.
- line `791`: new `similar_names` around the input bundling helper.
- lines `912`, `985`, `1046`, `1119`, `1186`, and `1253`: moved
  `cast_precision_loss` branch-helper allowances for existing integer-to-float
  validation casts.
- line `1317`: existing `too_many_lines, cast_precision_loss` for
  `build_perennial_decomposition_control` outside CQR09 scope.

Static: the scoped target function no longer needs the former
`too_many_lines` suppression.
