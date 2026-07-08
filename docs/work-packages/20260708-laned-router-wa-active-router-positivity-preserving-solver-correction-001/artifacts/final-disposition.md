# Final Disposition

Status: EXECUTED-COMPLETE
Evidence mode: Static + Ran.

## Outcome

The package closes the WA active-router positivity-preserving solver correction.
`SC-OFEROUTE-001` rev 41 now binds conservative predictor/corrector stage-face
limiting and final TVD scaling. The implementation keeps water inside the
finite-volume flux update instead of publishing material clamp mass.

WA final evidence:

- `baseline_fixed10`: PASS, total clamp `2.717124262301002e-13 m3`, max
  cascade/seam/identity residuals `1.53e-14` / `3.05e-14` / `5.93e-14`.
- `dx5`: PASS, total clamp `7.305156020320419e-13 m3`, max
  cascade/seam/identity residuals `4.71e-14` / `4.83e-14` / `5.93e-14`.

The retained rev-40 `laned_active_clamp_exceeds_source` guard remains live; WA
passes because the solver no longer creates material clamp amplification.

## Gates

Final gates passed:

- `git diff --check`
- Markdown/doc lint for touched docs
- `cargo fmt --check`
- focused solver branch tests
- D10B / Case-4 focused tests: 19/19
- runner selector tests: 2/2
- WA fixed10 and `dx5` release rerun
- BEI / SC unit / unit registry checks
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`: 1420/1420
- `cargo deny check`

## Follow-On

Target-`dx` promotion is still not made here. The appropriate follow-on is a
Tier-2 target-`dx` mesh-policy re-adjudication on the rev-41 solver, including
WA fine-reference adequacy.
