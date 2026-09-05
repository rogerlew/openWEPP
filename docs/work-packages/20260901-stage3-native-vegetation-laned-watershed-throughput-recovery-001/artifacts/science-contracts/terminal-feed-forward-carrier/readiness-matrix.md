# Kernel-profile and readiness matrix

Evidence mode: `Static + Ran + Expected-red`

Contract set: `SC-SNOWENERGY-001` revision 61,
`INV-SNOWENERGY-088`, `OBL-SNOWENERGY-C-056`.

```text
science_implementation_status = REJECTED_AND_REVERTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

| Profile obligation | Status | Evidence and rationale |
|---|---|---|
| Canonical authority and registry | `PASS` | Contract and index agree on revision 61, active lifecycle, date, and no process-solver V61. |
| Source-real dependency | `PASS` | Real carrier consumes no ending hint or coupling ordinal; focused capture proves two generic-loop calls are bit-identical with zero deltas. Exact release directly proves only 400 pre-change calls; 200 groups remains an inference until postimplementation measurement. |
| Algorithm/state surfaces | `PASS` | The exactly-one rule is local to one evaluator invocation. Distinct step-doubling, retry, root, discovery/exact outer invocations, batch, and final paths cannot alias. |
| Branch and error precedence | `PASS` | The Guard Map binds actual outer, provider-wrapper, carrier, boundary-join, transition, exact, and final boundaries to existing typed variants without hoisting validators or adding fallback. |
| Conservation/custody | `PASS` | Every retained carrier call performs unchanged complete physics, receipt, ledger, WB14, owner, and rollback validation. |
| Units/constants/tolerances | `NOT_APPLICABLE` | No dimensional, parameter, constant, equation, tolerance, or convergence change. |
| Calibration/identifiability | `NOT_APPLICABLE` | No parameter, observation, objective, or calibration claim. |
| External constitutive suite | `NOT_APPLICABLE` | No constitutive family or required-case change. |
| Contract-derived assertion | `PASS` | Focused revision-61 authority test passes. |
| Executable production seam | `EXPECTED_RED` | The structural production seam is absent. This source assertion is not behavioral acceptance authority; package-owned in-crate CQR tests and negative-capability evidence must execute after implementation. |
| Full contract target isolation | `PASS_WITH_EXPECTED_RED` | Complete target: 39 pass, exactly the named revision-61 structural seam fails, 22 historical superseded-path tests ignored; no stale revision pins or unrelated failures. |
| Performance retention | `FAIL_REVERTED` | Source `650f6713…` / binary `e6b57efa…` passed timing medians at `1,032,416/3,913,131 us` and exact science/count/200-invocation multiset identity, but RSS `69,768/59,504/70,484 KiB` failed the per-run `65,536 KiB` ceiling twice. Production and telemetry were fully reverted. |
| Binding Exposure Index | `PASS` | Strict lint passes all 51 consolidated rows; core summaries register both the pre-existing `INV-087/C-055` companion and revision-61 `INV-088/C-056`. |
| Independent review | `PASS` | Both reviewers reproduced corrected manifest `a8a667...804`, closed all seven findings, and issued `APPROVE` / `GO`. |
| Independent verification | `CONTRACT_GO_ONLY` | Both verifiers independently reproduced the preimplementation manifest and issued implementation `GO`. The later prospective runtime gate failed and overrides that permission with mandatory production reversion. |

## Obligation-to-test binding

| Canonical obligation | Required executable binding |
|---|---|
| Exact one-call invocation | pre-change 400-call evidence; post-change exact 200-invocation role/path multiset; exactly one provider call per evaluator invocation |
| Full reference parity | every role/path compares bitwise with both forced two-call reference outputs and exact-zero deltas |
| Distinct evaluation chronology | Full/Retry/Half1/Half2/Root plus separate discovery/exact outer invocations, single/batch paths, and same-map final completion remain distinct |
| Typed mode | production request has no feedback fields; feedback-capable provider cannot be selected dynamically |
| Poison/error/rollback | sole-call and exact/final poisons, adjacent competing binding poisons, cross-invocation refusal, unchanged precedence, byte-identical rollback |
| Real consumer | exact science/output/closure, `48/56/20/32/4`, publication chronology, ordinary canonical and batch cadence |
