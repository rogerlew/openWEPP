# Gate Results And Frozen Command Inventory

Static: commands selected before implementation/result execution. Results are
recorded after each run with `Ran:` evidence.

| Phase | Command | Expected inventory / claim |
| --- | --- | --- |
| A/C | `.venv/bin/python -m pytest -q docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/tools/test_analyze_evolving_carrier.py` | exact analyzer malformed/alias/aggregation tests |
| B | `cargo nextest run --test snow_stage3_turbulent_operator_reconciliation_contract` | amended focused contract-derived tests |
| B | `cargo fmt --all --check` | Rust integration-test formatting |
| B/F | `cargo clippy --test snow_stage3_turbulent_operator_reconciliation_contract -- -D warnings` | warnings-denied focused Rust test target |
| C | `.venv/bin/python docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/tools/analyze_evolving_carrier.py --verify-retained --receipt target/snow_stage3_operator_reconciliation_v3/execution-receipt.json --results target/snow_stage3_operator_reconciliation_v3/results/operator-reconciliation-results.json --output target/snow_stage3_evolving_state_carrier_plausibility_reconciliation/attempt-001` | exact schema-v6 independent reconstruction and immutable output |
| D/E | comparator runner executes the exact Phase-C analyzer command | 4 sites; eligible WY counts 34/44/41/35; WY2025 censoring; one immutable attempt |
| F | `cargo nextest run --workspace --profile quick` | exact terminal diff fast-workspace correctness |
| F | comparator runner executes `cargo nextest run --workspace --profile full` | critical package/full correctness |
| F | scoped Markdown validation/local links and `git diff --check` | documentation/diff hygiene |
| F | line counts and exact-diff/write-set reconciliation | package governance |

No release CLI rebuild is selected because no Rust production or schema change
is intended. Manifest/dependency files remain unchanged, so `cargo deny check`
is not selected. If the terminal diff changes either fact, the gate set
escalates before disposition.
