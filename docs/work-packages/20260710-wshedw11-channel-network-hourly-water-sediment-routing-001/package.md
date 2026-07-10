# WSHED-W11 Channel-Network Hourly Water/Sediment Routing

Status: `EXECUTED-HOLD-MISSING-CHANNEL-HOURLY-SEDIMENT-SEQUENCING-AUTHORITY`

Package ID:
`20260710-wshedw11-channel-network-hourly-water-sediment-routing-001`

Queue row: `WSHED-W11`

Evidence mode: `Static` plus targeted `Ran` repository/source commands; no
production or canonical contract edits were made.

## Progress

- [x] 2026-07-10: froze execution baseline at
  `2be11f763c8f966a5ac3cab038d88af82650f637` and preserved unrelated worktree
  files.
- [x] 2026-07-10: refreshed instruction discovery for the declared contract,
  Rust, test, package, roadmap, and catalog write set.
- [x] 2026-07-10: completed current-code, ADR-0036, canonical-contract, and
  pinned-baseline water/sediment source-lineage audits.
- [x] 2026-07-10: established that HBP minor-1 is sufficient for the
  ADR-authorized uniform event-fraction hourly class reconstruction; no HBP
  schema extension is required for that explicitly limited fidelity claim.
- [x] 2026-07-10: established water support authority for `ipeak` 3-5 and typed
  failure boundaries for `ipeak` 1/2 and impoundments.
- [x] 2026-07-10: stopped before contract-derived tests or production edits at
  the missing per-interval channel sediment sequencing/state authority boundary.
- [x] Complete dual independent hold reviews and finding disposition.
- [x] Complete dual verification after accepted review fixes.
- [ ] Post-hold: execute production W11 after WSHED-W11A ratifies or rejects
  the missing channel sediment time-stepping authority.

## Surprises and Discoveries

- ADR-0036 D2 already authorizes `M[h,k] = S_h * frcflw[k]` as a labeled
  first-cut uniform reconstruction. `SC-SED-001#GAP-SED-008` keeps the lack of
  true enriched per-hour composition explicit; it does not make minor-1
  unusable for the limited uniform reconstruction.
- The pinned baseline routes water on `q1(0:ntchr, channel)` for `ipeak` 3-5 and
  adds upstream channel `q1` directly into the downstream `qin` series.
- The pinned baseline runs channel sediment once per event after water routing.
  It converts event per-class mass to constant flux over `rundur`; no time index
  or routed `q1` series enters the sediment solve.
- Current Rust exposes a second issue for the eventual implementation:
  `sediment_yield_kg` is populated from `qsed_kg_s`, and publication aggregation
  sums internal channel states. W11A/W11 review must prevent unit aliasing and
  two-channel double counting.

## Decision Log

- Decision: support true W11 channel-network hourly routing only for named
  `ipeak` 3, 4, and 5 branches; retain current no-hourly behavior and fail
  closed for hourly dependency routing under `ipeak` 1/2 or impoundments.
  Rationale: only `wshchr.for` branches 3-5 publish a routed channel time series.
  Date/author: 2026-07-10, Codex.
- Decision: do not require an HBP schema extension for W11's limited class
  fidelity claim.
  Rationale: accepted ADR-0036 D2 explicitly authorizes the event-fraction
  uniform hourly split, while `GAP-SED-008` requires it to remain labeled as not
  enriched per-hour composition.
  Date/author: 2026-07-10, Codex.
- Decision: stop W11 before canonical contract/test/production edits.
  Rationale: no current source or canonical contract defines interval-by-
  interval WS18-WS26 execution, channel bed/profile carry, or the relationship
  between routed water memory and sediment detachment/deposition state. An
  implementation would be surrogate process physics.
  Date/author: 2026-07-10, Codex.

## Outcomes and Retrospective

W11 executed to a contract-first hold. The water implementation route is
source-mapped and feasible, but the package cannot truthfully close water and
sediment network routing together until WSHED-W11A supplies canonical channel
sediment sequencing/state authority. No partial water-only runtime was landed,
so the current fail-closed M-T3 dependency boundary remains intact.

## Objective

Close the explicit M-T3 follow-on by routing HBP minor-1 hourly water and
sediment through a multi-channel watershed dependency chain. Replace the
leaf-channel-only scalar reduction with typed channel-hourly output state that
downstream channels consume on a shared, contract-authorized time grid.

Completion requires the real production watershed CLI to prove this path:

`HBP V_h/S_h -> HillslopeContribution -> channel inlet time series ->`
`channel water/sediment routing -> RoutedChannelState hourly outputs ->`
`downstream channel dependency intake -> public daily aggregates`.

No producer-only, parser-only, leaf-channel-only, peak-only, active-span-only,
or scalar-summary evidence can close the package.

