# LANED Router mn_corn_h4 Routed-Shape Attribution

Status: `EXECUTED-HOLD-SOLVER-CLASS-DAY792`

## Objective

Attribute the single-day `mn_corn_h4` routed-hourly-shape adequacy miss before
any renewed target-`dx` promotion package.

Target row:

- member: `mn_corn_h4`
- `sim_day_index = 792`
- `lane_index = 1`
- observed max L1 shape miss: flat around `0.0202..0.0209`
- strict one-third threshold: `0.0166667`

## Rationale

`20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001` showed the
`dx0p625` reference does not close the existing shape adequacy blocker. The
named counters do not indicate a cliff in uniform shape, degenerate shape,
tail-fold, or end-window storage. This package classifies the miss before any
metric repair or production mesh-policy work.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/package.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/artifacts/worker-handoff.md`

## Scope

Included:
- Scaffold package-local execution, evidence, review, verification, and
  disposition artifacts.
- Re-run `mn_corn_h4` rungs `dx2p5`, `dx1p25`, and `dx0p625` with exact
  release-binary provenance.
- Build a package-local single-day attribution fixture around day 792 lane 1.
- Run the normalization-amplification test.
- Run the hour-edge aliasing test.
- Run the raw unbinned outlet-hydrograph convergence test.
- Classify the miss as metric-class, projection-aliasing metric-class,
  solver-class, or unresolved hold.
- If metric-class, propose a conditioned shape gate and amend
  `SC-OFEROUTE-001` contract-first before rerunning adequacy under the repaired
  gate.
- If solver-class, stop at hold with attribution and no contract amendment.

Excluded:
- Production target-`dx` promotion or active mesh default flip.
- Widening the existing `0.05` shape tolerance in place.
- Relaxing the one-third adequacy rule without an explicit repaired metric.
- Cost optimization.
- Hybrid solver revival.
- WEPPpy, management, climate, or disturbed-producer changes.

## Discriminating Tests

1. **Normalization-amplification test.** Record day-792 absolute outlet mass at
   `dx2p5`, `dx1p25`, and `dx0p625`; express the shape L1 deltas as absolute
   cubic metres and compare against the `9.04e-5 m3` end-window storage
   difference.
2. **Hour-edge aliasing test.** Compare cumulative hourly CDF curves across
   the three rungs. If CDF distance converges while binned max L1 stays flat,
   classify the miss as projection aliasing.
3. **Genuine-nonconvergence test.** Compare the raw unbinned outlet hydrograph
   for day 792 across rungs. If it does not converge, classify as solver/day
   nonconvergence and hold for a numerics package.

## Disposition Gate

The predeclared-tolerance/metric amendment path is available only after the
three tests classify the miss as metric-class. If metric-class, choose a
conditioned shape gate (mass-floored, mass-weighted, or CDF-based), justify the
choice, amend `SC-OFEROUTE-001`, and rerun adequacy under the repaired gate.

If adequacy closes under the repaired gate, this package stops there. The
`dx5` ratification and fidelity-first production flip remain a successor
package.

## Intended Write Set

Expected:
- `docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Conditional if metric-class:
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/index.md` only if lifecycle registry
  metadata changes.
- Narrow contract-derived tests if required by the amended text.

Conditional if a diagnostic extraction surface is needed:
- Bounded trace/debug code under `crates/openwepp-hillslope-orchestrator/` or
  `crates/openwepp-runner/`, with no default/off behavior change.

## Phase Plan

1. **MCS-A Scaffold and authority map.** Create package files, prompts,
   ignored raw run root, and catalog pointers.
2. **MCS-B Evidence rerun.** Re-run `mn_corn_h4` `dx2p5`, `dx1p25`, and
   `dx0p625` with trace enabled and exact release-binary provenance.
3. **MCS-C Attribution tests.** Run normalization, CDF, and raw-hydrograph
   tests on day 792 lane 1.
4. **MCS-D Classification.** Decide metric-class vs solver-class and record
   evidence.
5. **MCS-E Contract amendment if metric-class.** Amend `SC-OFEROUTE-001`
   only if tests justify a repaired gate, then rerun adequacy under that gate.
6. **MCS-F Review, verification, and disposition.** Complete dual review,
   disposition, dual verification, line-count governance, gate results, final
   disposition, and handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to review, verification, comparator/timing, explorer, and
bounded worker subagents. Expected outputs are package-local review,
verification, timing, comparator, attribution, and implementation-readiness
artifacts. Write access is read-only for review/verification/comparator/
explorer roles; worker write access is bounded to package artifacts unless the
executing parent explicitly assigns a disjoint implementation write set.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/fixture-plan.md`
- `artifacts/shape-attribution-summary.md`
- `artifacts/shape-attribution-summary.json`
- `artifacts/day792-attribution.md`
- `artifacts/day792-attribution.json`
- `artifacts/metric-repair-proposal.md` or
  `artifacts/solver-class-hold-audit.md`
- `artifacts/implementation.md`
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
- Exact release-binary provenance for reruns.
- `mn_corn_h4` `dx2p5`, `dx1p25`, and `dx0p625` active trace reruns.
- Normalization-amplification test.
- Hour-edge aliasing CDF test.
- Raw unbinned outlet-hydrograph convergence test.
- Contract/profile/BEI checks if `SC-OFEROUTE-001` changes.
- Focused contract-derived tests if contract text lands.
- Focused Lane D / `ofe_routing` tests if Rust code changes.
- `cargo fmt --check`.
- Full closure gates only if contract or production Rust text lands:
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full`, and `cargo deny check`.
- Source-level anti-evasion guards if required-case bindings, cohort fixtures,
  or external-authority suite posture are touched.

## Exit Criteria

`EXECUTED-COMPLETE` requires:
- The day-792 miss is classified by the three discriminating tests.
- Any metric repair is contract-first and rerun evidence shows the repaired
  adequacy gate result.
- No production mesh-policy flip lands in this package.
- Reviews, disposition, verification, gates, and final handoff are complete.

`EXECUTED-HOLD-*` is required when:
- Raw hydrograph evidence shows solver/day nonconvergence.
- Required raw-hydrograph evidence cannot be produced in-envelope.
- Metric repair is not contract-authorized inside this package.
- Reviews or verification leave a blocker open.

## Final Outcome

This package exits at `EXECUTED-HOLD-SOLVER-CLASS-DAY792`. Day 792 lane 1 is
not metric-class under the binding tests: absolute hourly mass movement is not
noise-scale, hourly CDF distance does not converge, and raw outlet-hydrograph
evidence also worsens on the fine rung pair. No `SC-OFEROUTE-001` amendment,
target-`dx` promotion, or production mesh-policy flip landed.
