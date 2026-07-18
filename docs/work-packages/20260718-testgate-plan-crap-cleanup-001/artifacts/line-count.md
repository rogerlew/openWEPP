# Line-Count Governance

`wc -l crates/openwepp-gate-planner/src/*.rs` after decomposition reports:

| File | Lines | Disposition |
|---|---:|---|
| `ledger.rs` | 1,027 | PASS |
| `main.rs` | 482 | PASS |
| `planner.rs` | 2,250 | WARN |
| `verifier.rs` | 1,794 | PASS |

`planner.rs` remains below the 3,000-line blocking threshold. Moving its
manifest/root and execution-context sections during a CRAP-only refactor would
introduce module-privacy, imports, and source-location churn outside the
retained behavior risk. The WARN is accepted for this package with a binding
split intent: the next structural planner package must extract those two seams
before adding new planner behavior.
