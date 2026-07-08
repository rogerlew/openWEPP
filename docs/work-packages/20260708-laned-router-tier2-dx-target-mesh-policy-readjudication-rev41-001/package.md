# LANED Router Tier-2 dx-Target Mesh-Policy Re-adjudication on Rev-41 Solver

Status: `EXECUTED-HOLD-DX5-PROMOTION-UNRATIFIED`

## Objective

Re-adjudicate the Tier-2 target-`dx` active mesh-policy question on the
`SC-OFEROUTE-001` rev-41 positivity-preserving TVD solver. Decide whether the
plain Lane D active router should retain the current fixed `10 cells/OFE`
production mesh or move to a contract-authorized target-`dx` per-OFE mesh
policy.

## Rationale

`20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001` held because
`wa_cascades_forest_h1` high-resolution reference rungs failed active closure
before the package could establish fine-reference adequacy. The rev-41 solver
correction package closed the material clamp-amplification defect for WA
fixed10 and `dx5`. This package reruns the Tier-2 decision surface on the
corrected solver and records the new promotion, rejection, or hold verdict.

The package does not revive the abandoned hybrid implicit stepper. H2637
remains synthetic stress evidence only under ADR-0037.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/decisions/0037-abandon-hybrid-implicit-stepping.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/package.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/mesh-ladder-summary.md`
- `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/package.md`
- `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/mesh-ladder-summary.md`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`

## Scope

Included:
- Scaffold package-local execution, evidence, review, verification, and
  disposition artifacts.
- Reuse the Tier-2 diagnostic mesh ladder on current `main` after rev-41.
- Run the real selected cohort: `mn_corn_h4`, `n_idaho_forest_h1`, and
  `wa_cascades_forest_h1`.
- Run H2637 as synthetic stress evidence only.
- Compare baseline fixed10, `dx20`, `dx10`, `dx5`, `dx2p5`, and `dx1p25`
  against the adequate-fine-reference rule.
- Judge fixed10 on the same candidate-vs-reference surface as target-`dx`
  rungs.
- Amend `SC-OFEROUTE-001` only if the evidence changes canonical mesh-policy
  disposition or authorizes production promotion.
- Implement a production mesh-policy change only if the real selected cohort
  passes predeclared tolerances and contract authority is amended first.

Excluded:
- Hybrid implicit stepping, selector revival, or `SC-OFEROUTE-002` work.
- H2637-only fleet or production-performance claims.
- Co-tuning `LANED_ACTIVE_SAMPLE_DT_S = 900` or
  `LANED_ACTIVE_MAX_DT_S = 300`.
- WEPPpy management, soil, climate, or disturbed-producer changes.
- Relaxing active closure tolerances to make a target-`dx` rung pass.

## Judged Surfaces and Tolerances

The rev-39 T2R adjudication surfaces remain binding:

- Fine-reference adequacy: `dx2p5` compared to `dx1p25`; every judged surface
  must move by no more than one third of its acceptance tolerance.
- Candidate-vs-reference and baseline-vs-reference basis: compare every
  candidate rung to `dx2p5` only after the fine reference is adequate.
- Daily terminal routed-outlet mass: L1 relative tolerance `<= 1%`; adequacy
  threshold `<= 0.333333%`.
- D13 routed hourly shape: max L1 shape tolerance `<= 0.05` with zero
  exceedances; adequacy threshold is max L1 `<= 0.0166667` and no exceedance
  above the threshold.
- Annual pass-sediment sums: max annual relative tolerance `<= 2%`;
  adequacy threshold `<= 0.666667%`.
- End-window storage and tail-fold totals: relative-to-source tolerance
  `<= 1%`; adequacy threshold `<= 0.333333%`.
- `days_uniform_shape` and
  `lane_days_erosion_source_shape_degenerate`: no candidate increase relative
  to reference.
- Active closure, rev-40 clamp-source guard, rev-41 roundoff-only clamp mass,
  and fixed `dt` caps remain hard gates, not tunable tolerances.

## Dependencies

- ADR-0037 hybrid abandonment is on `main`.
- `SC-OFEROUTE-001` rev 41 is on `main`.
- Selected-cohort materialization exists and points at executable run dirs.
- The diagnostic target-`dx` selector and lane-day trace output remain
  available and non-promotional.

## Intended Write Set

Expected:
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Conditional only if evidence changes canonical authority or production policy:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/index.md`
- Active Lane D routing code under `crates/openwepp-hillslope-orchestrator/`
- Runner selector/projection code under `crates/openwepp-runner/`
- Focused tests under `tests/` or crate-local test modules.

## Phase Plan

1. **T2R41-A Scaffold and authority map.** Create package-local scaffolding,
   prompt, artifacts, and catalog/roadmap pointers.
2. **T2R41-B Rev-41 ladder execution.** Build the exact release runner binary
   and rerun the full selected-cohort plus H2637 mesh ladder on current `main`.
3. **T2R41-C Reference adequacy and candidate comparison.** Apply the rev-39
   judged surfaces to the rev-41 evidence, including fixed10 as a judged rung.
4. **T2R41-D Adjudication.** Accept, reject, or hold production target-`dx`
   promotion. If accepted, amend contract authority before implementation.
5. **T2R41-E Implementation if accepted.** Implement only contract-authorized
   production changes and prove active/default/off consumer surfaces.
6. **T2R41-F Review, verification, and disposition.** Complete review,
   disposition, verification, line-count governance, gate results, final
   disposition, and handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, `explorer`, and bounded `worker` subagents for mesh ladder
execution, timing/comparator evidence, review, verification, focused codebase
questions, and bounded implementation help if T2R41-E is reached. Expected
outputs are package-local review, verification, timing, comparator, and
fixture-inventory artifacts. Write access is read-only for
review/verification/comparator/explorer roles; worker write access is bounded
to package artifacts unless the executing parent explicitly assigns a disjoint
implementation write set.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/fixture-cohort-plan.md`
- `artifacts/rev41-ladder-results.md`
- `artifacts/mesh-policy-adjudication.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
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
- Exact runner-binary provenance for timing/comparator evidence.
- Full selected-cohort active plain target-`dx` mesh ladder.
- H2637 synthetic stress ladder reported separately.
- Fine-reference adequacy against `dx1p25`.
- Candidate-vs-reference and baseline-vs-reference tables for every judged
  surface.
- Active-mode closure evidence and rev-40/rev-41 clamp-source/roundoff-clamp
  proof for every completed rung.
- Fixed `dt` caps across all ladder rungs.
- Case-4 focused machinery tests.
- Focused Lane D / `ofe_routing` and mesh-selector tests.
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
- The rev-41 ladder evidence is current and package-local.
- Fixed10 and every target-`dx` candidate are adjudicated against the same
  adequate-fine-reference basis, or the package records a legitimate hold.
- Any production mesh-policy change is contract-first and fully verified.
- Dual review, finding disposition, dual verification, line-count governance,
  gates, and final disposition are complete.

`EXECUTED-HOLD-*` is required when:
- The fine reference is not adequate under rev-41.
- A real selected-cohort member fails a hard active closure/clamp guard.
- Candidate evidence fails a required tolerance and no in-envelope correction
  is authorized.
- Runtime cost is unacceptable under the only fidelity-backed policy.
- A required implementation consumer proof cannot be produced in-envelope.
