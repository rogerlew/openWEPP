# Legacy Ag/Management Port Strategy (openWEPP)

Status: Draft (deferred activation until current water-balance and sediment closure work is complete)

Last updated: 2026-05-25

## 1. Purpose

Define a complete execution strategy for porting the remaining legacy WEPP cropland plant/ag/management routines from `/workdir/wepp-forest_260430_baseline` into openWEPP with:

- contract-first governance,
- typed-error and no-silent-default behavior,
- clean boundaries (domain/use-case/adapters), and
- incremental, testable migration slices.

This is a deferred-start strategy document intended to be activated after current water-balance and sediment closeout waves complete.

## 2. Why This Strategy

The legacy routines are not just "missing functions." They are tightly coupled process loops sharing mutable common-block state across plant growth, residue decomposition, tillage effects, winter/frost transitions, irrigation hydraulics, and runoff/erosion coupling.

A direct line-by-line port would preserve coupling debt and delay testability. The recommended strategy instead uses seams, characterization coverage, and phased kernel extraction so each migrated slice is:

- scientifically traceable,
- isolated enough to test deterministically,
- composable in runner/orchestrator paths, and
- promotable under openWEPP governance.

## 3. Current State Snapshot (openWEPP)

### 3.1 What is already true

- Runner executes climate-span daily iteration and carries writeback surface across days.
  - `crates/openwepp-runner/src/lib.rs:1731-1758`
- Runner kernel implementation is still placeholder-only.
  - `RunnerDailyPhaseKernel` writes only `runner.phase_counter`.
  - `crates/openwepp-runner/src/lib.rs:2855-2877`
- Orchestrator has equation-based growth and decomposition transition logic.
  - Growth equation surface: `crates/openwepp-hillslope-orchestrator/src/lib.rs:7544-7751`
  - Decomposition equation surface: `crates/openwepp-hillslope-orchestrator/src/lib.rs:8355-8559`
- Management parser covers operation/surface/contour/drain definitions.
  - `crates/openwepp-input-contract/src/parsers/management.rs:703-828`
  - `crates/openwepp-input-contract/src/parsers/management.rs:910-960`
  - `crates/openwepp-input-contract/src/parsers/management.rs:963-1050`
- Runtime projection currently emits schedule selectors (`tilseq`, `conset`, `drset`), but operation-level tillage effects are not yet projected into executable kernel payloads.
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1241-1250`

### 3.2 Governance posture that must remain in force

- Contract-first sequence for kernel-affecting work.
- Canonical authority in `SC-*` contracts, not package-local prose.
- Typed failures, no silent defaults/clamping for invalid domain states.
- Required dual review + dual verification + profile checklist before disposition.

Primary authority docs:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`

## 4. Onboarding Digest: Working Effectively with Legacy Code

Reference: `/home/workdir/openWEPP/references/copyrighted/Working.Effectively.with.Legacy.Code.pdf`

This section is intentionally explicit because agents should assume imperfect recall.

### 4.1 Legacy Code Change Algorithm (Feathers)

The book's core algorithm:

1. Identify change points.
2. Find test points.
3. Break dependencies.
4. Write tests.
5. Make changes and refactor.

(openWEPP adaptation)

1. Identify routine-level and phase-level change points in legacy + orchestrator.
2. Choose interception points/pinch points in state trajectories.
3. Introduce seams (kernel composition seams, projection seams, wrapper seams).
4. Write characterization + contract-derived tests.
5. Port slice and refactor toward domain purity.

### 4.2 Seam model you must apply

A seam is a place where behavior can change without editing at the change point.

For openWEPP migration, preferred seam types:

- Kernel phase seam: `HillslopeKernelRequest` -> `KernelRunResponse` boundary.
- Projection seam: parser model -> runtime boundary surface mapping.
- Adapter seam: runner composition selecting kernel implementation.
- Wrapper seam: legacy-dependent or hard-to-test code behind typed interfaces.

### 4.3 Sprout/Wrap patterns and when to use them

Use these deliberately, not as style trivia.

- Sprout Method: add new behavior via new method and a single call site insertion.
  - Use when logic can be a cohesive sequence and host method is hostile to direct refactor.
