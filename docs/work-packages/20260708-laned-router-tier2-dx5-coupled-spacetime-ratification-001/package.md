# LANED Router Tier-2 dx5 Coupled Space-Time Ratification

Status: `EXECUTED-HOLD-DX5-UNRATIFIED`

## Objective

Decide whether `dx5` can become the active production mesh default under
`SC-OFEROUTE-001` rev 43 using coupled space-time evidence over the selected
real cohort. If the evidence is sufficient, amend the contract first and then
flip the active production default. If not, hold with the exact blocker and
first follow-on action.

## Rationale

The rev-41 Tier-2 re-adjudication showed `dx5` is the only tested target-`dx`
rung without a provisional candidate-table blocker, but production promotion
held because `mn_corn_h4` failed the strict one-third fine-reference shape
adequacy gate. The follow-on timestep-policy adjudication proved one
fine-reference shape miss was a timestep-policy artifact and amended
`SC-OFEROUTE-001` rev 43 to require coupled space-time evidence before any
renewed target-`dx` production promotion.

This package executes that renewed decision. Fidelity remains first and
runtime cost remains priced evidence, not a promotion blocker.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/package.md`
- `docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/timestep-policy-adjudication.md`
- `docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-policy-adjudication.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/artifacts/fine-reference-adequacy.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md` if
  `SC-OFEROUTE-001` is amended.
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  if `SC-OFEROUTE-001` is amended.
- `docs/specifications/science-contract-spec.md` if BEI rows are amended.
- Prior Tier-2 raw-hydrograph/day-attribution package artifacts if a new
  selected-cohort surface fails and mechanism attribution is needed.

## Scope

Included:

- Scaffold package-local evidence, prompt, review, verification, gates,
  disposition, and handoff artifacts.
- Predeclare and run a selected real-cohort coupled space-time ladder for
  `dx5`, `dx2p5`, and `dx1p25` under production `max_dt=300` and refined
  diagnostic `max_dt=75`.
- Judge `dx5` candidate surfaces against an adequate fine-reference basis,
  with same-`dt` spatial comparisons and same-`dx` timestep controls.
- Price runtime cost and record solver counters; do not make cost a fidelity
  promotion blocker.
- Amend `SC-OFEROUTE-001` before any production default flip if `dx5` passes.
- Implement only the contract-authorized active production mesh default change
  if evidence ratifies it.

Excluded:

- Hybrid implicit stepping, selector revival, or `SC-OFEROUTE-002` work.
- Routed-shape tolerance widening.
- Source/coefficient, climate, soil, crop, or management tuning.
- Shadow mesh-policy changes unless explicitly recorded out-of-scope or held.
- Production max-substep default changes unless the package proves `dx5` cannot
  be ratified at the current production cap and closes by hold.
- Treating H2637 synthetic stress evidence as fleet-general promotion
  authority.

## Judged Surfaces and Tolerances

The rev-38/rev-43 target-`dx` surfaces remain binding:

- Fine-reference adequacy: one further target-`dx` halving must move every
  judged surface by no more than one third of that surface's acceptance
  tolerance.
- Candidate-vs-reference basis: compare `dx5` against the adequate `dx2p5`
  reference, not against fixed10.
- Daily terminal routed-outlet mass: L1 relative tolerance `<= 1%`; adequacy
  threshold `<= 0.333333%`.
- D13 routed hourly shape: max L1 shape tolerance `<= 0.05` with zero
  exceedances; adequacy threshold is max L1 `<= 0.0166667`.
- Annual pass-sediment sums: max annual relative tolerance `<= 2%`; adequacy
  threshold `<= 0.666667%`.
- End-window storage and tail-fold totals: relative-to-reference-source
  tolerance `<= 1%`; adequacy threshold `<= 0.333333%`.
- `days_uniform_shape` and
  `lane_days_erosion_source_shape_degenerate`: no candidate increase relative
  to reference.
- Active closure, rev-40 clamp-source guard, rev-41 roundoff-only clamp mass,
  DC01/no-double-feed behavior, and D13 routed-hydrograph erosion consumption
  remain hard gates.

## Dependencies

- `SC-OFEROUTE-001` rev 43 is on `main`.
- The active max-`dt` diagnostic selector is trace-gated and available.
- Selected-cohort materialization exists at
  `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json`.
- ADR-0037 hybrid abandonment is on `main`; H2637 is synthetic stress only.

## Intended Write Set

Expected:

- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Conditional only if evidence ratifies production promotion:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/index.md` only if lifecycle metadata
  changes.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- Focused tests under crate-local modules or `tests/`.

