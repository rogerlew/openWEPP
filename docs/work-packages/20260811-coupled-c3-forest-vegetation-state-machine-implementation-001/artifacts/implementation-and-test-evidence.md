# Implementation And Test Evidence

Status: `executing / Milestone 4 closed / Increment 4B receiving owners active / public candidate fail-closed`

## V7 Storage-Transfer Phenology Increment

Static + Ran: active runtime identity is the exact released V7 definition.
Historical V5-to-V6 migration is frozen to V6, while the new V6-to-V7 boundary
validates complete source/target receipts, preserves seasonal nonidentity bytes,
does not execute onset, and returns exhaustive evergreen unresolved fields.

Static + Ran: `prepare_storage_for_onset` is pure and moves independently for
all six tissues and both elements exactly half of immutable beginning storage
into existing transfer. `advance_phenology` prepares only on the strict upward
Dormant-to-Onset edge, deploys every tissue, uses the exact terminal remainder,
and enters Active only when all twelve transfer values are exact zero. It
commits its caller-owned tissue map only after validation and closure pass.

Ran: vegetation 203/203, implementation contract 13/13, strict all-target
vegetation Clippy, formatting, and diff hygiene pass. Public persistent mutation
and the all-owner candidate remain intentionally unavailable.

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

## V5 Authority Intake And Active Remediation

Static + Ran: authority predecessor commit
`b7e6f08b655452c5c59a498ac9becd1439dd21ef` released
`OPENWEPP_C3_WOODY_V5` definition
`0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`,
independent vectors
`6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d`,
and generator
`4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775`.
Both authority science reviews returned GO, the focused/heavy authority gates
passed, and both terminal verifiers returned PASS.

Static + Ran: V6 runtime identity, exact V5-to-V6 snapshot/diagnostic migration,
the rejected-failure typed category, and the tightly firewalled rtol-only
`step_norm` comparison pass focused tests. Two implementation-review rounds
found material evidence and migration defects; all are preserved, accepted,
and remediated, with another independent review pending. Authority release and
this portability evidence are not public-path implementation evidence:
`STAGE_B_E11_E15_EXACT_ORACLE`, finalized-use debit, Milestones 2/3, and the
public candidate remain incomplete/fail-closed.

## V6 Public Water-Phase Integration

Static + Ran: `water_phase::execute_uncommitted_water_phase` now performs the
complete public E01--E15 water sequence: exact input validation, whole-column
radiation, owner-uncapped potential columns, typed potential request batch, one
hydrology authorization call, immutable typed authorization validation, and a
fixed-cap rebuild from the original beginning state. It exposes typed D/A/F,
potential/final columns and radiation, complete diagnostics, capped operands,
and a receiving-owner water candidate.

The water owner constructs its own candidate debit. Vegetation independently
reconstructs the exact resource protocol, key set, per-layer aggregate maximum
authorization against the typed immutable beginning snapshot, the required
per-request authorization reason, finalized debit, and ending store. Only
finalized use is debited. Shared-layer aggregation uses one canonical sorted-key
sum and one subtraction in both the receiving owner and independent validator.
The sealed phase has no conversion
to `CoupledCandidate` and no commit method; occupancy accepted lineage remains
unchanged. `execute_candidate` consumes this same stage and then returns a typed
E16--E22/multi-owner implementation-incomplete error.

The first production-stage run rejected because the capped owning validator
incorrectly attempted to reconstruct vulnerability-demand residuals as
`beta * Emax`. The solver itself was unchanged. The runtime diagnostic seam now
carries the exact hydraulic vulnerability-demand operands outside the frozen
V5/V6 failure serialization, and the independent validator reconstructs the
two residuals from those operands. The failed attempt and correction are
preserved in `gate-results.md`.

Ran after first-review remediation: public water phase 4/4, vegetation quick
184/184, implementation contract 13/13,
affected vegetation/orchestrator strict all-target Clippy, formatting, and diff
hygiene pass. Public integration review and remaining multi-rank production
vectors are pending; E16--E22, energy/BGC candidates, atomic commit, runtime
activation, and terminal completion remain unclaimed.

## V7 E19 Final-Demand Ordering Correction