- Sprout Class: same as above but in a new class when host class cannot be harnessed.
  - Use for introducing new domain kernels without destabilizing orchestrator monolith sections.
- Wrap Method: rename old method, add new wrapper method that orchestrates old + new.
  - Use to place seam around legacy-compatible behavior while adding new deterministic path.
- Wrap Class: add wrapper class around existing class to insert behavior without invasive edits.
  - Use for temporary adapter layers while migrating symbol/stringly surfaces to typed surfaces.

### 4.4 Characterization test posture

The point is to lock actual current behavior before surgery, not to assert ideal behavior first.

Use this characterization loop:

1. Run code in harness at a targeted change area.
2. Observe actual output/trajectory.
3. Encode observed behavior in tests.
4. Mark suspicious behaviors explicitly for later correction decisions.

Then apply Feathers' heuristic:

1. Write tests for the broader area being changed.
2. Add tests for exact behavior being modified.
3. If moving/extracting logic, verify existence + connection behavior case-by-case.

### 4.5 Interception points and pinch points

- Interception point: where change effects are observable.
- Pinch point: narrow encapsulation boundary through which many effects pass.

For this migration, high-value pinch points are:

- Daily runtime surface writeback at phase boundaries.
- WB13/WB12 published daily outputs.
- Plant/residue state vectors (`sumgdd`, `vdmt`, `lai`, `cancov`, `rtmass`, `rtd`, `sumrtm`, `sumsrm`, cover metrics).

## 5. Legacy Codebase Roadmap (Where to Read First)

All paths below are in `/workdir/wepp-forest_260430_baseline/src`.

### 5.1 Daily simulation control flow

Start here to understand macro ordering and coupling:

1. `contin.for:760-866`
   - daily precipitation setup, contour toggles, decomp/soil, winter gate.
2. `wshdrv.for:710-790`
   - watershed daily plane loop, decomp/soil, winter, irrigation callout.
3. `watbal.for:881-922`
   - plant growth branch selection and `swu` coupling.
4. `irrig.for:470-499`
   - furrow vs sprinkler branch and furrow driver invocation.

### 5.2 Call-chain map for target routines

- `contin.for`/`wshdrv.for` -> `tilage` (initial/year transition setup)
- `contin.for`/`wshdrv.for` -> `decomp` (daily decomp + management + cover refresh)
- `decomp` -> `resup` (management date pathways)
- `decomp` -> `covcal` (cover aggregation)
- `watbal.for`/`watbal_hourly.for` -> `swu` (root-zone extraction + water stress)
- `tilage` -> `yldopt` (potential yield/biomass factor pre-compute)
- `contin.for`/`wshdrv.for` -> `winter` (hourly winter/frost/snow coupling)
- `irrig.for` -> `furrow` -> (`furgps`, `kostia`, `furadv`, `furrun`)
- `conrun.for`/`irs.for` -> `appmth` (approx peak runoff method)
- `ptgrp.for` and other perennial paths -> `cutgrz`

### 5.3 Routine dossiers (quick functional topology)

#### `tilage.for`

- File: `tilage.for:1-685`
- Responsibility:
  - reads/assigns crop management schedules,
  - resolves annual/perennial management options,
  - invokes yield option prep,
  - enforces tillage layer constraints.
- Direct calls:
  - `eatcom`, `yldopt`.
- Key coupling surfaces:
  - `tilseq`, `conseq`, `drseq`, residue-management selectors.

#### `newtil.for`

- File: `newtil.for:2-45`
- Responsibility:
  - selects next tillage date index for active sequence.
- Role in architecture:
  - schedule advancement helper; deterministic and small.

#### `grow.for`

- File: `grow.for:1-965`
- Responsibility:
  - GDD accumulation,
  - harvest index dynamics,
  - biomass/canopy/LAI/root mass/root depth evolution,
  - senescence transitions and residue handoff via `resup`.
- Key lines:
  - growth and stress core `grow.for:283-667`
  - senescence path `grow.for:677-780`

#### `resup.for`

- File: `resup.for:1-395`
- Responsibility:
  - residue pool shifting/reset behavior around senescence/harvest.
- Called from:
  - `grow`, `decomp`, perennial flows.

