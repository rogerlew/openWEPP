# Terminal Gate Results

Updated: 2026-07-19 PDT.

| Gate | Result | Evidence |
| --- | --- | --- |
| Rustfmt | PASS | Exact candidate, exit 0. |
| Clippy | PASS | Workspace/all targets, warnings denied, exit 0. |
| Full Nextest | PASS | 2,165/2,165 passed, 5 skipped, exact forest1 candidate. |
| Cargo deny | PASS | Advisories, bans, licenses, and sources `ok`. |
| Affected CRAP | PASS | 62/62 tests; 0 raw / 0 adjudicated / 0 actionable. |
| Global CRAP | PASS | 2 raw / 2 adjudicated / 0 actionable; closure eligible. |
| Forest1 image/runtime | PASS | Exact image, provider ID 23, confinement and cleanup probes pass. |
| Conservative rollback smoke | PASS | Run 29692305394; hosted non-qualifying receipt; all broad steps skipped. |
| Normal forest1 workflow consumer | HOLD | Run 29692405550 proved forest1 admission/bootstrap/build, then failed pre-gate because the increment omitted its owning `package.md`; retry includes that intent delta. |
| Line count | PASS with WARN | Planner/executor/verifier are 2,963/2,514/2,526 lines; all remain below 3,000 with retained state-machine rationale. |
| Documentation integrity | PASS | Changed package Markdown passes canonical lint and link-relative layout. |

No successful heavy command is scheduled for repetition. Final cutover is
conditional only on the cheap normal workflow consumer and dual terminal
inspection of its authenticated aggregate.
