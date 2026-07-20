# Review Finding Disposition

Evidence mode: `Static + pending verification`

Status: `all findings accepted; fixes in progress`

| Finding ID | Source | Severity | Decision | Action taken / required | Artifact ref | Rationale |
|---|---|---|---|---|---|---|
| A-01 | agent_a | high | accepted | Add a no-transfer first-realization state and reject aggregate `vdmt` seeding. | corrected implementation and tests pending | Foliar and aggregate live biomass are distinct contract operands. |
| A-02 | agent_a | high | accepted | Keep contracts draft until both verification agents pass, then promote all three together. | verification artifacts pending | Contract promotion logic is binding. |
| A-03 | agent_a | high | accepted | Add real-run runtime-value assertions across all named consumers. | consumer proof pending | Source order and execution success are insufficient consumer evidence. |
| A-04 | agent_a | medium | accepted | Replace the half-year-only test with full wrapped NH/SH canopy and limb-phase checks. | contract test pending | The operator selected a phase transform of a complete NH climate. |
| A-05 | agent_a | medium | accepted | Enforce finite positive `bb` through schema, projection, and kernel. | negative tests pending | Matches ratified CP-GSI02 domain. |
| B-01 | agent_b | high | accepted | Withdrew prototype in `0692cec7`; corrected intent must be admitted before reimplementation. | `artifacts/intent-plan.md` | Terminal planning cannot retroactively authorize edits. |
| B-02 | agent_b | high | accepted | Require native phenology schedules to use the continuous `jdplt=0`, `jdstop=0` forest window. | schema/projection tests pending | GSI must advance on every chronological climate day. |
| B-03 | agent_b | high | accepted | Same fix as A-05. | negative tests pending | Duplicate independent finding confirms severity. |
| B-04 | agent_b | high | accepted | Same fix as A-03 and correct the evidence artifacts. | consumer and conservation evidence pending | Closure verbs must match dynamic evidence. |
| B-05 | agent_b | medium | accepted | Reject every negative derived VPD; do not clamp. | runner negative test pending | No bounded-normalization authority exists. |
| B-06 | agent_b | medium | accepted | Add full transformed-calendar checks and bit-identical periodic endpoint/transfer assertions. | contract tests pending | Matches package acceptance wording. |
| B-07 | agent_b | high | accepted | Replace exact float comparisons and rerun a fresh exact-head terminal plan after promotion. | terminal receipt pending | Failed and blocked gates cannot be deferred. |

No finding is rejected, deferred, or silently closed.
