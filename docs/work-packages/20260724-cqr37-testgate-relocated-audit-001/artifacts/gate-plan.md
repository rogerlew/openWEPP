# Gate Plan

Status: `ACTIVE`

| Order | Gate | State |
| ---: | --- | --- |
| 1 | Diff hygiene and Markdown lint | PASS |
| 2 | Focused public relocated-audit verifier test | PASS |
| 3 | Rust formatting and affected warnings-denied Clippy | PASS |
| 4 | Affected adjudicated CRAP for `openwepp-gate-planner` | PASS |
| 5 | Exact-diff TESTGATE planning and pre-heavy audit | NOT RUN |
| 6 | One changed-head TESTGATE qualification | NOT RUN |

Full-workspace Nextest and global CRAP are terminal TESTGATE nodes. They must
not run before gates 1 through 4 pass.

Ran:

- Focused test: `1 passed`, `185 skipped`.
- Fresh affected report:
  `target/cqr37-testgate-relocated-audit-r2/adjudicated-crap-report.json`.
- Target after: coverage `100%`, CC `8`, CRAP `8`.
- Affected report: `PASS`, raw `0`, actionable `0`.
- Frozen/final production manifest:
  `c95e9549c74c9b3d18adf12810a84aec33041568d647caf00dcb9e95ed51d6d8`.
