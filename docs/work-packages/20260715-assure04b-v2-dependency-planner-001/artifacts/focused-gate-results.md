# ASSURE-04B Focused Gate Results

Status: PASS; focused evidence and terminal heavy closure are current

Evidence class: Ran

| Gate | Result |
| --- | --- |
| `cargo fmt --all` / subsequent format | PASS |
| Assurance crate unit/binary tests | PASS, 6/6 |
| Three assurance integration suites | PASS, 35/35 |
| Post-heavy-HOLD planner integration renewal | PASS, 10/10 |
| Post-CRAP dispatcher remediation | PASS: assurance crate 6/6; three assurance suites 35/35; focused instrumented suites 35/35 |
| Focused all-target clippy, warnings denied | PASS |
| Quick workspace Nextest | PASS after final CRAP remediation, 1,916/1,916; 34 profile skips |
| Real named/all human and JSON plans | PASS |
| Ordinary-plan no-write status comparison | PASS, no delta |
| `git diff --check` | PASS |
| `markdown-doc lint` on package and changed docs | PASS, zero errors/warnings |
| `markdown-doc validate` on package | PASS, 23 files at terminal review |
| `uk2us` preview on changed prose | PASS, no proposed changes |
| Protected four-file SHA-256 set | PASS, exact intake identities |
| Aggregate `usersum/**` SHA-256 | PASS, `deb9f2c...bdcb7a` |

The prior heavy attempt passed full workspace, all-target Clippy, full Nextest,
and dependency policy but failed fresh CRAP. Those results are retained as HOLD
chronology and are not combined with remediation evidence. They were superseded
by the fully restarted terminal sequence recorded in `heavy-gate-runner.md`:
all five gates PASS, full Nextest 2,001/2,001 with three skipped, and fresh CRAP
2 raw / 2 adjudicated / 0 actionable with every touched-file maximum at or
below 26.
