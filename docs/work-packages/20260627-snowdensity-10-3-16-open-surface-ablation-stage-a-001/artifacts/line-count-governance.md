# Line-Count Governance

Evidence mode: Ran.

Command:

```text
wc -l \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs \
  crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs \
  crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs \
  tools/snowfreeze_observed/open_surface_ablation_stage_a.py \
  tests/integration/snowdensity10_3_16_open_surface_ablation_stage_a.rs
```

Results:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs` | 1864 | below WARN |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs` | 990 | below WARN |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2605 | WARN, pre-existing large direct-publication helper; touched only selector/trace plumbing |
| `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs` | 916 | below WARN |
| `tools/snowfreeze_observed/open_surface_ablation_stage_a.py` | 405 | below WARN |
| `tests/integration/snowdensity10_3_16_open_surface_ablation_stage_a.rs` | 173 | below WARN |

No touched Rust file exceeds the 3000-line refactor threshold.
