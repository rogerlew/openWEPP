# Gate results

Status: `TECHNICAL-PASS / REVIEW-HOLD`

Evidence mode: `Ran`

| Gate | Result | Evidence |
|---|---|---|
| preimplementation contract red | PASS | two expected missing-helper failures |
| focused covered convergence policy | PASS | 19/19 nextest |
| affected contract consumers | PASS | 47/47 nextest |
| canonical one-day qualification | PASS | 491/205, 32 caps, zero discrete rejections |
| ledger and receipt closure | PASS | all unchanged bounds satisfied |
| formatting | PASS | `cargo fmt --all -- --check` |
| workspace source validation | PASS | all targets/all features |
| science-contract admission | PASS | `A0_ADMITTED` |
| authority-suite anti-evasion | PASS | source guard passed; required-suite obligations 3/3 |
| diff hygiene | PASS | `git diff --check` |
| line-count governance | PASS | 2,726 + 529 line split |
| clippy warnings-denied | FAIL (pre-existing backlog) | 773 crate-wide errors; no new-helper warning identified; `/tmp/stage3_fp_cold/clippy-final.log` |
| broad orchestrator nextest without required stack env | FAIL/INCOMPLETE | 1,006 pass, six fail, three skip, 113 not run; three stack overflows, one unrelated snow-free LSE failure, two canceled long tests |
| independent review A/B | BLOCKED | higher-precedence no-subagent policy |
| independent verification A/B | BLOCKED | higher-precedence no-subagent policy |

The bounded package-specific and exact-head critical gates pass. The two broad
observations are not represented as passes; the required stack environment was
used for package qualification. Independent evidence remains the sole package
completion blocker.
