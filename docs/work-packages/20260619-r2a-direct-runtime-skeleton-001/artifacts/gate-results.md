# R2A Gate Results

Status: complete.
Evidence mode: Static + Ran.

| Gate | Result | Evidence |
|---|---|---|
| Required-reading map populated | PASS | `required-reading-map.md` complete; required-reading bytes `212304 <= 400000`. |
| Owned-file manifest complete | PASS | `owned-file-manifest.md` complete and scoped. |
| Pre-implementation gate complete | PASS | `pre-implementation-contract-gate.md` PASS before Rust edits. |
| Direct type namespace implemented | PASS | `direct_runtime.rs` added with direct frame/executor namespace. |
| Direct-frame type prohibitions pass | PASS | Source-token test and `rg` scan returned no forbidden direct-runtime tokens. |
| Direct skeleton selected once and inactive by default | PASS | Static selector review plus runner R2A tests. |
| No-compatibility proof passes | PASS | Direct source scan, scheduler no-diff, and corrected proof model. |
| Runtime counters/audit proof passes | PASS | Direct skeleton construction/execution counters passed after race fix. |
| Focused tests pass | PASS | `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`; `cargo test -p openwepp-runner r2a_ -- --nocapture`. |
| Default-disabled H2637 median `<= 676.67 s` | PASS | Reps `634.06/636.01/640.93 s`; median `636.01 s`. |
| Protected identity passes | PASS | HBP/loss/WAT/plot checksum stability; PASS parquet DuckDB row/schema equivalence. |
| No phase math or publication cutover added | PASS | Static diff review; no output writer or scheduler phase logic touched. |
| Full Rust closure gates pass | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`. |
| Markdown lint passes | PASS | `markdown-doc lint --path docs/work-packages/20260619-r2a-direct-runtime-skeleton-001 --no-ignore`; `markdown-doc lint --path docs/work-packages/README.md --no-ignore`; `markdown-doc lint --path docs/ROADMAP.md --no-ignore`. |
| `git diff --check` passes | PASS | `git diff --check` exited 0. |
| Line-count governance complete | PASS | `line-count-governance.md` complete; touched WARN-band runner setup file disposition recorded. |
| Dual review complete | PASS | `review_agent_a.md` and `review_agent_b.md` complete; both findings dispositioned. |
| Finding disposition complete | PASS | Race fixed; tautological counters removed; artifacts and roadmap updated. |
| Dual verification complete | PASS | `verification_agent_a.md` and `verification_agent_b.md` complete. |

No failing, blocked, or deferred gate remains.
