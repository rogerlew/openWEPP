# LANED Router Tier-2 dx5 Fine-Reference Hold Lift

Status: `EXECUTED-HOLD-MN-CORN-H4-SHAPE-NONCONVERGED`

## Objective

Execute the narrow hold-lift named by the rev-41 Tier-2 target-`dx`
mesh-policy re-adjudication: run the `mn_corn_h4` `dx0p625` fine reference,
re-run the strict one-third adequacy gate without amending the rule, and close
or hold the `dx5` production active mesh-policy decision on that evidence.

Operator posture, 2026-07-08: fidelity first, speed secondary. Runtime cost is
priced and recorded, but is not a promotion blocker for the mesh-policy
fidelity verdict. Cost optimization is follow-on work after the fidelity
verdict.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/package.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-policy-adjudication.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-ladder-summary.md`
- `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/package.md`
- `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json`

## Scope

Included:
- Scaffold package-local execution, evidence, review, verification, and
  disposition artifacts.
- Build and record exact release-runner provenance.
- Run `mn_corn_h4` active plain target-`dx` rungs needed to close the
  `dx0p625` reference question and rebase the `mn_corn_h4` candidate table.
- Re-run the strict one-third adequacy gate for `dx1p25` versus `dx0p625`.
- Retain the prior PASS adequacy verdicts for other selected-cohort members
  unless the new `mn_corn_h4` evidence implicates another judged surface.
- If adequacy closes, finalize the selected-cohort candidate adjudication on
  the adequate reference basis.
- If `dx5` ratifies as fidelity-adequate, amend `SC-OFEROUTE-001` before code
  and promote `dx5` as the opt-in active production mesh default.
- Record the accepted cost from the rev-41 cohort (`~4.85x` aggregate real
  selected-cohort user time) as priced, non-blocking evidence under the
  operator posture.
- Decide the shadow-mesh question explicitly.

Excluded:
- Relaxing or amending the one-third fine-reference adequacy rule.
- Tuning tolerances after evidence is known.
- Re-running the full selected cohort unless a gate or new evidence requires
  it.
- Hybrid implicit stepping or `SC-OFEROUTE-002` revival.
- Cost optimization or local numerics optimization.
- WEPPpy, climate, management, or disturbed-producer changes.
- Shadow mesh-policy implementation unless the contract amendment explicitly
  scopes it.

## Judged Surfaces and Tolerances

The rev-39/rev-41 Tier-2 surfaces remain binding:

- Fine-reference adequacy: one further halving must move every judged surface
  by no more than one third of its named tolerance. For this package the new
  check is `dx1p25` compared with `dx0p625` for `mn_corn_h4`.
- Candidate-vs-reference basis: compare candidate rungs to an adequate fine
  reference. If `dx1p25` becomes adequate, `mn_corn_h4` candidate checks are
  judged against `dx1p25`; other members retain their existing adequate
  reference basis unless newly implicated.
- Daily terminal routed-outlet mass: L1 relative tolerance `<= 1%`; adequacy
  threshold `<= 0.333333%`.
- D13 routed hourly shape: max L1 shape tolerance `<= 0.05`; adequacy
  threshold max L1 `<= 0.0166667`.
- Annual pass-sediment sums: max annual relative tolerance `<= 2%`; adequacy
  threshold `<= 0.666667%`.
- End-window storage and tail-fold totals: relative-to-source tolerance
  `<= 1%`; adequacy threshold `<= 0.333333%`.
- `days_uniform_shape` and
  `lane_days_erosion_source_shape_degenerate`: no candidate increase relative
  to reference.
- Active closure, rev-40 clamp-source guard, rev-41 roundoff-only clamp mass,
  and fixed `dt` caps remain hard gates.

If `mn_corn_h4` shape does not converge at `dx0p625`, this package stops at a
hold. The hold audit must attribute the shape surface, checking first for cliff
flips in `degenerate-shape` days, `routed_tail_fold_m3`, and
`routed_end_window_storage_m3`.

## Intended Write Set

Expected:
- `docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Conditional only if `dx5` promotion is authority-backed:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- Focused crate-local tests or integration tests.

## Phase Plan

1. **T2R42-A Scaffold and evidence plan.** Create package scaffolding,
   package-local ignored run root, required-reading map, and catalog pointers.
2. **T2R42-B Fine-reference execution.** Build the exact release runner and run
   `mn_corn_h4` rungs including `dx0p625`.
3. **T2R42-C Adequacy and adjudication.** Apply the strict one-third gate and
   either hold with attribution or finalize the `dx5` fidelity verdict.
4. **T2R42-D Contract-first promotion if authorized.** Amend
   `SC-OFEROUTE-001`, then implement the production active mesh default.
5. **T2R42-E Production-surface proof.** If promotion lands, prove active
   closure, protected default/off behavior, no-double-feed posture, and routed
   hydrograph-to-erosion consumer evidence.
6. **T2R42-F Review, verification, and disposition.** Complete dual review,
   disposition, dual verification, line-count governance, gates, final
   disposition, and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to review, verification, comparator/timing, explorer, and
bounded worker subagents. Expected outputs are package-local review,
verification, timing, comparator, and implementation-readiness artifacts. Write
access is read-only for review/verification/comparator/explorer roles; worker
write access is bounded to package artifacts unless the executing parent
explicitly assigns a disjoint implementation write set.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/fixture-cohort-plan.md`
- `artifacts/fine-reference-summary.md`
- `artifacts/fine-reference-summary.json`
- `artifacts/fine-reference-adequacy.md`
- `artifacts/mesh-policy-final-adjudication.md`
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
- Exact release-runner provenance for timing/comparator evidence.
- `mn_corn_h4` `dx0p625` fine-reference run with opt-in active routing and
  trace enabled.
- Strict one-third fine-reference adequacy gate.
- Candidate-vs-adequate-reference table if adequacy closes.
- Active-mode closure evidence and rev-40/rev-41 clamp-source/roundoff-clamp
  proof for completed rungs.
- Fixed `dt` caps across ladder rungs.
- If production default changes: contract/profile/BEI checks, focused Lane D /
  `ofe_routing` tests, protected default/off byte identity evidence,
  active-mode closure evidence, DC01/no-double-feed proof, routed-hydrograph
  erosion-consumer proof, `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full`, and `cargo deny check`.
- Source-level anti-evasion guards if required-case bindings, cohort fixtures,
  or external-authority suite posture are touched.
- `.rs` line-count governance if Rust files change.

## Exit Criteria

`EXECUTED-COMPLETE` requires:
- `mn_corn_h4` fine-reference adequacy closes under the predeclared rule.
- The selected-cohort mesh-policy verdict is finalized on an adequate
  reference basis.
- Any production mesh-policy change is contract-first and fully verified.
- Review, disposition, verification, gate results, final disposition, and
  handoff are complete.

`EXECUTED-HOLD-*` is required when:
- `mn_corn_h4` shape does not converge at `dx0p625`.
- A required active closure or clamp guard fails.
- `dx5` fails the fidelity comparison after the reference basis closes.
- A required production proof cannot be produced in-envelope.
