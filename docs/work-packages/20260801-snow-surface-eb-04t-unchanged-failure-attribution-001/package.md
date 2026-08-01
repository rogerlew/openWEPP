# SNOW-SURFACE-EB-04T Unchanged-Failure Attribution And Criterion Fitness

Status: `complete / DIAGNOSTIC_COMPLETE`

Date: `2026-08-01`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Retained-evidence scientific diagnostic / governance assessment`

This living ExecPlan follows `docs/codex_exec_plans.md`.

## Purpose / Big Picture

Explain why all 16 forcing-robust failures remained failures when combined
sub-canopy longwave and sublimation improved the aggregate canopy score. Map
each failure to its observation operator, numerical response, governing process
family, and target-mechanism sensitivity, then determine whether total-failure
reduction was a scientifically fit promotion requirement.

## Objective

Analyze the immutable EB-04S retained adjudication without rerunning the model.
For every baseline robust failure, compare B/L/S/LS primary metrics and ordinal
labels; distinguish exact label invariance from metric movement; assign bounded
process ownership using existing snow contracts and the prior post-partition
residual decomposition; and recommend the next evidence route without changing
EB-04S or activating defaults.

## Implementation Intent

Intent: `diagnostic failure attribution` plus `promotion-criterion fitness`.

Calibration intent: `NOT_APPLICABLE`. No coefficient, rubric boundary,
observation, forcing, fixture, process equation, selector, or default changes.

## Included Scope

- the 16 EB-04S baseline forcing-robust failures and their B/L/S/LS metrics;
- density trajectory, depth-SWE geometry, peak timing, and melt-out timing;
- process-family ownership, open/control role, and mechanism sensitivity;
- direction-of-change analysis against each observation operator's target;
- criterion-fitness assessment and prospective evidence recommendations;
- accessible SVG figures with Markdown sidecars;
- roadmap, catalog, reviews, verification, and final disposition.

## Excluded Scope

- model execution, calibration, coefficient fitting, threshold rewriting, new
  observations, or a new empirical promotion decision;
- retroactive EB-04S pass, default activation, or broad transfer claims;
- production Rust, tests, fixtures, observations, canonical contract changes,
  or edits to EB-04R/04S;
- treating legacy/comparator similarity as correctness authority.

## Dependencies

- committed EB-04S retained adjudication and version-6 snow-energy authority;
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-050` observation rubric;
- prior post-partition residual decomposition and its process-family readings;
- immutable EB-04R prospective protocol and B/L/S/LS meanings.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

All production, tests, contracts, fixtures, observations, prior packages, and
retained runtime outputs are read-only.

## Phase Plan

1. Scaffold, bind exact retained inputs, freeze failure taxonomy and
   direction-of-improvement operators, and self-check the analytical consumer.
2. Reconstruct all 16 failures across B/L/S/LS and quantify main/combined
   metric responses without inventing new thresholds.
3. Attribute process ownership and target sensitivity from canonical/prior
   authority; assess criterion fitness and alternative prospective routes.
4. Produce figures/sidecars, scientific synthesis, and claim limitations.
5. Run selected validation, dual review, finding disposition, dual
   verification, roadmap/catalog reconciliation, and final disposition.

## Frozen Diagnostic Rules

- A failure is in scope only if B is forcing-robust and labeled `fail` in the
  committed EB-04S report.
- Ordinal transition uses the unchanged rubric label; no threshold is changed.
- Numerical response reports the primary scalar already used by that rubric
  cell: KGE for densification, distance of slope ratio from one for depth-SWE,
  and absolute median day offset for timing.
- A response is `toward_observation`, `away_from_observation`, or `unchanged`
  only by exact reduction in the existing primary error magnitude. No material-
  effect threshold or new pass boundary is introduced.
- Process ownership distinguishes `primary`, `mixed`, and `indirect`; it does
  not claim unique causality from factorial association.
- Criterion fitness distinguishes density/geometry and mixed interception
  guards from open-control timing cells. The latter identify sublimation and
  combined-bundle behavior, not canopy-longwave efficacy. EB-04S remains
  authoritative for its own decision regardless of the fitness assessment.

