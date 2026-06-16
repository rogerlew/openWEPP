# CQR35 Behavior Equivalence

Status: complete.

Static: no production Rust file was modified. Behavior equivalence holds by
construction for public API, private helper behavior, runtime symbols, formulas,
typed guards, parser compatibility, output surfaces, and science contracts.

Ran: before and after package LCOV/CRAP runs both executed the full workspace
test set under coverage and passed.

Ran: before and after target-file CRAP and LCOV values match exactly:

| Surface | Before | After |
| --- | ---: | ---: |
| Highest target-file CRAP | 26.541362973760947 | 26.541362973760947 |
| Target-file rows above CRAP `30` | 0 | 0 |
| Target-file covered lines | 1698 | 1698 |
| Target-file instrumented lines | 2122 | 2122 |

Static: because no production edit was made, no comparator delta review was
needed for behavior preservation beyond the required package metrics and gates.
