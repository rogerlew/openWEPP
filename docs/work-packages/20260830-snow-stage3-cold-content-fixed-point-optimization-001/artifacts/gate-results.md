# Gate results

Status: `REVIEW-CORRECTION-ACTIVE`

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
| clippy warnings-denied | FAIL / rerun pending | initial 773-error result is closure-blocking; `/tmp/stage3_fp_cold/clippy-final.log` |
| broad orchestrator nextest without required stack env | FAIL/INCOMPLETE / superseding full profile pending | 1,006 pass, six fail, three skip, 113 not run; this does not satisfy critical regression |
| independent review A/B | NO-GO, findings accepted | RA-001..003 and RB-001..005 |
| review-correction focused convergence | PASS | 19/19, run `45648dea-a2a7-4272-b5c5-81b0e2764cee` |
| review-correction affected contracts | PASS | 47/47, run `ea732fe3-013a-4ca1-b1cb-3914d5a013ea` |
| review-correction A0 admission | PASS | v29 `A0_ADMITTED`, authority `a8828192...` |
| independent verification A/B | PENDING | dispatch only after accepted findings and exact-source gates are reconciled |

The initial bounded package-specific gates pass, but the recorded lint and
broad-regression failures are closure-blocking and independent review is
`NO-GO`. No sole-blocker or technical-pass claim remains. Clean-source
canonical, full-profile, warnings-denied, re-review, and verification evidence
must supersede this active table before final disposition.
