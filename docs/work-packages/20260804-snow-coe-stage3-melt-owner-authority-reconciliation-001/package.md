# SNOW-COE-STAGE3-MELT-OWNER-AUTHORITY-RECONCILIATION

Status: `scaffolded / authority freeze complete / execution queued`

Date: `2026-08-04`

Package ID:
`20260804-snow-coe-stage3-melt-owner-authority-reconciliation-001`

Plan class: `Contract-first process-physics authority reconciliation`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
`Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` remain current throughout execution.

## Purpose / Big Picture

Resolve the canonical ownership conflict between the production empirical CoE
snowmelt generator and the existing state-resolved Stage 3 snow-energy control
volume. The package must decide whether the post-2007 CoE formulation can be
retained inside an independently supported, enforceable production envelope or
whether Stage 3 becomes the admitted future sole melt owner after cold-content
satisfaction.

The observable result is an authority decision promoted into
`SC-SNOWFREEZE-001` and `SC-SNOWENERGY-001`, a contract-derived static test
that binds the decision, and an implementation hold/handoff precise enough for
a later production package. This package does not change runtime arithmetic,
selectors, defaults, public output, fixtures, or observations.

## Context And Orientation

21M proved that current Rust reproduces the pinned post-2007 CoE term
arithmetic and audited caller ordering, so this is not a transcription-defect
package. It also found no cited independent validation or bounded
transferability authority for material post-handbook changes, including the
2008 `C_canopy` branch, daily midpoint-temperature gate, embedded shortwave
albedo lineage, and revised rain-heat handling.

Current Stage 3 already carries a Marks/SNOBAL active thermal control volume,
cold content, shortwave, optional longwave and latent exchange, conduction,
mass-dependent substeps, and an explicit `unused_positive_energy_j_m2` result.
The contracts deliberately prohibit converting that positive excess to melt
and route already-generated CoE liquid through Stage 3. That intentional split
is the seam this package adjudicates.

## Implementation Intent

- Intent: `canonical authority amendment plus contract-derived static test`.
- Current science implementation:
  `IMPLEMENTED_LEGACY_EMPIRICAL_COE_AND_STAGE3_THERMAL_NO_MELT`.
- Calibration evidence: `NOT_APPLICABLE`.
- Identifiability: `NOT_APPLICABLE`.
- Observation role: all 21L evidence remains `DIAGNOSTIC_ONLY`.
- Production/kernel edit intent: `none`.
- Contract edit intent: `SC-SNOWFREEZE-001`, `SC-SNOWENERGY-001`, and their
  lifecycle registry row.
- Test edit intent: reconcile
  `tests/integration/snow_surface_eb03_contract.rs` for new authority and, after
  the first heavy run discovered repository-wide version-token drift,
  mechanically advance every existing integration assertion that requires
  `SC-SNOWFREEZE-001` version 125 to version 126. Do not alter any other test
  expectation or add executable melt behavior.
- Risk: `critical authority-only change`; canonical production-process
  authority changes even though executable behavior remains byte-identical.

## Included Scope

1. Freeze exact 21M evidence, current contracts, contract test, Stage 3/CoE
   source seams, and pinned libsnobal melt/energy/mass chronology.
2. Inventory every active clause that assigns melt generation to CoE or
   prohibits Stage 3 positive-energy conversion.
3. Test whether CoE has specific independent validation plus an enforceable
   input/process envelope for the post-2007 formula.
4. Derive the state-resolved Stage 3 melt-owner chronology from admitted
   energy, cold-content, latent-fusion, liquid-routing, and single-ledger
   authority without inventing coefficients.
5. Select exactly one frozen outcome and promote it into both canonical
   contracts, their guard/obligation/gap surfaces, and lifecycle registry.
6. Update the existing static contract test after contract amendment so it
   binds the new invariant and also proves runtime remains on the documented
   implementation hold.
7. Complete dual independent contract/science review, finding disposition,
   dual verification, exact-diff reconciliation, prompt archival, and truthful
   roadmap/catalog closure.

## Excluded Scope

- Production Rust, runtime selectors, defaults, parser/runfile/user surfaces,
  public schemas, fixtures, observations, or reference files.
- Implementing Stage 3 melt conversion, turbulent sensible heat,
  precipitation advection, residual-snow disposition, or a cutover.
- Coefficient fitting, site/climate tuning, empirical calibration, or a new
  replacement formula.
- Treating 21L association as causation or using efficacy to choose science
  authority.
- Allowing CoE and Stage 3 to generate melt simultaneously or creating a
  second independently mutable snow-mass ledger.
- Claiming runtime conformance to newly admitted target authority before a
  real-consumer implementation package passes.

