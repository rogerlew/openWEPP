# Gate Results

Status: passed

Evidence mode: Ran

| Gate | Result |
|---|---|
| pre-implementation contract gate | PASS, 36/36 |
| terminal analytical and real-seam tests | PASS, 8/8 orchestrator |
| serialized-row terminal consumer tests | PASS, 4/4 nextest |
| touched-crate nextest suite | PASS, 697/697 |
| terminal contract binary | PASS, 3/3 |
| exact-head quick workspace, `profile quick`, 8 workers | PASS, 2,261/2,261; 40 profile skips |
| exact-head full workspace, `profile full`, 8 workers | PASS, 2,310/2,310; 33 profile skips |
| snow/frost domain profile, 8 workers | PASS, 366/366; 1,989 profile skips |
| anti-wiring regression found during full gate | PASS after replacing a forbidden test-only `surface_energy` token; focused 1/1 and full rerun green |
| targeted Clippy, all targets, `-D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS after formatting |
| `git diff --check` | PASS |
| `cargo deny check` | PASS; unmatched MIT-0 allowance warning only |
| assurance v2 validation | PASS; three DRAFT reports, zero public reports |
| authority-suite anti-evasion shell guard | PASS |
| required-suite obligation guard contract | PASS, 3/3 |

An initial quick attempt was externally terminated while cleaning up an
obsolete competing runner. The final exact-head rerun passed all 2,261 selected
tests. Full and snow/frost profiles independently passed all selected tests.