Static: SC-VEGETATION-001@11 contains no
`Ndem_final<=Ndem_pot` invariant. Its canonical finalization retains the
potential request batch, caps `Fext` by `Asum`, distributes `F_N`
proportionally, computes common `eta` from actual internal plus external use,
and retains unsupported final carbon in NSC. SC-BIOGEOCHEM-001 independently
requires `F<=A<=D`.

Static + Ran: the implementation-only ordering errors and guards are removed.
`persistent_phase` prepares every stratum request before one global arbitration
call, validates exact returned authorizations, finalizes use once, and passes a
receipt containing exact final demand/internal use/external use/internal
remainder to six-tissue growth. Growth independently reconstructs demand from
the final carbon offer, applies one common eta, updates the uncommitted
candidate, and closes carbon and nitrogen. It does not recompute external use
from authorization.

Ran: the actual full-water fixture preserves exact potential/final carbon and
N-demand values separated by two ULPs. It makes one nitrogen-authorizer call,
does not modify the potential requests, retains the final demand without clamp,
obeys every layer/species `F<=A<=D` bound, produces `eta<1` and positive NSC,
and leaves serialized beginning vegetation bytes unchanged. The public path
still returns typed post-nitrogen implementation-incomplete and cannot publish
or commit a multi-owner candidate.

## V7 Increment 4A Sealed Vegetation Candidate

Static + Ran: `vegetation_candidate::construct_uncommitted_vegetation_candidate`
consumes the sealed water and nitrogen phases, requires exact phase/beginning/
transaction identity, takes occupancy warm starts only from the final capped
column set, advances accepted lineage, and reconstructs all shared derived
areas from ending displayed leaf C. It computes and validates a new canonical
V7 state digest. Beginning state remains byte-identical.

Material amounts are sorted by typed stratum, donor, receiver, and original
source sequence, then receive positive deterministic proposal IDs. They remain
candidate escrow outside the ending state; no unresolved pending transfer can
enter the next accepted state. `vegetation_ledger` independently reconstructs
per-stratum carbon, nitrogen, and dry-material obligations without consuming a
producer residual. Missing/potential-only/duplicate occupancy results,
carbon-as-dry-matter, forged dry aggregate, elemental imbalance, and cross-
ledger identity poisons reject. Post-review remediation separates physical
vegetation C from signed `XS_C`, consumes the directly retained maintenance
operand, uses the canonical whole-owner closure bound, binds every ledger to
the exact candidate transaction and beginning/ending digests, and requires the
exact configured stratum set with globally unique proposal IDs.

Ran after review remediation: `cargo nextest run -p openwepp-vegetation
--profile quick` passed 223/223. The real two-ULP candidate now rejects ending
`XS_C` corruption, a coherently forged ending digest, and a carbon-as-dry-
matter proposal while retaining exact beginning bytes. A distinct capped water
phase cannot be paired with the retained nitrogen phase.
Ran: `cargo clippy -p openwepp-vegetation --all-targets -- -D warnings` passed.
The public function constructs and validates this sealed candidate, then still
returns `V7 post-nitrogen multi-owner candidate is implementation-incomplete`.
Final exact-byte correctness review returned GO and QA returned PASS after the
initial HOLD findings and all remediation attempts were preserved. This closes
Milestone 4; receiving BGC/energy candidates and atomic commit remain
Milestone 5.

## Increment 4B BGC Receiving Owner

Static + Ran: `construct_biogeochemistry_candidate` replaces the former API
that accepted caller-supplied proposals and receipts. It accepts typed
potential requests, maximum authorizations, finalized uses, and vegetation
material proposals, then independently constructs exact receipts, species- and
layer-preserving mineral debits, receiver credits, ending state, and explicit
mineral plus C/N/dry-material receiver operands. It validates the owner
candidate before return and retains `TransformationsMode::Required` as
`BGC-E-040`.

Ran: BGC quick passed 5/5; hillslope-orchestrator quick passed 490/490; strict
all-target BGC and hillslope Clippy and affected checks passed. Poisons cover
duplicate proposal identity, wrong-species finalized use, missing receiver,
unsupported transformations, and authorization distinct from finalized debit.
The independent energy owner and sealed V7 cross-owner connection remain
pending, so Increment 4B and Milestone 5 are not closed.