## Intended Write Set

- `docs/work-packages/20260804-snow-coe-stage3-melt-owner-authority-reconciliation-001/`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/snow_surface_eb03_contract.rs`
- Existing `tests/integration/*.rs` files containing the exact stale marker
  `contract_version: 125`, limited to a mechanical `125 -> 126` replacement.
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `target/snow_coe_stage3_melt_owner_authority_reconciliation/` for ignored
  validation output.

Everything else is read-only. Reviewer, verifier, and heavy-suite agents are
read-only; the orchestrator owns all tracked edits.

## Authority And Dependencies

- Repository, work-package, contract, test, and validation governance.
- `SC-SNOWFREEZE-001` and `SC-SNOWENERGY-001` current authority.
- 21M lineage, chronology, physical-authority, quantitative, review, and
  verification evidence.
- WEPP Chapter 3, Marks et al. 1998/1999, Ohmura 2001, Walter et al. 2005, and
  CC0 libsnobal commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`.
- Current CoE producer/caller and Stage 3 surface-energy/liquid-routing
  consumer seams.

Package artifacts remain evidence; binding authority belongs in canonical
`SC-*` files.

## Frozen Outcome Matrix

The prospective freeze selects one primary outcome without result-aware rule
changes:

- `BOUNDED_COE_RETAINED`: permitted only if evidence specifically validates
  the material post-2007 formula, supplies an enforceable meteorological,
  canopy, pack-state, and chronology envelope, and leaves no material
  ownership/closure conflict.
- `STAGE3_MELT_OWNER_ADMITTED`: selected when CoE cannot meet that bounded
  evidence bar and independent energy-balance authority supports conversion of
  positive energy remaining after cold-content satisfaction to bounded solid-
  to-liquid mass inside the existing Stage 3 chronology.
- `CURRENT_DUAL_OWNER_ALLOWED`: prohibited by construction because two
  simultaneous melt generators would violate the one-authoritative-ledger
  invariant.
- `AUTHORITY_UNRESOLVED`: selected only if neither a bounded CoE claim nor a
  reproducible Stage 3 target chronology can be supported from frozen sources.

No outcome activates or implements a runtime change in 21N.

## Decision Requirements

The decision must explicitly disposition:

- the 2008 `C_canopy` branch and all empirical `A/B/C/D` terms;
- daily midpoint-temperature gating versus surface/cold-content gating;
- embedded CoE albedo versus the typed Stage 3 albedo/radiation path;
- net radiation, sensible/latent exchange, conduction, and precipitation
  advection completeness;
- positive-energy-to-melt conversion and available-ice bounds;
- `m_s <= 1 kg m^-2` unresolved/residual-snow authority;
- same-substep melt/refreeze/retention/routing chronology;
- exact-one solid-to-liquid and downstream liquid ledgers; and
- current-runtime compatibility and cutover guard posture.

If a Stage 3 target is admitted, any incomplete flux or thin-pack authority is
an explicit implementation `HOLD`, not permission for a proxy or partial
cutover.

## Contract-First Sequence

1. Freeze and adjudicate authority.
2. Amend both canonical contracts and registry.
3. Reconcile the contract-derived static test and every discovered stale global
   contract-version assertion without weakening any behavioral marker.
4. Record the pre-implementation gate proving production Rust is unchanged.
5. Do not perform production edits in this package.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes and requires
spawning/delegating to two independent read-only science/contract reviewers,
two independent read-only terminal verifiers, and one read-only
`comparator_suite_runner` for selected heavy correctness execution. Expected
outputs are compact evidence-classified findings, exact contract/line
citations, outcome-matrix and guard-map checks, exact commands/results, and
final verdicts. No subagent may edit tracked or target files.

## Deliverables

- Frozen machine-readable input and decision contract.
- Active-claim inventory and supersession map.
- Bounded-CoE envelope assessment.
- Stage 3 melt-authority derivation and single-ledger chronology.
- Canonical contract amendments plus lifecycle registry update.
- Reconciled contract-derived static test.
- Contract-cycle references, dual reviews, finding disposition, and dual
  verification for both contracts.
- Required implementation/nonimplementation, gate, security, calibration,
  line-count, exact-diff, disposition, and handoff artifacts.

## Phase Plan

### Phase A: Scaffold And Freeze

Create and commit this package, active prompt, queued artifacts, exact authority
freeze, declared write set, outcome matrix, and active queue state before
result-bearing adjudication or contract edits.

### Phase B: Authority Adjudication

Inventory current claims and apply the frozen evidence bar to CoE. Derive the
Stage 3 target chronology from independent physical authority and existing
typed state. Select one primary outcome and record alternatives explicitly.

### Phase C: Contract And Static-Test Amendment

Promote the selected authority into both contracts, guard maps, obligations,
gaps, binding exposure, and change logs. Update the lifecycle index and then
the authority-owning static contract test and the mechanically affected global
version assertions. Preserve runtime source bytes.

### Phase D: Review, Validate, And Close

Complete dual independent review, disposition every finding, remediate accepted
findings, run direct focused and critical authority-change validation, obtain
dual terminal verification, archive the prompt byte-identically, reconcile the
exact diff, update roadmaps/catalog, and commit truthful closure.

## Validation And Exit Criteria

- Every frozen local and Git-blob identity reproduces.
- Every active CoE/Stage 3 ownership clause is classified as retained,
  superseded-target, historical, or implementation-hold.
- The selected outcome satisfies its frozen predicates; rejected outcomes have
  evidence-backed reasons.
- Contract amendments add canonical invariant, guard, obligation, gap,
  binding-exposure, calibration, and test-vector coverage for the decision.
- The static contract test binds the new target and the current runtime hold;
  it does not weaken or delete existing unrelated assertions.
- Production Rust, fixtures, references, selectors, and public schemas are
  byte-identical.
- Focused contract test, formatting, contract/schema/documentation checks,
  quick/frost profiles, and critical full-workspace correctness pass as
  selected by the critical authority-change classification.
- No external-authority suite posture, cohort fixture, or required-case binding
  changes; anti-evasion gates are required only if terminal diff contradicts
  this declaration.
- Exact terminal diff stays inside the intended write set; changed `.rs` line
  counts are dispositioned and no production `.rs` file changes.
- Dual reviews, finding disposition, dual verification, gate legitimacy,
  prompt archive, security assessment, and truthful status all pass.

## Security And Data Impact

Expected impact is none. Work uses local versioned files and ignored target
output only, performs no credential or network access, and changes no input,
fixture, provider, or public-output surface.

## Progress

- [x] (2026-08-04) User authorized 21N scaffolding and end-to-end execution.
- [x] (2026-08-04) Resolved applicable package, contract, standards, test, and
  documentation instructions before edits.
- [x] (2026-08-04) Froze the authority inputs and outcome matrix before
  result-bearing adjudication.
- [ ] Commit the scaffold checkpoint.
- [ ] Complete authority adjudication and canonical amendments.
- [ ] Complete dual review, direct validation, dual verification, and closure.

## Surprises & Discoveries

- Observation: the current Stage 3 solver already reports positive energy that
  is intentionally left unused after cold-content application.
  Evidence: `runoff_reconciliation.rs` Stage 3 substep loop and diagnostics.
- Observation: the existing static contract test explicitly requires the
  phrase that positive excess does not replace CoE melt.
  Evidence: `tests/integration/snow_surface_eb03_contract.rs`.
- Observation: current Stage 3 does not yet supply a complete melt-owner flux
  set; sensible heat is zero and precipitation advection is absent from the
  implemented carrier.
  Evidence: `stage3_hourly_surface_energy` source inspection.
- Observation: the first quick/frost profiles discovered 35 stale assertions
  across 34 additional integration tests that bind the global
  `SC-SNOWFREEZE-001` version token.
  Evidence: every failure required the stale literal `contract_version: 125`;
  no science or behavior assertion failed. The intended test write set was
  expanded before the mechanical replacements.
- Observation: changing `SC-SNOWFREEZE-001` correctly invalidates the locked
  snow/frost assurance report until its accountable refresh/review cycle.
  Evidence: assurance v2 rejects the old source hash rather than silently
  assembling a stale report. 21N records this as a truthful downstream hold
  and does not rewrite assurance identity or scientific review artifacts.

## Decision Log

- Decision: Keep 21N contract-first and runtime-neutral.
  Rationale: 21M authorized an ownership decision, not a correction; runtime
  work requires the canonical decision and explicit implementation holds.
  Date/Author: 2026-08-04 / Codex.
- Decision: Include the existing static contract test only after the canonical
  amendment.
  Rationale: the test directly binds the superseded prohibition and must remain
  aligned without implementing production behavior.
  Date/Author: 2026-08-04 / Codex.
- Decision: classify the authority amendment as Critical for validation.
  Rationale: the exact diff changes production process authority even though
  production source remains byte-identical.
  Date/Author: 2026-08-04 / Codex.

## Outcomes & Retrospective

Queued pending frozen-matrix adjudication.

## Revision Note

2026-08-04: Initial scaffold created from the roadmap-authorized 21N objective
and the verified 21M handoff.