#### `decomp.for`

- File: `decomp.for:1-1012`
- Responsibility:
  - standing/flat/root decomposition,
  - management transitions (burn/cut/remove),
  - tillage-driven residue transformations,
  - ridge/rill residue bookkeeping,
  - invokes cover update.
- Key lines:
  - management actions `decomp.for:645-727`
  - tillage effects `decomp.for:730-790`
  - cover recalc call `decomp.for:977`

#### `covcal.for`

- File: `covcal.for:1-179`
- Responsibility:
  - aggregates standing + ridge/rill flat residue cover into interrill/residue cover metrics.
- Key lines:
  - cover equations `covcal.for:151-177`

#### `swu.for`

- File: `swu.for:1-212`
- Responsibility:
  - distribute transpiration demand by root profile,
  - layer moisture extraction,
  - water-stress update.
- Key lines:
  - extraction core `swu.for:125-188`
- Important migration note:
  - legacy contains silent clamping/defaults (`pltol` bounds, near-zero floors) that must become explicit typed guard/policy behavior in openWEPP.

#### `yldopt.for`

- File: `yldopt.for:1-330`
- Responsibility:
  - derive potential yield and adjust biomass conversion factor (`be`) from growth curve estimates.
- Key lines:
  - yield/be adjustment `yldopt.for:310-319`

#### `winter.for`

- File: `winter.for:1-467`
- Responsibility:
  - hourly winter partition and state transitions,
  - frost/snow interactions,
  - melt/rain/snow process wiring.
- Key lines:
  - 24-hour loop and hourly dispatch `winter.for:260-367`

#### `furrow.for`

- File: `furrow.for:1-399`
- Responsibility:
  - furrow event driver coordinating geometry, advance, and runoff generation.
- Key lines:
  - hydraulic dispatch chain `furrow.for:229-341`

#### `appmth.for`

- File: `appmth.for:1-135`
- Responsibility:
  - approximate kinematic-wave peak runoff relation used in irrigation contexts.

#### `cutgrz.for`

- File: `cutgrz.for:1-45`
- Responsibility:
  - updates perennial harvest date progression through cut/grazing cycles.

### 5.4 State-coupling hotspots to map before coding

Legacy common blocks drive cross-routine mutation. High-priority mapping files:

- `ctillge.inc` (tillage depth/type, roughness/ridge outputs)
- `cridge.inc` (ridge/rill residue masses and cover)
- `ccrpvr1.inc`, `ccrpvr2.inc`, `ccrpvr3*.inc` (crop/residue/plant state families)
- `cwater.inc` (soil-water state)
- `cstruc.inc` (soil layer structure)

Build symbol alias tables from these into canonical `SC-*` contracts before implementation.

## 6. Derived Migration Strategy

### 6.1 Core architectural rule

Port by process-kernel slices behind stable phase seams; do not port by file translation.

Target layering:

- Domain kernels (pure equations + typed guards, no IO).
- Use-case/phase orchestrators (sequence and context dispatch only).
- Adapters (parser projection, runner/CLI publication).

### 6.2 Migration principles

1. Keep legacy semantics visible, not duplicated blindly.
2. Move one causal responsibility at a time.
3. Make state ownership explicit at each seam.
4. Replace legacy silent defaults with typed failure or explicit policy branch.
5. Preserve canonical WEPP symbol continuity in contracts; map runtime aliases explicitly.

### 6.3 Why this works for openWEPP

- Matches existing scheduler/kernel architecture.
- Aligns with contract-first governance and kernel profile requirements.
- Reduces blast radius by isolating behavior at phase boundaries.
- Supports incremental parity evidence instead of all-or-nothing parity claims.

## 7. Detailed Implementation Roadmap

This is the execution sequence once WB/sediment closeout authorizes start.

## Phase 0: Activation and Preflight

Objective:

- Establish migration lane, ownership boundaries, and test harness baseline before physics edits.

Steps:

1. Author queue-authorized work package for Phase 0.
2. Confirm required reading and canonical contract set.
3. Freeze target write set for first kernel slice.
4. Add/refresh baseline replay fixtures and provenance capture.
5. Record initial gap register snapshot.

