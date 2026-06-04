# Gate Results

Status: completed/HOLD
Evidence mode: ran

| Gate | Result | Notes |
|---|---:|---|
| `cargo fmt --check` | pass | Ran after formatting. |
| Focused HPHYS0281 runner tests | pass | `cargo test -p openwepp-runner hphys0281 -- --nocapture`: 2 passed. |
| Focused HPHYS0281 orchestrator tests | pass | `cargo test -p openwepp-hillslope-orchestrator hphys0281 -- --nocapture`: 1 passed. |
| SIMIMPL18 blocker rerun | pass | `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18 -- --nocapture`: 2 passed. |
| Unit registry gate | pass | `tools/release/check_unit_registry.sh`: 13 passed plus focused clippy. |
| Workspace clippy | pass | `cargo clippy --workspace --all-targets -- -D warnings`. |
| Workspace tests | pass | `cargo test --workspace`. |
| Cargo deny | pass | Existing duplicate/unmatched-license warnings only. |
| Docs lint | pass | `markdown-doc lint` scoped to SC-EVAP, unit registry, HPHYS0281 package, and work-package README; 24 files. |
| Diff hygiene | pass | `git diff --check`. |
| SC-EVAP unit compliance lint | HOLD | 11 pre-existing HPHYS0279 findings remain; new symbol is covered. |

Post-review rerun impact: after Review A/B fixes, focused HPHYS0281 tests,
SIMIMPL18, workspace clippy, full workspace tests, cargo deny, docs lint, and
diff hygiene all pass. Package remains `completed/HOLD` only for pre-existing
SC-EVAP unit-compliance remediation debt outside this package's physics
correction scope.

Post-verification rerun impact: after correcting the producer fixture residue
literal identified by Verification B, focused HPHYS0281 tests, workspace clippy,
docs lint, diff hygiene, and full workspace tests all pass.
