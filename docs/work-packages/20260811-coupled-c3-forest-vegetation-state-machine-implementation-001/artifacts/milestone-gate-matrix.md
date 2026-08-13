# Milestone Gate Matrix

Status: `executing / V6 Stage-B focused gate passed; Milestones 2 and 3 incomplete`

The former Stage-B portability blocker is lifted. The focused E11--E15
equation/ownership gate passes under V6. Milestones 2 and 3 remain incomplete
until the real public water transaction invokes the potential and fixed-cap
column passes and exposes the resulting finalized ownership operands.

| Milestone | Direct current-scope gates |
|---|---|
| 0 | required reading/write set/artifact freeze; A0 admission; model and contract schema/digest checks |
| 1 | identity/schema/state/migration unit, negative, round-trip, and serialization tests |
| 2 | E01--E06 oracle, topology, water/energy closure, poison, typed-domain tests |
| 3 | E07--E15 oracle, solver, competition, nonconvergence, and rollback tests |
| 4 | E16--E22 oracle, C/N/DM closure, phenology trajectory, ownership tests |
| 5 | request/authorization/final use, receiving state, injected-failure atomic rollback tests |
| 6 | default-off real diagnostic harness, unchanged-selector negative proof, benchmarks, A1/A3, focused and Critical terminal gates |
| terminal | dual reviews/disposition, exact diff/line count, delegated full workspace, dual verification, prompt archive |

No row may pass on artifact presence or later evidence.

Increment 2A closes only the internal topology/routing sub-gate: controlled
occupancy results prove ordering, routing, area conversion, closure, poisons,
and rollback. Milestone 2 still requires exact occupancy-local E01--E06 oracle
evidence, and Milestone 3 still requires `STAGE_B_E11_E15_EXACT_ORACLE`.

V4 closed the identity/state/migration sub-gate: exact displayed-pool
ownership, recursive schema, structural serialization, all 155 mutation
digests, and explicit V3-to-V4 migration pass. It does not change the
Milestone 2/3 result. At that checkpoint the capped Stage-B gate remained
incomplete for lack of a digest-bound fully coupled cap-active equality/operand
vector; V5 supplies that authority but still requires implementation evidence.

V5 supplied the exact cap-active vectors and V6 admitted the single rejected
`step_norm` portability rule. Production Rust now reconstructs the accepted,
singular, iteration-limit, and backtracking vectors; independent bounded
correctness and QA reviews pass; and the internal final-column pass proves
finalized-use identity, rerouting, `F<=A<=D`, diagnostics, and rollback.
Therefore `STAGE_B_E11_E15_EXACT_ORACLE` is PASS. Milestones 2 and 3 remain
incomplete until that accepted final pass is consumed by the real public column
transaction.
