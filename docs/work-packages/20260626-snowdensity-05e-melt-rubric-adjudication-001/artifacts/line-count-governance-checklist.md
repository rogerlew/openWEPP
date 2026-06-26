# Line-Count Governance Checklist

Evidence mode: Static.

Command:

```text
wc -l crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs crates/openwepp-runner/src/bin/openwepp-snowbench.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs tools/snowfreeze_observed/coe_melt_adjudication.py tests/integration/snowdensity05e_melt_adjudication.rs
```

Observed counts:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs` | 695 | Below `.rs` warn threshold. Self-contained diagnostic replay; split only if 05F promotes it into a longer-lived API. |
| `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | 172 | Below threshold. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs` | 1520 | Below 2000-line WARN threshold. Existing shared helper touched narrowly. |
| `tools/snowfreeze_observed/coe_melt_adjudication.py` | 431 | Python evidence tool, below practical governance concern. |
| `tests/integration/snowdensity05e_melt_adjudication.rs` | 80 | Below threshold. |

No `.rs` file is at or above the 2000-line WARN threshold. No line-count
refactor is required for 05E closure.