## Acceptance Criteria

1. Exact EB-04S report, protocol, contract, and prior decomposition identities
   are hash-bound before analysis.
2. Exactly 16 baseline failures reconstruct, with counts `9/2/2/2/1` across the
   five known signatures and unchanged fail labels in every B/L/S/LS cell.
3. Every failure retains B/L/S/LS primary metric, error magnitude, direction,
   process owner, target sensitivity, and authority reference.
4. The analysis reports favorable, unfavorable, and unchanged movements rather
   than treating label invariance as numerical invariance.
5. Criterion-fitness conclusions distinguish implementation correctness,
   empirical efficacy, inherited process debt, and transferability.
6. Figures have one readable Markdown sidecar each.
7. No model subprocess runs and every read-only input remains unchanged.
8. Validation, dual review, disposition, dual verification, exact-diff,
   security, roadmap/catalog, and handoff evidence pass.

Any unmet current-scope criterion forces `HOLD`.

## Validation Requirements

- Python syntax, frozen-input/self-check, exact 16-row inventory, and rejected-
  alias controls;
- deterministic analysis regeneration and independent summary reconstruction;
- scoped Markdown and SVG parsing;
- production/test/prior-package empty diff, exact write-set, security, and
  line-count checks;
- dual independent science/governance review and dual terminal verification.

No Rust regression is selected because this package is read-only analysis and
documentation over committed evidence.

## Security Impact Gate

No network, secret, authentication, unsafe Rust, dependency, external write,
subprocess/model execution, or public schema change is authorized.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/governance reviewers
and two read-only terminal verification agents. Expected outputs are bounded
package-local review and verification artifacts; no other write access.

Subagent requirement: REQUIRED for scientific attribution and terminal
verification. No comparator or heavy test runner is selected.

## Progress

- [x] (2026-08-01) User selected EB-04T after EB-04S and required the prior
  stable increment to be committed/pushed first (`4e5c2022`).
- [x] (2026-08-01) Exact 16-failure inventory and B/L/S/LS primary-metric
  response reconstructed from committed EB-04S evidence.
- [x] (2026-08-01) Process attribution and criterion-fitness assessment
  completed without model execution or threshold changes.
- [x] (2026-08-01) Three accessible SVG figures and Markdown sidecars generated
  deterministically.
- [x] (2026-08-01) Dual independent reviews completed; all findings accepted
  and resolved.
- [x] (2026-08-01) Dual terminal verification, roadmap/catalog reconciliation,
  exact-diff, documentation, figure, and final disposition gates passed.

## Decision Log

- Decision: preserve EB-04S and use retained evidence only.
  Rationale: EB-04T diagnoses the decision; it is not another empirical round.
  Date/Author: 2026-08-01 / Codex.

## Outcomes & Retrospective

All 16 baseline forcing-robust failures retain the `fail` label under B/L/S/LS,
and no complete metric object is identical between B and LS. Under LS, 15
selected primary errors move away from their observation target and one is
unchanged. This exact sign classification has no materiality threshold. Eleven
failures are density/geometry or mixed interception-geometry diagnostics,
confirming mixed alignment of the total-failure gate. The five peak/melt-out
timing failures are all open SNOTEL controls: four move away and one is
unchanged. They diagnose sublimation and combined-bundle noninferiority, but
cannot identify canopy-longwave efficacy.

Thirteen failures have nonzero factorial interactions. Most interactions
mitigate S-associated adverse movement, but the Niwot peak-depth interaction
exactly cancels a favorable four-day S-only response. Thus the combined bundle
did not resolve these failures, while the factorial does not uniquely assign
their cause to sublimation.

The combined model remains implemented and physically qualified with narrow
canopy benefit, but this retained evidence does not support retroactive or
limited default promotion. A future promotion rule should prospectively
separate canopy-longwave efficacy, sublimation efficacy, and adjacent-process
noninferiority using mechanism-identifying independent evidence.
Density/geometry and mountain under-persistence should be routed to their
owning process/forcing investigations rather than tuning longwave or
sublimation.