## Phase Plan

1. **CST-A Scaffold and authority map.** Create package files, ignored raw run
   root, prompt, required-reading map, and catalog/roadmap pointers.
2. **CST-B Coupled ladder execution.** Build the exact release runner and run
   the selected real cohort at the predeclared rungs. Record binary
   provenance, timings, solver counters, hashes, and trace summaries.
3. **CST-C Coupled adjudication.** Apply same-`dt` spatial and same-`dx`
   timestep controls. Decide whether `dx5` at the production max-substep cap
   is fidelity-ratified or held.
4. **CST-D Contract-first production flip if ratified.** If, and only if,
   `dx5` passes, amend `SC-OFEROUTE-001` before changing the production
   default from fixed10 to target `dx5`.
5. **CST-E Consumer and closure proof if flipped.** Prove active production
   owns the mesh policy, default/off behavior remains protected, DC01 surface
   runon is not double-fed, and D13 consumes the routed hydrograph shape.
6. **CST-F Review, verification, and gates.** Complete dual review,
   disposition, dual verification, line-count governance, gates, final
   disposition, and handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `explorer`, and bounded `worker` subagents for coupled
space-time ladder execution, timing/comparator evidence, code/contract review,
verification, and bounded implementation help if CST-D is reached. Expected
outputs are package-local review, verification, timing/comparator, and
implementation artifacts. Write access is read-only for review, verification,
comparator, and explorer roles; worker write access is bounded to package
artifacts unless the executing parent explicitly assigns a disjoint
implementation write set.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/fixture-cohort-plan.md`
- `artifacts/coupled-spacetime-summary.md`
- `artifacts/coupled-spacetime-summary.json`
- `artifacts/mesh-policy-ratification.md`
- `artifacts/mesh-policy-ratification.json`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/contract-disposition.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review-agent-a.md`
- `artifacts/review-agent-b.md`
- `artifacts/disposition.md`
- `artifacts/verification-agent-a.md`
- `artifacts/verification-agent-b.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Exact release-binary provenance for ladder evidence.
- Selected real-cohort coupled space-time ladder.
- Same-`dt` spatial reference adequacy and candidate comparison.
- Same-`dx` timestep-refinement controls for `dx5`, `dx2p5`, and `dx1p25`.
- Active-mode closure evidence and rev-40/rev-41 clamp-source proof for every
  completed rung.
- Runtime timing/cost evidence recorded but not used as a fidelity blocker.
- Focused active mesh-selector / trace tests.
- Focused Lane D / `ofe_routing` tests for Rust changes.
- Contract/profile/BEI checks if any `SC-*` contract changes.
- Protected default/off byte identity if production/default surfaces change.
- DC01/no-double-feed proof if production active routing changes.
- Routed-hydrograph-to-erosion consumer proof if production active routing
  changes.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, cohort fixtures,
  or external-authority suite posture are touched.
- `.rs` line-count governance.

## Exit Criteria

`EXECUTED-COMPLETE` requires:

- Coupled selected-cohort evidence is current and package-local.
- `dx5` is adjudicated against an adequate coupled fine-reference basis.
- Any production mesh-policy flip is contract-first and verified on active
  consumer/default/off surfaces.
- Reviews, disposition, verification, line-count governance, gates, and final
  disposition are complete.

`EXECUTED-HOLD-*` is required when:

- Coupled reference adequacy does not close.
- `dx5` fails a required selected-cohort fidelity surface.
- `dx5` passes only under a diagnostic max-substep policy outside this
  package's production-change authority.
- A real selected-cohort member fails active closure/clamp-source hard gates.
- Required implementation consumer proof cannot be produced in-envelope.
