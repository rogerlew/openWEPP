# Operand Lineage

Status: complete.
Evidence mode: Static.

R5E does not introduce new public publication operands. It validates the full
direct endpoint over the typed state and shadow projections completed in R3, R4,
and R5A-D.

| Surface | Authority in R5E | Publication disposition |
|---|---|---|
| Canonical phase entries | `DirectPhaseKind::ORDERED` and direct executor report | Internal direct-runtime evidence only. |
| Direct sub-operation counters | R3/R4/R5 span reports | Internal lifecycle evidence only. |
| Shadow projections | Existing R3/R4/R5 span outputs | Validation evidence, not public output authority. |
| HBP/WAT/PASS/loss/manifest | Compatibility publication path | R6 owns cutover. |

Conservation/publication acceptance is not waived; it is intentionally deferred
to R6 because no public output authority changes in R5E.

R5E endpoint evidence therefore uses protected output identity/equivalence as a
regression guard only. It does not make HBP/WAT/PASS/loss/manifest readers
consume direct projection operands.
