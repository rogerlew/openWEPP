# Terminal diagnostic correlation V6 numerical/evidence/cardinality review

Recommendation: **HOLD / NO GO-to-evidence**

Evidence class: `Static`. Independent read-only review; no edits and no contact
with the Rust reviewer. All 13 frozen manifest hashes matched exactly.

## Findings

1. **Critical:** most evidence leaves lack compiler bindings. The 60 rows cover
   state, ledger, stack and carrier fields but not support/key/role identity,
   coupling/selection assembly, selected-trial joins, pair calculations,
   admission/floor operands, three ingress expressions, outer error variant or
   caller-local joint/clock/counter.
2. **Critical:** collection cardinality is not typed. Admissions, arenas,
   iterations, selections, selected trials, pair decisions, iteration keys and
   five component errors are singular nested DTO fields with prose ordering.
3. **Major:** constraints are inert strings; the tool checks only nonemptiness.
   It does not enforce role mappings, exact counts, floor arithmetic, provider
   equality, ingress +0.0, fold/winner semantics or cross-record identities.
4. **Major:** the sufficiency matrix omits necessary floor operands and exact
   refined-trial/receipt reconstruction. Noninterference rests on an uncomputed
   `unchanged` boolean.
5. **Major:** `DiagnosticF64V6` does not bind finiteness to decoded bits, and
   error equations/fold/winner lack checked derived-expression bindings.
6. **Major:** the three ingress DTO fields have no resolved source expressions
   or availability points.

Exact `TerminalState` and `TerminalLedger` field sets, the narrow inner outcome
witness and explicit coarse/fine scalar slots are accepted directionally.
