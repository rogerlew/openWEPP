# SNOW-SURFACE-EB-04S Authority Reconciliation And Retained-Output Adjudication

Status: `complete / CLOSE_NONPROMOTION_EMPIRICAL_RULE / terminally verified`

Date: `2026-08-01`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Science-authority amendment / retained-evidence adjudication ExecPlan`

This living ExecPlan follows `docs/codex_exec_plans.md`.

## Purpose / Big Picture

Resolve the cross-unit tolerance contradiction that truthfully held EB-04R,
then adjudicate its immutable 48-cell output population without rerunning the
model. The authority decision is frozen before any EB-04R result or observation
is read. EB-04R remains an unchanged historical HOLD.

## Objective

First, derive the water-equivalent-to-area-mass tolerance using only canonical
unit governance, the named production conversion, the pre-result EB-04E
qualification protocol, and pre-amendment `SC-SNOWENERGY-001`. Freeze and
independently review that decision. Amend the canonical contract so both units
are explicit. Only then rehash and independently reconstruct the retained
EB-04R physical/provenance gates and, if they pass, apply the unchanged frozen
observation rubric and decision rule.

## Implementation Intent

Intent: `science-authority reconciliation` plus `retrospective retained-output
independent-validation adjudication`.

Calibration intent: `NOT_APPLICABLE`. No coefficient, forcing, observation,
rubric, physical output, threshold magnitude in water-equivalent units, process
equation, selector, fixture, or default may be fitted or changed.

## Bias Firewall

Phase A may read only the authority whitelist in
`artifacts/authority-input-manifest.md`. It must not read EB-04R traces, WAT,
provenance, result reports, scores, terminal audit, or residual magnitudes.
The authority receipt is frozen and dual-reviewed before Phase B starts.

Phase B may read EB-04R retained outputs only after the canonical contract
amendment and authority receipt pass review. It may not invoke a simulation
binary or modify EB-04R. The known diagnostic score is explicitly quarantined
from Phase A and carries no authority over the unit decision.

## Included Scope

- a machine-readable authority-only derivation and freeze receipt;
- explicit `1e-9 m SWE == 1e-6 kg m^-2` closure-tolerance authority in
  `SC-SNOWENERGY-001`;
- hash/provenance and independent physical reconstruction of the immutable
  EB-04R 12-lane by four-cell population;
- unchanged post-gate scoring, protected-group, compensation, and stop-loss
  rules from EB-04R;
- package evidence, roadmap, and catalog reconciliation.

## Excluded Scope

- any new model execution, scientific retry, production/test/fixture change,
  coefficient tuning, observation change, rubric change, or default activation;
- rewriting EB-04R evidence or retroactively changing its HOLD;
- using EB-04R results, residuals, or scores to choose the authority outcome;
- an EB-05 assurance report or a new empirical calibration round.

## Dependencies

- `SC-SNOWENERGY-001` version 5 and canonical unit governance;
- named `snow_water_equivalent_meters_to_area_mass_kg_m2` conversion;
- EB-04E's prospectively frozen `1e-9 m` mass and `1e-6 kg m^-2`
  vapor/sublimation closures;
- EB-04R's immutable attempt, per-file provenance, traces, WAT, observations,
  rubric, and decision dependencies.

## Intended Write Set

- this package tree;
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`.

Production Rust, tests, fixtures, observations, prior packages, and retained
runtime output are read-only. Any need outside this write set forces a
prospective amendment before editing.

## Phase Plan

1. Scaffold and freeze the authority whitelist, dimensional derivation, and
   package write set without opening EB-04R result-bearing evidence.
2. Complete two independent authority reviews; disposition findings; amend and
   verify the canonical contract.
3. Seal the authority decision, then rehash and reconstruct all 48 retained
   cells without launching the model.
4. If and only if the physical/provenance population gate passes, reconstruct
   the unchanged observation rubric and eight-part decision rule.
5. Complete dual terminal review, finding disposition, dual verification,
   validation, exact-diff, roadmap/catalog, and final disposition.

## Acceptance Criteria

1. The authority receipt lists only whitelisted pre-result inputs and proves
   `1e-9 m * 1000 kg m^-3 = 1e-6 kg m^-2`.
2. Two independent reviews accept the authority decision before Phase B.
3. Contract version 6 states the equivalent tolerances and distinguishes them
   from the `1e-9 kg m^-2` represented-layer lifecycle boundary.
4. No simulation subprocess is launched and EB-04R remains byte-unchanged.
5. All 48 cells and every retained file identity reconcile to the EB-04R
   attempt/provenance before physical or empirical acceptance.