## Current-State Assessment

Static inspection on 2026-07-10 found that M-T3 is complete within its declared
scope, but that scope stops before channel-network hourly routing:

- HBP minor-1 and `HillslopeContribution` carry paired 24-slot
  `hourly_runoff_volume_m3` and `hourly_sediment_mass_kg` arrays.
- `assemble_direct_incoming_peak_partition` sums leaf hillslope arrays, but
  water is reduced to peak, total volume, and active span before the existing
  event-level channel solve.
- Sediment is reduced to total mass plus the active-hour span before the
  quasi-steady, particle-class channel sediment solve.
- `RoutedChannelState` has scalar runoff, peak, duration, and sediment fields;
  it has no routed hourly water or sediment output.
- `direct_hourly_resolved_runon` deliberately rejects an hourly hillslope inlet
  when dependency nodes exist because upstream channels cannot supply hourly
  state.
- The M-T3 production CLI test uses one HBP contributor and one channel. It
  proves distribution sensitivity at that leaf channel, not channel-to-channel
  propagation.
- `SC-ROUTE-001#INV-ROUTE-005(e)` explicitly labels the current sediment path
  as a single-rate scope limit until channels carry hourly surfaces.

The detailed source evidence and rejected closure claims are in
`artifacts/intake-assessment.md`.

## Rationale

The current fail-closed dependency guard is correct, but it prevents an
hourly-authoritative HBP contribution from traversing an ordinary channel
network. Removing that guard without adding routed hourly state would silently
fall back to daily scalars and violate `INV-ROUTE-005`.

The required correction is not 24 independent event calls. Channel routing has
memory, travel time, storage, and a configured `dtchr`/`ntchr` grid. Sediment
routing is particle-class dependent. Execution must establish canonical
authority for:

1. conservative projection of 24 one-hour HBP bins onto the channel time grid;
2. routed channel hydrograph state and channel-to-channel superposition;
3. per-time-step sediment class ingress, transport, detachment/deposition, and
   routed egress;
4. the ADR-0036-authorized uniform event-fraction reconstruction for hourly
   total HBP sediment, labeled explicitly as not enriched per-hour composition.

## Authority Boundary

This is a contract-first baseline-authoritative migration/integration package.
Canonical physics authority remains the `SC-*` contracts and the pinned legacy
baseline `/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Relevant baseline sources include at least:

- `src/wshchr.for` and `src/chrqin.for` for time-step channel inflow/routing;
- `src/wshscs.for` and `src/wshpek.for` for hydrograph superposition and peak
  behavior;
- `src/wshrun.for`, `src/wshirs.for`, and the WS18-WS26 sediment family for
  runoff and sediment continuity;
- `src/wshpas.for` for watershed pass sediment/class lineage.

Do not infer that uniform-within-hour water, constant-within-event particle
fractions, or independent hourly event solves are authoritative. Each may be
used only if canonical contract text and cited provenance explicitly authorize
it.

## Included Scope

- Contract-first authority and typed state for channel-hourly water and
  sediment routing.
- A channel-network support matrix by `ipeak` branch and topology node kind.
- Conservative HBP 24-slot to `dtchr`/`ntchr` projection with exact integral
  closure and explicit time-origin/end-of-day rules.
- Typed routed channel water series with interval, units, provenance, volume,
  peak, duration, storage, and loss closure.
- Typed routed sediment series with sufficient particle-class state to execute
  the canonical channel sediment solver without proxy allocation.
- Channel dependency intake that combines routed upstream channel series and
  local HBP hourly contributors on one authoritative grid.
- Actual channel water routing and sediment continuity on the time series for
  every branch claimed as supported.
- Daily/publication aggregates derived from routed hourly outputs, including
  negative proof that scalar M-T3 summaries are not carrying the W11 claim.
- A committed or generated-with-provenance two-channel sediment-active fixture
  plus a production `openwepp-cli-watershed` acceptance test.
- Same-daily-total/different-hourly-shape vectors proving downstream outlet
  hydrograph and sediment response sensitivity after at least two channels.
- Protected minor-0/no-hourly behavior and typed failure for partial, mixed,
  malformed, unsupported-branch, or unsupported-topology hourly authority.
- Review, verification, comparator-delta review, line-count governance, and
  final package disposition.

## Excluded Scope

- Hillslope hourly producer physics already closed by M-T3.
- New surrogate water or sediment physics.
- Impoundment-hourly routing. Impoundments cross distinct storage, outlet, and
  sediment-trapping authority; W11 must fail closed at an hourly impoundment
  boundary and open a separate package if that path is required.
- Changes to Lane D activation, mesh, groundwater/baseflow generation, or HBP
  producer ownership.
- Public output schema expansion unless a named acceptance surface cannot prove
  the real consumer path without it.
- wepppy orchestration.
- Silent fallback from hourly to scalar routing.

## Required Reading

### Core

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- this package's `package.md`

### Conditional

- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` before contract edits.
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` before baseline
  migration decisions.
- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits.

### On-Demand

- Canonical contracts, ADR-0036, completed M-T3 artifacts, and pinned baseline
  sources named under Dependencies and Authority Boundary.
- Load only the mechanism-specific sections needed for the active phase and
  record them in `artifacts/required-reading-map.md`.

## Required Deliverables

1. `artifacts/required-reading-map.md` with applicable `AGENTS.md` chains and
   byte-budget disposition.
2. `artifacts/intake-assessment.md` updated from scaffold evidence against the
   execution-start revision.
3. `artifacts/baseline-source-map.md` covering water time stepping, channel
   state, sediment classes, dependency propagation, and branch applicability.
4. `artifacts/branch-topology-support-matrix.md` for `ipeak` and channel versus
   impoundment boundaries.
5. `artifacts/operand-lineage.md` completed before production edits.
6. Canonical contract amendments, contract-derived tests, and
   `artifacts/pre-implementation-contract-gate.md` before production edits.
7. Typed hourly channel state and a cohesive implementation module; do not grow
   `kernel/direct.rs` with the full new solver.
8. Focused unit/integration tests and a real two-channel production CLI proof.
9. Independent water and sediment reconstruction plus per-channel and outlet
   closure evidence.
10. Dual review, finding disposition, dual verification, full closure gates,
    and truthful final disposition/handoff.

## Dependencies

- Completed M-T3 package:
  `20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001`.
- `SC-INFILE-HBP-001` minor-1 hourly pair and integral guards.
- `SC-ROUTE-001` rev 50, especially `INV-ROUTE-005`, `INV-ROUTE-009`,
  `INV-ROUTE-011`, and `OBL-ROUTE-C-001`.
- `SC-SED-001` exported mass and particle-class authority.
- `SC-SYSTEM-001` typed frame, dependency ordering, and publication authority.
- ADR-0036 HBP hourly format authority and ADR-0032 watershed runtime authority.
- Pinned baseline source listed under Authority Boundary.

## Intended Write Set

Package and queue:

- `docs/work-packages/20260710-wshedw11-channel-network-hourly-water-sediment-routing-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Canonical authority, only through contract-first phases:

- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md` only if
  the existing hourly-total sediment surface cannot authoritatively support the
  required particle-class time series and a schema extension is approved.

Primary production:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/types.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` only for
  bounded integration/deletion of the M-T3 dependency guard.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs` as the
  preferred cohesive new hourly-routing owner.
- `crates/openwepp-watershed-orchestrator/src/lib_mod/mod.rs` and
  `crates/openwepp-watershed-orchestrator/src/lib.rs` for typed exports/tests.
- HBP writer/parser files only if a contract-authorized schema change is
  required.
- Watershed output writers only if approved publication fields change.

Focused tests/fixtures:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct_tests.rs`
- a new focused hourly-network test module if needed for line governance;
- `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` for
  protected M-T3 behavior;
- a new W11 production CLI contract test and bounded fixture paths.

## Phase Plan

### Phase A - Intake and Authority Map

1. Record clean/dirty state without reverting unrelated changes.
2. Run `tools/agents/find-agents --for` over the actual write set and update the
   required-reading map.
3. Revalidate every statement in the scaffold intake assessment against current
   source and tests.
4. Trace pinned-baseline hydrograph and sediment time-step routines, symbols,
   units, call order, state mutation, and branch applicability.
5. Produce the branch/topology support matrix. Do not begin production edits.

### Phase B - Contract-First Adjudication

1. Amend `SC-ROUTE-001` to define the typed channel time grid, conservative
   inlet projection, routed water/sediment state, dependency superposition,
   per-step closure, and supported `ipeak` branches.
2. Amend `SC-SED-001` with the authoritative hourly particle-class lineage.
3. If event-level fractions cannot authoritatively define each hourly class
   load, amend `SC-INFILE-HBP-001` for the required producer/parser payload
   before implementing sediment routing. Do not synthesize a class allocation.
4. Amend `SC-SYSTEM-001` for channel-state handoff/publication aggregation if
   needed.
5. Record BEI, unit, guard, provenance, tolerance, and gap-register changes.

### Phase C - Contract-Derived Tests and Pre-Implementation Gate

1. Add failing tests for HBP-bin projection closure, time-origin boundaries,
   channel routing memory, upstream/local superposition, sediment class
   continuity, and malformed/mixed authority.
2. Add anti-alias tests where peak-only, active-span-only, daily-mass-only,
   uniform independent-event, and wrong class allocation all produce distinct
   rejected results.
