# Line-Count Governance Checklist

Evidence class: Ran.

```text
wc -l crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs \
  crates/openwepp-runner/src/hillslope/snowbench.rs \
  tools/snowfreeze_observed/coe_melt_adjudication.py \
  tests/integration/snowdensity05g_harness_fidelity_rerun.rs \
  docs/work-packages/20260626-snowdensity-05g-harness-fidelity-rerun-001/package.md
```

Result:

- `snowbench_coe_melt.rs`: `721` lines.
- `snowbench.rs`: `1424` lines.
- `coe_melt_adjudication.py`: `431` lines.
- `snowdensity05g_harness_fidelity_rerun.rs`: `122` lines.
- `package.md`: `119` lines.

No touched `.rs` file crosses the 2000-line WARN threshold or 3000-line
refactor threshold.

