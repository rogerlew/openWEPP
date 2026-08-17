# Rust Correctness Review At `f249431d4`

Evidence class: `Static + Ran`

Verdict: `HOLD`

A fresh reviewer inspected exact clean commit
`f249431d409afd2de169cba544db64b75698c792` against the package baseline and
accepted two material findings.

1. `A-TERMINAL-F249-HIGH-001`: configuration/restart validators and outer
   public seams interleave per-record identity and domain checks. An earlier
   record with a NaN or capacity defect can therefore return E003 before a
   later record's E002 identity violation, contrary to the contract's global
   precedence.
2. `A-TERMINAL-F249-MEDIUM-002`: checked mass-to-depth and receipt aggregation
   is substantially duplicated between receiver preflight and final receiver
   construction, creating drift risk in arithmetic, conversion, guard and
   classification behavior.

The reviewer rechecked the `2e32a8a0e` raw-mass/source reconstruction defect
and found it corrected. Canonical-last unsafe proportional representability
correctly fails closed as E003 under v6 and is not a defect or authorization to
normalize.

Formatting passed. A full-workspace attempt was stopped after the material
HOLD was confirmed: 176 passed, 19 interrupted and 2,688 did not run. It is
preserved as interrupted evidence, not terminal validation.