6. Observation access is logically downstream of the complete physical gate.
7. Empirical disposition follows the unchanged EB-04R rule without tuning or
   post-result amendment.
8. Selected validation, dual review, disposition, dual verification,
   exact-diff, security, and handoff evidence pass.

Any unmet current-scope criterion forces `HOLD`.

## Conservation / Publication Acceptance

The retained-output consumer must independently reconstruct WAT/trace/layer,
snow mass, energy, vapor aggregation, and vapor-to-sublimation identities. The
area-mass tolerance is derived from canonical `1e-9 m` SWE using the named
`1000 kg m^-3` conversion. Producer-carried residuals and prior generated
scores are corroborating evidence only.

## Validation Requirements

- authority-tool syntax, input whitelist, forbidden-path, dimensional, and
  deterministic self-checks;
- science-contract schema/profile and unit-governance gates applicable to the
  documentation-only clarification;
- retained 48-cell hash/provenance and independent physical reconstruction;
- unchanged decision reconstruction after the physical gate;
- scoped Markdown, exact-diff, security, and line-count checks;
- dual independent contract/science review and dual terminal verification.

No Rust source changes occur, so no Rust build or workspace regression is
selected. EB-04R already binds the executed binary and fresh Critical full run;
this package changes authority text and consumes immutable evidence only.

## Security Impact Gate

No network, secret, authentication, unsafe Rust, dependency, external write,
or public schema change is authorized. The tool reads only repo files and the
existing `target/snow_surface_eb04r_factorial` output tree and writes only this
package's artifacts.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science-contract reviewers and
two independent read-only terminal verification agents. Expected outputs are
package-local review and verification artifacts; write access is bounded to
those named artifacts only.

Subagent requirement: REQUIRED for the contract revision and terminal science
disposition. No comparator or heavy test runner is selected because no runtime
execution or production change is authorized.

## Progress

- [x] (2026-08-01) User authorized authority reconciliation plus retained-output
  adjudication.
- [x] (2026-08-01) Authority-only receipt frozen and independently accepted.
- [x] (2026-08-01) Canonical contract version 6 amended and dual-verified.
- [x] (2026-08-01) All 48 retained outputs independently adjudicated without a
  model subprocess; physical/provenance gate passed.
- [x] (2026-08-01) Terminal review findings repaired; dual terminal verification,
  roadmap/catalog, and final disposition complete.

## Decision Log

- Decision: preserve EB-04R as an unchanged HOLD and create a successor record.
  Rationale: a frozen protocol controls its own experiment even when later
  authority shows a transcription error.
  Date/Author: 2026-08-01 / Codex.
- Decision: use a two-phase read firewall rather than rerunning the population.
  Rationale: runtime output is observation-independent and already sealed;
  rerunning would add cost without new scientific information.
  Date/Author: 2026-08-01 / Codex.

## Surprises & Discoveries

- The first full retained-analysis pass completed physical reconstruction but
  the inherited pure report reducer attempted a read-only `git` subprocess.
  EB-04S's no-process guard stopped it before a report was written. The
  successor now binds that metadata directly to the immutable attempt record;
  no model or other subprocess ran.
- Terminal review found that complete pre-score identity binding needed to be
  explicit in the successor rather than inferred from per-cell provenance. It
  also recovered EB-04R's package-specific `1e-12 kg m^-2` vapor-aggregation
  gate. Both repairs were added before final adjudication, and independent full
  replay reproduced the same nonpromotion outcome.

## Outcomes & Retrospective

Phase A read exactly four pre-result authority inputs and froze the dimensional
identity `1e-9 m SWE == 1e-6 kg m^-2`. Both independent reviewers required and
verified narrow operand wording: vapor aggregation and represented-layer
lifecycle retain separate `1e-9 kg m^-2` predicates. Canonical
`SC-SNOWENERGY-001` version 6 now exposes those distinctions.

Phase B launched no model subprocess. It rechecked 48 cell provenance records,
288 retained file identities, the complete 761,212-row physical population,
and unchanged EB-04R scoring. All physical/provenance gates pass at the
authority-bound transfer tolerance; the maximum vapor-to-sublimation residual
is `8.109983287707401e-8 kg m^-2`. Combined LS improves the robust ordinal score
from 177 to 180 but leaves robust failures at 16. The frozen rule therefore
returns `CLOSE_NONPROMOTION_EMPIRICAL_RULE`, invokes the stop-loss, and
authorizes no further factorial/calibration round. EB-04R remains an unchanged
historical HOLD.
