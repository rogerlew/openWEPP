# PERFDEEP09 Gate Results

Status: complete.
Evidence class: Static + Ran.

| Gate | Result | Evidence |
|---|---|---|
| Required-reading map populated | PASS | `required-reading-map.md` complete; budget `265417 <= 400000` |
| Owned-file manifest complete | PASS | `owned-file-manifest.md` complete |
| Same-machine baseline/control recorded | PASS | no-edit control `682.65 s`, RSS `228924 KB` |
| Profile or micro-benchmark attribution complete | PASS | prior PERFDEEP04 default profile + static repeated-scan proof |
| Remediation loop ledger complete | PASS | `perfdeep09-remediation-iteration-log.md` complete |
| Candidate ledger complete | PASS | rejected registry candidate and retained one-pass guard recorded |
| Focused tests pass | PASS | `cargo test -p openwepp-hillslope-orchestrator pl12_contract_conformance`; `cargo test -p openwepp-hillslope-orchestrator decomposition` |
| Default-disabled H2637 identity passes | PASS | HBP/loss/WAT/plot byte checks; PASS row equivalence zero-diff |
| Default-disabled H2637 median `<= 676.67 s` | PASS | `634.61/635.65/636.58 s`; median `635.65 s` |
| Zero-cost-disabled proof passes | PASS | retained patch removes always-on guard scans; no opt-in activation |
| No R2+ runtime implementation added | PASS | diff limited to decomposition guard, test, and docs |
| Full Rust closure gates pass | PASS | `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check` |
| Markdown lint passes | PASS | `markdown-doc lint --path docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001 --no-ignore`; `markdown-doc lint --path docs/ROADMAP.md --no-ignore`; `markdown-doc lint --path docs/work-packages/README.md --no-ignore` |
| `git diff --check` passes | PASS | `git diff --check` |
| Line-count governance complete | PASS | touched Rust files `1682` and `550` lines |
| Dual review complete | PASS | `review_agent_a.md`, `review_agent_b.md` |
| Finding disposition complete | PASS | no blocking findings |
| Dual verification complete | PASS | `verification_agent_a.md`, `verification_agent_b.md` |

Final disposition is `READY-FOR-R2`.
