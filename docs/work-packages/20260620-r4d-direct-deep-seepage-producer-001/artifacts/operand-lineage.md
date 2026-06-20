# R4D Operand Lineage

Status: complete.
Evidence mode: Static.

R4D creates the direct deep-seepage operand consumed by R4B storage
reconciliation. It does not publish `Dp` or edit output schemas.

| Operand | Units | Sign / basis | Authority | R4D role |
|---|---|---|---|---|
| direct deep-seepage handoff `D` | `m` | nonnegative below-root-zone percolation loss depth | `SC-PERC-001` Chapter-5 / WB18 percolation-loss authority | source for direct `deep_seepage_m` |
| `deep_seepage_m` | `m` | nonnegative storage-reconciliation outflow depth | `SC-PERC-001`; `SC-WATBAL-001#WB12 Reconciliation Authority Addendum` | direct downstream operand for R4B |

Anti-alias obligations:

- Do not source `deep_seepage_m` from public `Dp` publication.
- Do not source `deep_seepage_m` from WB19 `Qd`, lateral `q`, or drainage
  `Qdd`.
- Do not source `deep_seepage_m` from ET, snow coupling, precipitation, runoff,
  storage change, R3B diagnostic ledger values, or closure residuals.
- Do not treat the R4D shadow projection as public WB13/WAT/PASS authority.

Pre-implementation conclusion:

The selected handoff is the lowest-risk remaining R4B storage operand producer.
`SC-PERC-001` plus `SC-WATBAL-001` is sufficient for this narrow direct-runtime
handoff. Full WB18 process physics and public `Dp` publication remain out of
scope.
