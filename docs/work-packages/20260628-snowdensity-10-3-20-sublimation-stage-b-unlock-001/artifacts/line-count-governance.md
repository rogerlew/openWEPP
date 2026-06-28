# Line Count Governance

Evidence class: Static.

Checked package-relevant files for line-count risk:

```bash
wc -l \
  docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs \
  tools/snowfreeze_observed/sublimation_stage_b_unlock.py \
  tests/integration/snowdensity10_3_20_sublimation_stage_b_unlock.rs \
  docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/package.md
```

The contract and hydrology helper are pre-existing large files. The package kept
edits scoped to contract authority, one opt-in selector branch, and focused
diagnostic tooling rather than splitting unrelated refactors into this science
package.

Measured counts:

- `SC-SNOWFREEZE-001.md`: `2809` lines.
- `infiltration_reconciliation.rs`: `1971` lines.
- `sublimation_stage_b_unlock.py`: `626` lines.
- `snowdensity10_3_20_sublimation_stage_b_unlock.rs`: `208` lines.
- `package.md`: `149` lines.
