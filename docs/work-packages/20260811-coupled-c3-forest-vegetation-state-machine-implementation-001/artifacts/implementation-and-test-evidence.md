# Implementation And Test Evidence

Status: `executing / V4 state runtime focused PASS / capped path fail-closed`

Increment 2B stopped before constitutive implementation. The retained
`occupancy_solver::resources` module constructs and validates complete typed
potential request batches and authorizations but does not calculate demand.

Evidence mode: `Static + Ran`

The executable state surface now implements V2 identity, configuration,
occupancy-local state, canonical digest binding, transaction lineage, and
offline V1/RHESSys migration. Historical shared liquid and hydraulic warm
starts are reachable only through the explicitly named V1 migration DTO.

The public candidate validates complete V2 state and then returns a typed
implementation-incomplete error before mutation or publication. Disabled V1
transaction physics was removed rather than retained beside the V2 state.
Internal E04 tile-column routing is now implemented through an
`OccupancyPassSolver` seam. It consumes immutable V2 state, derives conditional
plant area, routes only within a tile, exposes authoritative water operands,
and independently rejects closure/identity/basis poisons. It also proves that
an injected descendant failure cannot mutate any beginning lane.

The callback in this increment is controlled test machinery, not production
physiology. Exact potential and capped E11--E15 occupancy solves, hydrology
arbitration, owner candidates, the public E04 path, and commit remain pending.

## V4 Shared-State Runtime Evidence

Static: executable model/configuration/state identity is V4-only at definition
SHA-256 `8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`.
Strict state parsing validates recursive record shape before typed decoding,
then validates exact membership, tissue domains, displayed area caches,
occupancy lanes, pending-transfer identities, and transaction lineage.

Static + Ran: `OPENWEPP_V4_STATE_CANONICAL_V1` is implemented outside the
authority generator. It matches the released shared-state digest and all 155
independent whole-state mutation digests. Displayed leaf C alone derives LAI,
stem area, and root area. Displayed leaf N alone supplies positive-LAI FvCB and
leaf Rd inputs; leaf storage/transfer N cannot create a second maintenance
debit.

Static + Ran: the V3-to-V4 migration uses strict historical DTOs and returns
`Complete` only after complete source identity/digest/domain/membership/
lineage/area/transfer validation, unchanged constitutive-payload comparison,
removal of exactly two obsolete offset fields, V4 rebinding, digest
reconstruction, and target validation. Every invalid owner is reported
deterministically and no partial candidate is returned. Direct V1/V2-to-V4
normalization is unavailable.

Ran: vegetation quick 159/159, implementation contract 11/11, vegetation
authority 17/17, strict vegetation and hillslope all-target Clippy, formatting,
and diff hygiene pass. Independent Rust review returned GO with one accepted
Medium validation-duplication/decomposition item; independent QA returned PASS
with no material finding after remediation.

The potential occupancy evaluator remains reviewed positive-path/failure-vector
foundation. The authorization-capped evaluator remains a disconnected draft:
the authority lacks a digest-bound fully coupled cap-active vector fixing the
active-set equality convention and independent `q_law`/cap operands. Public
execution therefore remains fail-closed, `STAGE_B_E11_E15_EXACT_ORACLE` is
incomplete, and no Milestone 2/3, finalized-use, owner-candidate, or commit
claim is made.
