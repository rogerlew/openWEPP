# Line-Count Governance Checklist

Status: warning disposition recorded

Evidence mode: **Ran**

- `runoff_reconciliation.rs`: 2,598 lines — `WARN` at the 2,000-line policy
  threshold, below the mandatory 3,000-line pre-closure refactor threshold.
- runner `03_tests.rs`: 2,891 lines after the new EB-04W2B real-path test was
  extracted to `tests03/eb04w2b_warm_snow.rs` — `WARN`, below 3,000.
- `snowbench.rs`: 1,662 lines; `snowbench_coe_melt.rs`: 1,188 lines.
- SIMIMPL28 hourly forcing: 1,344 lines; EB-04W integration target: 217 lines.

Decomposition rationale: extracting the small closure helper during a critical
control fix would mix a mechanical refactor into the science diff and broaden
regression risk. Follow-on split intent: before the next substantive edit to
`runoff_reconciliation.rs`, the snow/frost maintainers should authorize a
mechanical package to extract typed snow storage validation and its tests into
a dedicated support module. The runner test aggregate has already begun the
safe pattern: future row-7/row-8 blocks should move into named `tests03/`
includes before `03_tests.rs` reaches 3,000 lines.
