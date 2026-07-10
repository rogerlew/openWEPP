# WSHED-W11A Channel-Hourly Sediment Sequencing Authority

Status: `EXECUTED-COMPLETE-AUTHORITY` (2026-07-10; see
`artifacts/final-disposition.md`)

Package ID: `20260710-wshedw11a-channel-hourly-sediment-authority-001`

Queue row: `WSHED-W11A`

Evidence mode: `Static + Ran` (per-artifact labels)

## Objective

Close `WSHED-W11-HOLD-001` by acquiring and ratifying, or explicitly rejecting,
canonical process authority for time-resolved channel sediment routing when
watershed water routing carries a `dtchr` discharge series and HBP carries the
paired hourly `V_h/S_h` surfaces.

The package must define enough authority for WSHED-W11 to implement channel
sediment routing without inventing how WS18-WS26 state resets or carries between
time intervals.

## Blocker Being Lifted

Pinned baseline `chnrt` runs once per event after `wshchr` water routing. It
converts event per-class mass into constant class flux over scalar `rundur` and
has no time-indexed sediment state. Current `SC-ROUTE-001#INV-ROUTE-005(e)`
therefore retains a single-rate channel sediment scope limit.

ADR-0036 requires the watershed to route paired water/sediment timing and
authorizes uniform event-fraction reconstruction of local HBP class mass, but it
does not specify:

- the channel sediment temporal quantum (hour or `dtchr` interval);
- whether channel width/profile/bed state resets or carries between quanta;
- how routed water storage/travel time couples to class sediment ingress;
- how detachment, deposition, transport capacity, and routed class egress close
  per quantum and over the day;
- which zero-flow, dry-carry, and cross-midnight states persist.

## Included Scope

- Top-down Chapter-13/reference and peer-reviewed authority review.
- Pinned baseline source map for `wshchr`, `chnrt`, and WS18-WS31 routines.
- Explicit adjudication of hourly versus `dtchr` channel sediment quantum.
- State-transition specification for geometry/profile/bed and class mass.
- Water-discharge/sediment-flux coupling, class continuity, closure, guards,
  tolerances, and degenerate-state rules.
- ADR-0036 uniform local class reconstruction retained as a labeled
  non-enriched interchange rule unless stronger authority is acquired.
- `SC-ROUTE-001`, `SC-SED-001`, and `SC-SYSTEM-001` amendments only after
  authority is established.
- Contract-derived test obligations and a concrete W11 implementation handoff.
- Dual review, disposition, and verification.

## Excluded Scope

- Rust production implementation.
- HBP schema changes unless authority proves true per-hour enriched class state
  is mandatory; that finding must open a separate additive-schema package.
- Impoundment sediment routing.
- Surrogate/provisional/proxy/heuristic sediment physics.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- this package and WSHED-W11 hold artifacts.

Conditional:

- science-contract authoring procedure/profile/index;
- correctness-authority model and unit governance;
- pinned-baseline decision and source files.

On-demand:

- Chapter 13 and cited sediment/channel literature;
- ADR-0036;
- `SC-ROUTE-001`, `SC-SED-001`, `SC-SYSTEM-001`, `SC-INFILE-HBP-001`.

## Intended Write Set

- `docs/work-packages/20260710-wshedw11a-channel-hourly-sediment-authority-001/**`
- `docs/work-packages/20260710-wshedw11-channel-network-hourly-water-sediment-routing-001/artifacts/worker-handoff.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md` only if lifecycle metadata changes
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

## Required Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/authority-matrix.md`
- `artifacts/contract-disposition.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md` and `review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md` and `verification_agent_b.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`
- `artifacts/w11-handoff.md`

## Phase Plan

1. Reproduce and verify W11's source/contract hold evidence.
2. Build an authority matrix for temporal quantum, water coupling, channel state
   carry, class continuity, and degenerate states.
3. Select one authority-backed algorithm or record a non-promotable hold if the
   evidence is insufficient or contradictory.
4. If authority exists, amend canonical contracts with equations/steps,
   variables/units, branches, guards, invariants, BEI, tolerances, and test
   vectors.
5. Complete dual independent reviews, disposition findings, fix accepted
   findings, and complete dual verification.
6. Publish a W11 implementation handoff naming exact state, algorithms, and
   acceptance vectors.

## Exit Criteria

`EXECUTED-COMPLETE-AUTHORITY` requires canonical authority sufficient to
implement time-resolved channel sediment routing without an executor science
choice, including:

- supported water branch/time grid;
- temporal sediment quantum and state-carry order;
- per-class ingress/egress and detachment/deposition/storage closure;
- typed failure behavior and tolerances;
- contract-derived test vectors;
- explicit W11 resume instructions;
- dual review and verification with no undispositioned findings.

If evidence cannot define the process, close `EXECUTED-HOLD-AUTHORITY` with the
missing external decision/evidence and do not amend production semantics.

## Security Impact

Docs/authority only. No secrets, network service changes, or executable input
surface changes are authorized.

Subagent authorization: this package explicitly authorizes spawning/delegating
to source-lineage, literature/authority, scientific review, and verification
subagents. Expected outputs are package-local authority/review/verification
artifacts. Write access is read-only except for explicitly assigned package
artifact files.
