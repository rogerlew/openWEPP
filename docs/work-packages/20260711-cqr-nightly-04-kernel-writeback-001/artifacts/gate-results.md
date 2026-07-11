# Gate Results

| Gate | Status | Evidence |
|---|---|---|
| Focused nextest | PASS | `42/42` crate tests; independent current writeback filter `11/11`. |
| Science-tier coverage | PASS | `98.344%` lines, `97.543%` regions; all logical functions above `91%`. |
| Target CRAP | PASS | Zero rows above `30`; maximum `9.0`. |
| Focused clippy | PASS | `cargo clippy -p openwepp-kernel-contract --lib -- -D warnings`, exit `0`. |
| Delegated closure round 1 | FAIL/FIXED | Workspace clippy found test `similar_names`/`float_cmp`; names and typed-value assertions fixed. Round 1 full gate results are superseded because source changed. |
| Delegated closure round 2 | FAIL/FIXED | Workspace clippy found one `too_many_lines` test; it was split mechanically into three focused tests. Round 2 remaining results are superseded. |
| Final `cargo fmt --check` | PASS | Delegated round 3 exit `0`; `1.95s`. |
| Final workspace clippy | PASS | All targets, warnings denied; round 3 exit `0`; `10.58s`. |
| Final full workspace nextest | PASS | Round 3 exit `0`; `1709/1709` passed, `4` slow, `3` skipped; `603.03s`; run `8eda74bd-51cd-441a-8699-6200fd18ee05`. |
| Final `cargo deny check` | PASS | Round 3 exit `0`; `1.03s`. |
| `git diff --check` | PASS | Exit `0` after final reconciliation. |
| Package/catalog docs lint | PASS | Package `21` files and catalog `1` file; zero errors/warnings. |

No external-authority suite posture, cohort fixture, or required-case binding
changed; anti-evasion gates are not applicable.

Raw round-3 logs and timings are under
`/tmp/openwepp-cqr-20260711-t04-closure-r3/`; the delegated runner edited no
repository file.
