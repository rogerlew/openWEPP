# HPHYS0269 Winter Melt Snowpack Baselining

Status: completed/HOLD

## Objective

Port and wire baseline-authoritative `winter.for` snowpack/melt behavior into
openWEPP architecture using contract-first amendments, iterative diagnosis,
verification, and baselining against the pinned legacy source. The package
focuses on the HPHYS0268 continuation: candidate snow-water is present in early
winter but is released too early relative to baseline, so `winter -> snowd ->
melt` state retention, daily snowpack update, hourly melt release, and WB13
`RM`/`Snow-Water` publication must be made authoritative before returning to
WB17 `Ep`.

## Rationale

HPHYS0268 corrected stale inactive-day snow hourly surfaces and proved that
H1/H7/H39 material spring divergences now have closed trace identities, but the
remaining residual is semantic: baseline retains large snowpack into spring
while openWEPP depletes snowpack much earlier. The next package must avoid
another narrow residual-tuning loop. It should migrate and wire the
baseline-authoritative snowpack process path so later `Ep`, runoff,
percolation, lateral-flow, and aggregate-storage residuals are interpreted
against a defensible winter hydrology substrate.

## Included Scope

- Iterative static and dynamic diagnosis of baseline `winter.for`,
  `snowd.for`, and `melt.for` against openWEPP snowpack/winter support.
- Canonical `SC-SNOWFREEZE-001`, `SC-WATBAL-001`, and any required seam
  contract amendments before production edits.
- Contract-derived tests and trace assertions for daily snowpack retention,
  negative-melt/cold-content handling, density/depth settlement, hourly melt
  release, `RM`, and WB13 `Snow-Water`.
- Baseline provenance mapping for every migrated equation, constant, branch
  guard, unit conversion, and runtime alias.
- Production implementation or wire-up only where contract and diagnosis prove
  a canonical authority gap.
- Targeted H1/H7/H39 verification plus full H1..H39 hillslope semantic metrics.
- Dual review and dual verification with explicit issue disposition.

## Excluded Scope

- WB17 `Ep` production edits unless a snowpack contract change directly proves
  an adjacent seam must be updated for snowpack publication.
- Frost activation for non-agricultural HPHYS parity.
- Forest `ksatadj`, infiltration, lateral-flow, or percolation migrations
  unless required only as read-only context for snowpack/runoff classification.
- Calibration, empirical tuning, surrogate snowmelt factors, or process
  substitutions not traceable to canonical contract authority, including pinned baseline authority where valid and accepted `/workdir/wepp-forest` bug-fix authority where explicitly ratified.
- Sidecar discoverability policy changes except where a missing snow input is
  proven to violate the existing legacy contract.

## Deliverables

- Baseline source provenance map covering `winter.for`, `snowd.for`,
  `melt.for`, and required support routines.
- Canonical contract amendments completed before production-code edits.
- Contract-derived tests and trace evidence that fail or classify the current
  defect before implementation and pass or improve after implementation.
- Iterative diagnosis log that records each source seam, candidate behavior,
  baseline behavior, and disposition.
- Production migration/wire-up for proven baseline snowpack gaps.
- Targeted H1/H7/H39 and full H1..H39 verification metrics.
- Final `GO`/`HOLD` disposition and continuation handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- `/workdir/wepp-forest_260430_baseline/src/melt.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- Pinned baseline commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Corrected negative-melt authority commit `/workdir/wepp-forest@03fee4558456535138592630b5dedc4d81ce8d06`
- `docs/audits/20260525_water_erosion_kernel_audit.md`
- `docs/audits/20260603_wepp_forest_nonag_frost_disable_audit.md`
- `docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/spring-snowpack-lineage-diagnosis.md`
- `docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/full-39-suite-metrics.md`
- `docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/worker-handoff.md`

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`, only
  if snowmelt runoff publication semantics require amendment.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`, only
  if forcing construction is proven inconsistent with baseline support routines.
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- New focused tests under existing test locations only when adjacent patterns
  support them.
- `docs/work-packages/README.md`
- `docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/**`

## Phase Plan

1. Read required authority and record the HPHYS0268 continuation state.
2. Build a baseline provenance map for `winter.for`, `snowd.for`, `melt.for`,
   and required support routines with source line ranges, units, variable
   aliases, state transitions, and publication surfaces.
3. Amend canonical `SC-*` contracts for daily snowpack state retention,
   negative-melt/cold-content behavior, snow density/depth settlement, hourly
   melt release, runoff melt publication, and WB13 `Snow-Water` state.
4. Add or update contract-derived tests and trace assertions before production
   implementation.
5. Record the pre-implementation contract gate with `Static:` and `Ran:`
   evidence labels.
6. Run iterative H1/H7/H39 diagnosis and classify each snowpack seam as
   forcing, winter trigger, daily `snowd`, hourly `melt`, publication, or
   out-of-scope context.
7. Implement or wire one baseline-authoritative vertical slice at a time,
   rerunning focused tests and updating diagnosis artifacts after each slice.
8. Run targeted H1/H7/H39 verification and the full H1..H39 hillslope suite.
9. Complete dual review, dual verification, final disposition, and continuation
   handoff.

## Contract-First Sequence

1. Contracts.
2. Contract-derived tests and trace assertions.
3. Pre-implementation contract gate.
4. Production code edits, only after the gate records adequate authority.

## Iteration Rules

Each implementation loop must target one authoritative source seam only. The
agent must record the baseline routine, source line ranges, openWEPP target
function, expected state transition, validation command, observed result, and
next seam. If a seam cannot be migrated completely, leave disposition in
`HOLD`, record the exact unmigrated baseline logic, and do not substitute
heuristic formulas.

## Exit Criteria

- Evidence artifacts truthfully distinguish `Static:` from `Ran:`.
- Canonical contracts contain the required snowpack authority before production
  edits.
- Every migrated equation, constant, guard, and unit conversion has pinned
  baseline provenance and an openWEPP alias mapping.
- Contract-derived tests cover daily snowpack retention and hourly melt release
  at the `winter -> snowd -> melt -> WB13` seam.
- H1/H7/H39 targeted metrics identify whether early snow depletion is corrected
  or remains assigned to an explicit unmigrated source seam.
- Full H1..H39 semantic metrics are recorded after implementation.
- The package closes only if baseline-authoritative snowpack/melt migration is
  complete for the declared scope; otherwise final disposition is `HOLD` with a
  concrete continuation package recommendation.

## Security-Impact Gate

No external systems or network actions are required. This package is local
repository engineering work limited to flat-file reads/edits and local command
execution in the worktree.

## Autonomy

This package is intended for end-to-end autonomous execution. Agents must work
through the phase plan, update artifacts through disposition, and only ask for
user direction when hard-blocked by missing local authority or unavailable
validation substrate.