3. Add a two-channel test in which identical daily totals but different HBP
   timing produce different downstream routed water and sediment series.
4. Complete the pre-implementation contract gate before production edits.

### Phase D - Typed State and Production Routing

1. Add typed interval/grid and routed channel-hourly water/sediment state.
2. Implement conservative HBP-to-channel-grid projection under contract
   authority.
3. Implement supported channel water routing using the authoritative stateful
   algorithm, not independent per-hour event solves.
4. Implement supported time-resolved particle-class sediment routing with
   per-step continuity and routed egress.
5. Publish routed hourly channel state into `RoutedChannelState` and consume it
   at downstream channel dependencies.
6. Delete or narrow the M-T3 dependency fail-closed guard only where W11 now
   supplies complete authority. Preserve typed rejection elsewhere.
7. Derive scalar/public daily fields from the routed series and prove the old
   scalar path is not carrying W11 acceptance.

### Phase E - Real Consumer and Conservation Evidence

1. Run a two-channel sediment-active production watershed CLI fixture from HBP
   through downstream outlet publication.
2. Independently reconstruct inlet/outlet/storage/loss water closure for each
   channel and the network.
3. Independently reconstruct inlet/detachment/deposition/outlet sediment closure
   by particle class and in total.
4. Prove equal daily totals/different timing changes downstream routed series
   and named public aggregates where physics requires it.
5. Prove minor-0/no-hourly protected identity and fail-closed unsupported
   topology/branch behavior.

### Phase F - Closure

1. Run focused contract, kernel, and production CLI tests.
2. Run anti-evasion guards if authority-suite or required-case bindings change.
3. Run `cargo fmt --check`.
4. Run `cargo clippy --workspace --all-targets -- -D warnings`.
5. Run `cargo nextest run --workspace --profile full`.
6. Run `cargo deny check`.
7. Run scoped Markdown lint and `git diff --check`.
8. Complete dual independent reviews, disposition every finding, and complete
   dual verification after accepted fixes.
9. Record line counts, comparator deltas/confidence tiers, final disposition,
   and worker handoff.

## Conservation and Output Acceptance

Before production edits, the operand-lineage table must record for every water
and sediment series: units, interval, time origin, normalization, area/volume
basis, particle-class basis, source authority, and authoritative/diagnostic
status. Fixtures must separate every plausible alias or wrong aggregation.

Acceptance requires independent reconstruction from produced outputs/state and
real two-sided magnitude/closure audits. Exact self-consistency, one-sided
bounds, or reusing producer formulas are supporting evidence only.

## Hold Boundaries

`HOLD` is legitimate only when evidence proves one of these boundaries:

- canonical/baseline authority cannot define HBP-to-channel time projection;
- authoritative hourly particle-class ingress cannot be obtained from the
  existing HBP surface and an HBP schema amendment is outside an approved
  amended write set;
- a required channel branch needs missing baseline/contract physics;
- the target topology crosses impoundment-hourly authority excluded here;
- required external fixture/comparator evidence is unavailable.

Implementation size, diagnostic uncertainty, a working leaf-channel scalar
path, or partial channel-network routing are not hold boundaries. A hold must
include `artifacts/hold-legitimacy-audit.md` and a defect-shaped first action.

## Exit Criteria

`EXECUTED-COMPLETE` requires all of the following:

- every supported W11 channel branch consumes and emits typed hourly water and
  sediment state through at least two dependency-ordered channels;
- the real production CLI consumes HBP hourly input and the downstream channel
  reads upstream routed hourly output;
- scalar M-T3 peak/span/mass summaries do not carry the W11 claim;
- per-channel and network water and particle-class sediment closure pass
  independent reconstruction and magnitude audits;
- unsupported branch/topology and malformed/mixed authority fail closed;
- protected no-hourly behavior passes identity gates;
- all package gates have current direct evidence;
- all review findings are dispositioned and accepted fixes are reverified;
- line-count governance and final workspace closure gates pass.

Any unmet current-scope criterion requires continued execution or an
`EXECUTED-HOLD-*` disposition with a proven boundary. It may not be deferred
while the package is marked complete.

## Security Impact

Expected impact: `none` beyond existing local runfile/HBP parsing and bounded
fixture generation. Preserve path validation, payload length/CRC guards,
allocation bounds, typed numerical errors, and fail-closed serialization.

Subagent authorization: this package explicitly authorizes spawning/delegating
to source-lineage, Rust review, verification, and comparator-suite-runner
subagents for read-only baseline/contract review, bounded package artifact
review, and heavy closure/comparator execution. Expected outputs are the named
package review/verification artifacts and command evidence. Write access is
read-only except for explicitly assigned review/verification artifact files.
