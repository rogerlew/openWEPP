# Coverage After

Ran: the exact clean implementation HEAD `dc935c7a` was measured once with the
same five-test binary LCOV profile as the baseline. All 5/5 tests passed in
20.81 seconds. Production is lines 1-1,090 and reports 197/807 lines (24.41%)
and 318/1,402 regions (22.68%).

This is an assessment, not ADR-0021 module coverage closure. The package added
no module characterization test and did not materially change one; two static
source-contract strings were updated only to follow relocated expressions. The
module was already below the glue-tier closure threshold at baseline. Helper
extraction expanded the production denominator while the unchanged test profile
remained fixed. No threshold pass or coverage improvement is claimed.

Among 63 compiled production functions, 52 remain below 75% region coverage
and 40 are at zero; two additional functions are expected non-current-cfg
variants. The complete function record is retained in
`main-function-regions-after.tsv`.

Evidence root: `/tmp/cqr-main-changed-97k1yM`. Source SHA-256:
`613df779b6dddffd2fac3079859d47acb33451ad6b124efbbd027265cb136fdd`.
