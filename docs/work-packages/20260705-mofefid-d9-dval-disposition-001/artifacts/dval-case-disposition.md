# D-val Case Disposition

Status: executed
Evidence mode: Static + Ran

## Inputs

Static:

- Case definitions and cut-points remain the D01/D8 definitions:
  `docs/work-packages/20260702-mofefid-d01-ofe-routing-scaffold-001/artifacts/`
  and `tools/dval/compare_dval.py`.
- Comparator target is `NS_trace` against digitized enhanced-WEPP model traces
  from `Figure_4.xlsx`, not paper `Ef_obs` against nature.

Ran:

- Subagent `comparator_suite_runner` (`Lagrange`) re-ran Cases 1-3 and the
  Case 2 `Ks=10 mm/h` sensitivity. Logs:
  - `artifacts/dval-0-20260705-153308.log`
  - `artifacts/dval-0-20260705-153312.log`
  - `artifacts/dval-0-20260705-153313.log`
  - `artifacts/dval-10-20260705-153314.log`

## Verdicts

| Case | Current metrics | Verdict | Rationale |
|---|---|---|---|
| 1 bare | `NS_trace=0.868483`; peak ratio `1.066`; rise `4999.7 s` vs reference `3579.9 s` | `operand-limited` | D8/D9 evidence attributes the slow limb to Green-Ampt operand uncertainty, not routing celerity. Steady magnitude is close, but shape does not cleanly satisfy the D-val surface. |
| 2 isolated roughness | default `Ks=20 mm/h`: `NS_trace=0.453954`, peak ratio `0.747`; plausible `Ks=10 mm/h`: `NS_trace=0.961209`, peak ratio `0.922` | `operand-limited` | The shortfall is materially controlled by uncertain sandy/gravel `Ks`; a plausible lower `Ks` reproduces the trace without changing form/wave kernels. |
| 3 vegetation | `NS_trace=0.537727`; peak ratio `0.547`; timing near the reference peak | `comparator-surface boundary` | The enhanced trace peak remains above the recorded rainfall-length ceiling under D01 operands. Treating that as a kernel defect would tune toward an inconsistent comparand. |

## Disposition

Static + Ran: Cases 1-3 do not become clean `satisfies` cases, but their
non-numerics dispositions are closed for D9. D10 does not own Cases 1-3.
