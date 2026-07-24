# Gate Plan

Status: `ACTIVE`

| Order | Gate | State |
| ---: | --- | --- |
| 1 | Diff hygiene and Markdown lint | NOT RUN |
| 2 | Focused public relocated-audit verifier test | NOT RUN |
| 3 | Rust formatting and affected warnings-denied Clippy | NOT RUN |
| 4 | Affected adjudicated CRAP for `openwepp-gate-planner` | NOT RUN |
| 5 | Exact-diff TESTGATE planning and pre-heavy audit | NOT RUN |
| 6 | One changed-head TESTGATE qualification | NOT RUN |

Full-workspace Nextest and global CRAP are terminal TESTGATE nodes. They must
not run before gates 1 through 4 pass.
