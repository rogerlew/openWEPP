# PERFDEEP08 Gate Results

Status: HOLD.
Evidence mode: Static/Ran.

| Gate | Result | Evidence |
|---|---|---|
| Required-reading map populated | PASS | `required-reading-map.md`. |
| Owned-file manifest complete | PASS | `owned-file-manifest.md`. |
| Disabled-path baseline recorded | PASS | `perfdeep08-disabled-path-baseline.md`. |
| Disabled-path audit complete | PASS | `perfdeep08-disabled-path-audit.md`. |
| Hard-isolation plan complete | PASS | `perfdeep08-hard-isolation-plan.md`. |
| Focused tests pass | PASS | `cargo test -p openwepp-runner`; `cargo test -p openwepp-hillslope-orchestrator writeback`; kernel-contract focused test. |
| Default-disabled H2637 identity passes | PASS | Candidate manifest reported protected output checksums matching anchor paths. |
| Default-disabled H2637 median `<= 676.67 s` | FAIL | Single screening run was `691.93 s`; three-run median not attempted. |
| Zero-cost-disabled proof passes | FAIL | Candidate did not improve endpoint. |
| No R2+ runtime implementation added | PASS | No direct-frame hydrology/executor/schema code added. |
| Line-count governance complete | PASS | No production Rust edit retained; `scheduler.rs` touch reverted. |
| Full Rust closure gates pass | NOT RUN | Blocked by HOLD; no `READY-FOR-R2` claim. |
| Markdown lint passes | PASS | `markdown-doc lint --path docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001 --path docs/work-packages/README.md --path docs/ROADMAP.md --format json`: `30` files scanned, `0` errors, `0` warnings. |
| `git diff --check` passes | PASS | `git diff --check`. |
| Dual review complete | PASS | `review_agent_a.md`, `review_agent_b.md`. |
| Dual verification complete | PASS | `verification_agent_a.md`, `verification_agent_b.md`. |
| Finding disposition complete | PASS | `disposition.md`. |

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` prevents `READY-FOR-R2`.
PERFDEEP08 therefore closes as `HOLD`.
