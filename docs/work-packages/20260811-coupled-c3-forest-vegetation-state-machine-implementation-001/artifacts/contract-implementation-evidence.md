# Contract Implementation Evidence

Status: `executing / V4 shared-state contract surfaces PASS; capped path incomplete`

Increment 2B adds only a non-constitutive typed water request/authorization
boundary. No radiation or potential solver output was retained; the exact
authority omissions are recorded in `potential-pass-hold-legitimacy-audit.md`.

Evidence mode: `Static + Ran`

Static: the public state is V2-only: shared stratum C/N and phenology state is
separate from exact `(stratum,tile)` occupancy lanes. Strict validation binds
the released V2 model, configuration digest, complete state digest, exact
occupancy/root identity, every occupancy field, pending transfers, and
transaction lineage.

Ran: focused tests cover exact two-tile/two-stratum state, duplicate/missing/
extra/wrong occupancy, all 15 lane fields in the state digest, layer order and
cardinality, unit spelling, V1 parser rejection, initial/prior transactions,
and every admitted migration branch. Public execution remains explicitly
fail-closed before E04; no E01--E22 public-path or commit claim is made.

Static: `column.rs` now owns deterministic tile/vertical ordering, exact
`LAI_s/C_s` and `WAI_s/C_s`, heterogeneous tile-rain identity, descendant
routing of throughfall plus initial and second drainage, same-tile stemflow
bypass, exact authorization back-conversion plumbing, and one-time local-water
weighting. The callback supplies constitutive results; it cannot supply an
accepted closure residual.

Ran: controlled routing vectors pass for empty, one-occupancy, two-rank,
condensation/second-drainage, tile isolation, tile-order invariance, area/store/
aggregate-first poisons, fixed-cap identity, local/column/weighted closure, and
byte-identical injected failure. This is topology evidence only. Exact
occupancy-local radiation and E11--E15 potential/capped solves remain pending.

## V4 Contract Surface

Ran: V4 executable identity, recursive exact state shape, displayed-leaf C/N
ownership, structural canonical serialization, 155 independent mutation
digests, typed pending-transfer identities, and explicit V3-to-V4 migration all
pass focused tests and independent runtime review. The two V3 offset fields are
historical migration input only and are not executable V4 state.

The production uncapped occupancy evaluator and typed potential-request seam
remain available, but the public transaction is fail-closed before capped
finalization. No digest-bound fully coupled cap-active vector currently fixes
the equality active set or independent `q_law`/cap operands. Therefore the
capped draft is not contract evidence and `STAGE_B_E11_E15_EXACT_ORACLE`
remains incomplete.
