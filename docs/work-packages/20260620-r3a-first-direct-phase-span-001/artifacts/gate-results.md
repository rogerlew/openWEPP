# R3A Gate Results

Status: complete.
Evidence mode: Static + Ran.

| Gate | Result | Evidence |
|---|---|---|
| Required-reading map populated | PASS | `required-reading-map.md`; `239992 <= 400000` bytes. |
| Owned-file manifest complete | PASS | `owned-file-manifest.md` complete and scoped. |
| Pre-implementation gate complete | PASS | `pre-implementation-contract-gate.md` PASS before Rust edits. |
| Complete phase span selected | PASS | `r3a-phase-span-contract.md`: direct transfer-input accounting. |
| Inputs/compute/mutation/downstream/shadow contract satisfied | PASS | Direct implementation and focused R3A identity test. |
| Phase-span identity passes | PASS | Exact binary-fraction fixture and shadow projection equality. |
| No-compatibility call-graph proof passes | PASS | Forbidden-token scan no matches; `scheduler.rs` no diff. |
| Runtime counters are non-tautological and pass | PASS | Production opt-in compatibility-edge handoff counter; default/opt-in runner counters. |
| Focused tests pass | PASS | Orchestrator R3A, orchestrator R2A, and runner R2A filters pass. |
| Default-disabled H2637 median `<= 676.67 s` | PASS | `630.31/640.85/632.08 s`; median `632.08 s`. |
| Protected identity passes | PASS | HBP/loss/WAT/plot checksum stability; PASS DuckDB row/schema equivalence. |
| No publication cutover or default activation | PASS | No output writer/schema or scheduler edits; compat policy benchmark. |
| Full Rust closure gates pass | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`. |
| Markdown lint passes | PASS | Scoped markdown lint recorded after final artifact edits. |
| `git diff --check` passes | PASS | `git diff --check` exited 0. |
| Line-count governance complete | PASS | Direct-runtime files below 2000 lines; pre-existing runner setup WARN-band file dispositioned; no scheduler diff. |
| Dual review complete | PASS | `review_agent_a.md` and `review_agent_b.md`. |
| Finding disposition complete | PASS | No blocking findings remain. |
| Dual verification complete | PASS | `verification_agent_a.md` and `verification_agent_b.md`. |

No failing, blocked, or deferred gate remains.
