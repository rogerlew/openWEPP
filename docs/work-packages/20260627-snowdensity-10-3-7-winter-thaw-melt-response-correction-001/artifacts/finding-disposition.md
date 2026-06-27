# Finding Disposition

Evidence mode: Static.

| Finding | Source | Disposition | Rationale |
|---|---|---|---|
| Candidate improves but does not fully close thaw under-ablation. | Review A/B | Accepted | Documented in package, strategy, README, and worker handoff. No activation claim is made. |
| Candidate event-window count differs by one from legacy. | Review B | Accepted | Event classification depends on modeled depth trajectory. The package gate compares under-ablation count and aggregate deficit from each model's paired thaw-ablation profile and both improve. |
| Need default/rollback isolation. | Package gate | Accepted | Focused tests prove legacy low-density behavior remains; diagnostic report records default activation false and no parser/runfile/user selector. |
| Conservation proof missing from first closeout. | Operator review | Accepted/resolved | Added v94 contract language, emitted retained/released-rain and residual operands, and reran the report. Candidate active-ledger residuals are zero for SWE balance and routed state loss. |
| Coupled WAT snow-control gate missing from first closeout. | Operator review | Accepted/resolved | Added `OPENWEPP_SNOWDENSITY1037_MELT_MODEL` as a package-bound diagnostic selector, ran real direct-production WAT, and recorded improvement from `1147 -> 978` failures with zero paired surfaces worsening. |
| Coupled snow-control still not cleared. | Coupled WAT report | Accepted/follow-up | Candidate remains opt-in only. Residual blocker is `SNOW-CONTROL-NOT-CLEARED`; no frost-unblock or default activation claim is made. |

No blocking findings remain open.
