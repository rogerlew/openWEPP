# Gate results

Status: `COMPLETE — HOLD`

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
| line-count governance | WARN / compliant | 2,721 + 529 lines; follow-up split intent recorded |
| clippy warnings-denied | FAIL / superseded | initial 773-error result; superseded by delegated terminal command below |
| broad orchestrator nextest without required stack env | FAIL/INCOMPLETE / superseded | 1,006 pass, six fail, three skip, 113 not run; superseded by complete profile below |
| independent review A/B | COMPLETE — NO-GO / HOLD | package-local RA-001/RA-003/RB-003..006 closed; RA-002/RB-001/RB-002 open |
| review-correction focused convergence | PASS | 19/19, run `45648dea-a2a7-4272-b5c5-81b0e2764cee` |
| review-correction affected contracts | PASS | 47/47, run `ea732fe3-013a-4ca1-b1cb-3914d5a013ea` |
| terminal generation-37 A0 admission | PASS | v29 `A0_ADMITTED`; 49 contracts / 4 science surfaces; authority `ce2befbd...` |
| terminal focused convergence | PASS | 19/19, run `8bc29a54-ac8f-4922-ad99-604514908b5b` |
| clean-commit canonical one-day | PASS | commit `6953a36b8`; 491/205, 32 caps, zero discrete, unchanged closure, 339.10 s body |
| full workspace correctness profile | FAIL, complete | all 3,628 attempted: 3,503 pass, 125 fail, zero not-run; 5,022.73 s; log SHA `dbdd682a...` |
| package-owned accepted-endpoint source-order correction | PASS | terminal parent run 5/5, `4c65045f-f129-422a-a722-0b0308dd634a`; both verifiers independently pass 5/5 |
| workspace warnings-denied Clippy | FAIL | coupled-time `filter_map_bool_then`; biogeochemistry `similar_names`; log SHA `aac68d69...` |
| terminal workspace source validation | PASS | all targets/all features |
| terminal formatting/diff hygiene | PASS | `cargo fmt --all -- --check`; `git diff --check` |
| terminal authority anti-evasion | PASS | source guard; obligations 3/3, run `b502b010-009d-48df-848d-32f7df176752` |
| independent verification A/B | COMPLETE — HOLD | both verify package-local closure and retain failed mandatory gates |

The bounded package-specific gates pass, but the recorded lint and complete
workspace-regression failures are closure-blocking. No sole-blocker,
technical-pass, waiver, or package `COMPLETE` claim remains.

Heavy-gate artifacts are
`/tmp/stage3_fp_cold/review_correction_heavy/nextest_full.log` and
`clippy.log`. The complete test profile has zero not-run tests but is not a
pass. Three of its seven orchestrator failures were package-owned stale
source-scan bindings and now pass focused; two are known snow-free LSE
assertions and two are long-fixture timeouts. The remaining failures are in
assurance, runner/CLI, and other workspace surfaces outside this package's
authorized write set. No authority exists here to relabel them as passing.
