# Line-Count Governance

Evidence class: `Ran`

Final source counts from `wc -l crates/openwepp-gate-planner/src/*.rs`:

| File | Lines | Disposition |
| --- | ---: | --- |
| `planner.rs` | 2,189 | WARN |
| `verifier.rs` | 1,561 | PASS |
| `repository.rs` | 1,233 | PASS |
| `ledger.rs` | 845 | PASS |
| `policy.rs` | 452 | PASS |
| `main.rs` | 421 | PASS |
| `canonical.rs` | 286 | PASS |
| `error.rs` | 48 | PASS |
| `lib.rs` | 16 | PASS |

No file reaches the 3,000-line closure blocker. `planner.rs` exceeds the
2,000-line warning threshold because it currently combines plan semantics,
manifest/root construction, and tool/environment identity. The split intent is
to extract manifest roots and execution-context identity into separate modules
during CRAP remediation, when that mechanical decomposition can be verified
once alongside the metric-closure work instead of triggering another broad
validation cycle here.