Exit criteria:

- package artifacts scaffolded,
- baseline replay evidence captured,
- no unresolved governance preconditions.

## Phase 1: Runtime Composition Seam Closure

Objective:

- Replace placeholder `RunnerDailyPhaseKernel` path with composed production kernel dispatch wiring.

Why first:

- If runtime wiring is fake, subsequent ports cannot produce truthful trajectory evidence.

Steps:

1. Introduce runner-level kernel composition seam (factory or injected kernel graph).
2. Wire orchestrator kernel implementation in production path.
3. Keep behavior-neutral for not-yet-ported phases (typed explicit no-op where contract allows).
4. Add tests proving phase-class routing and state writeback propagation over multi-day spans.

Primary files:

- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`

Exit criteria:

- runner no longer relies on `RunnerDailyPhaseKernel` placeholder,
- daily writeback trajectory reflects real kernel outputs,
- gates pass.

## Phase 2: Tillage + Cover Slice

Objective:

- Port tillage-application effects and unified cover calculations (`covcal`) into explicit kernels.

Legacy anchors:

- `tilage.for`, `decomp.for:730-790`, `covcal.for:151-177`.

Contract work:

- Amend `SC-RESIDUE-001` and relevant plant/hydrology boundary contracts with:
  - operation-level tillage effect surfaces,
  - cover aggregation equations,
  - ridge/rill transfer and invariants.

Implementation steps:

1. Extend runtime projection to emit operation payload for active tillage event:
   - `op_ref`, `tildep`, `typtil`, operation effect coefficients.
2. Add `TillageTransitionKernel`:
   - deterministic transformation of standing/flat residues and tillage-dependent fractions.
3. Add `CoverAggregationKernel`:
   - compute ridge/rill/overall cover each day from current residue + standing state.
4. Integrate kernels into phase order so cover is updated before erosion-coupled phases.
5. Add characterization and contract-derived tests with ridge/rill branch cases.

Exit criteria:

- cover metrics are kernel-owned outputs,
- tillage event effects observable in daily trajectory,
- no silent residue-mass compensation.

## Phase 3: Root Water Uptake (SWU) Slice

Objective:

- Implement root-zone extraction and plant water-stress coupling.

Legacy anchors:

- `swu.for:125-188`, call sites in `watbal.for` and `watbal_hourly.for`.

Contract work:

- Amend plant + ET/water-balance contracts with:
  - per-layer extraction equations,
  - stress threshold handling,
  - domain constraints for root depth/layer moisture.

Implementation steps:

1. Define typed per-layer soil-water input/output surfaces.
2. Implement extraction kernel with explicit policy for legacy clamp behaviors.
3. Feed resulting stress back into growth/decomp phases.
4. Add cross-phase integration tests (growth changes when stress changes).

Exit criteria:

- `watstr`-like signal is kernel-derived from soil moisture,
- ET partition is no longer disconnected from root dynamics.

## Phase 4: Yield Semantics Slice

Objective:

- Port `yldopt`/harvest yield semantics from setup-only behavior to explicit runtime/report outputs where contract requires.

Legacy anchors:

- `yldopt.for:310-319`, tillage setup calls.

Implementation steps:

1. Define yield state/output contract surfaces.
2. Implement potential-yield and conversion-factor update kernel behavior.
3. Emit yield events/rows at harvest boundaries.
4. Add tests for annual/perennial harvest branches.

Exit criteria:

- harvest is not reset-only; yield outputs are explicit and contract-governed.

## Phase 5: Winter/Frost Coupling Slice

Objective:

- Port winter hourly process coupling as a dedicated kernel subsystem boundary.

Legacy anchors:

- `winter.for:260-367`, daily call sites in `contin.for`/`wshdrv.for`.

Implementation steps:

1. Define daily-to-hourly adapter boundary and state carry surfaces.
2. Implement winter partition core and frost/snow coupling updates.
3. Integrate with hydrology inputs without hidden side effects.
4. Add tests for no-snow, snow-only, frost-only, mixed cases.

Exit criteria:

- winter behavior no longer represented only as bookkeeping/provenance guards.

## Phase 6: Furrow Irrigation + APPMTH Slice

Objective:

- Close deferred furrow runtime gap and application-month peak runoff method integration.

Legacy anchors:

- `irrig.for:470-499`, `furrow.for:229-341`, `appmth.for:1-135`, `irs.for`/`conrun.for` call sites.

Contract work:

- Resolve `SC-IRRIG-001` deferred furrow gap with process-authoritative algorithm sections and guards.

Implementation steps:

1. Port `appmth` as standalone tested kernel utility.
2. Implement furrow event driver with explicit branch-state modeling.
3. Integrate event outputs into runoff/erosion coupling surfaces (`peakro`, effective duration, etc.).
4. Add concurrency guards for rainfall/furrow constraints per contract.

Exit criteria:

- furrow is no longer parsed-only/deferred,
- promotability constraints in `SC-IRRIG-001` addressed.

## Phase 7: Remaining Management/Contour Closure

Objective:

- Close residual branch gaps:
  - herbicide/silage no-op branches,
  - contour runoff effects,
  - grazing finalization/eatcom semantics,
  - any remaining annual extension edge cases.

Steps:

1. For each residual gap, create contract row + invariant/guard map.
2. Implement branch behavior or explicitly non-promotable disposition.
3. Add focused vectors proving branch activation and effect.

Exit criteria:

- no silent no-op branches for supported modes,
- unresolved items clearly gated as non-promotable.

## Phase 8: Parity and Hold-Lift Readiness

Objective:

- Consolidate evidence for openWEPP-vs-legacy parity lane using migrated kernels.

Steps:

1. Re-run comparator tiers with provenance-complete artifacts.
2. Disposition residual deltas by confidence tier.
3. Update hold-lift decision artifacts.

Exit criteria:

- blocker set closed or explicitly risk-accepted under policy,
- final decision record updated with evidence links.

## 8. Work-Package Design Rules for This Migration

Every kernel-affecting migration package must include:

1. Canonical `SC-*` amendments first.
2. Contract-derived tests second.
3. Pre-implementation contract gate third.
4. Production code edits fourth.
5. Dual review + dual verification artifacts.
6. Kernel profile compliance checklist.
7. Truthfulness labels in evidence (`Static:` vs `Ran:`).

Recommended package slice size:

- one routine family or one coherent cross-routine responsibility per package,
- disjoint write sets when parallelizing workers.

## 9. Suggested Initial Package Queue (when activated)

1. `AGPORT01-runner-kernel-composition-seam-001`
2. `AGPORT02-tillage-and-cover-kernelization-001`
3. `AGPORT03-root-water-use-swu-kernelization-001`
4. `AGPORT04-yield-semantics-and-harvest-emission-001`
5. `AGPORT05-winter-frost-hourly-coupling-kernelization-001`
6. `AGPORT06-furrow-and-appmth-irrigation-kernelization-001`
7. `AGPORT07-management-residuals-contour-closure-001`
8. `AGPORT08-parity-disposition-and-hold-lift-readiness-001`

## 10. Risk Register and Mitigations

- Risk: Hidden coupling through legacy shared state semantics.
  - Mitigation: explicit state-surface contracts + alias mapping before code port.
- Risk: Reproducing legacy bugs via blind characterization.
  - Mitigation: mark suspicious-characterization cases and disposition explicitly.
- Risk: Integration churn from oversized packages.
  - Mitigation: small, responsibility-scoped packages with disjoint write sets.
- Risk: Runner/orchestrator divergence during transition.
  - Mitigation: enforce composed-kernel seam ownership and integration tests each phase.

## 11. Fast-Start Checklist for Future Agents

When this migration is authorized to begin:

1. Read this document fully.
2. Read required governance docs in Section 3.2.
3. Open the legacy roadmap files in Section 5.1 in order.
4. Confirm which `AGPORT*` package is authorized first.
5. Execute contract-first sequence exactly.
6. Keep all new behavior behind explicit seams; avoid invasive inline rewrites.

## 12. Final Note

This strategy intentionally favors controlled scars over brittle elegance in early phases.

The objective is not immediate beauty; it is deterministic, test-covered, contract-authoritative migration that can be steadily refactored into cleaner abstractions after each slice is safely under test.

