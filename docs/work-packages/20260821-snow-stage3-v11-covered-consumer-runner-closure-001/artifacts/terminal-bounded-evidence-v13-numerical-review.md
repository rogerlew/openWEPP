# V13 numerical/evidence/cardinality review

Disposition: `HOLD`.

The reviewer independently verified authority SHA-256
`b7e97d5b966d5d2ec790e9780a03eefe1b8b8234d2ae1c6d49b7647229ab85e0`.

Critical finding: the proposed five-component pair fold does not match the live
decision. Live source folds, in order, ice, liquid, cold content, complete
energy and unallocated energy. V13 instead named complete energy, external
liquid, melt, refrozen and cold-energy change. The latter are ledger and
effectivity operands and must be separate from the exact live `[...; 5]`
decision array. As written, V13 would validate a different estimator and cannot
prove the real rejection, maximum or first-bitwise-equal winner.

The owner-local receipt/ingress DTO fields and exact noninterference comparison
surface are also not closed enough for executable omission, substitution and
cardinality poison tests. No source expansion is authorized.
