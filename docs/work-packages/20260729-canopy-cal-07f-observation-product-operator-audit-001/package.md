# CANOPY-CAL-07F Observation Product and Operator Audit

Status:
`complete / do not calibrate / ecosystem-model limitation adjudicated`

Evidence mode: `Ran + retained provisional observation products`

Intent:
`calibration-readiness adjudication; no empirical calibration or production change`

## Objective

Re-evaluate the CAL-07D Bezà chronology contradiction with separate,
checksum-bound `gcc_mean` and `gcc_90` observation lanes, their reported
uncertainty intervals, and the retained daily curves. Decide whether one
bounded calibration round is scientifically worthwhile or whether further
work should stop and the mismatch should be adjudicated as an ecosystem-model
limitation.

## Rationale

CAL-07E found that CAL-07 retained `gcc_mean` transition dates while CAL-07D
attached `smooth_gcc_90` daily context. Rising dates agree closely, but four
falling transitions differ by 12 to 43 days. Product choice cannot be made
according to which one fits the model.

The user set a stop-loss: execute this audit, then either authorize one bounded
calibration round or defer further work as a model limitation.

## Included scope

- Retained provisional PhenoCam Data Record 4 daily `smooth_gcc_mean` and
  `smooth_gcc_90` curves.
- Retained provisional Data Record 5 2024–2025 transition dates, thresholds,
  and confidence intervals.
- CAL-07D BASE ensemble event-year-relative and absolute crossing inventories.
- Product-specific residual, confidence-interval coverage, member-ranking,
  direction-bias, and year-consistency analysis.
- Daily-curve/transition visual inspection and exact source binding.
- Calibration-readiness versus ecosystem-model-limitation disposition.
- Plot-only SVG figures with Markdown caption/ancillary-information sidecars.

## Excluded scope

- Parameter fitting, new candidate generation, empirical calibration, or
  validation.
- Production Rust, science contracts, forcing, process equations, thresholds,
  or defaults.
- Selecting an observation product because it reduces residuals.
- Treating GCC as GSI, LAI, biomass, canopy cover, or physiological activity.
- Further literature, thesis, field-data, or institutional acquisition.

## Dependencies

- CAL-07E commit `643381ed2b42b0378d661b0deb0f04d2dbef7ef9`;
- CAL-07E retained provisional transition subset and provenance;
- CAL-07 retained provisional daily PhenoCam product, ROI, and metadata; and
- CAL-07D validated BASE crossing and attribution artifacts.

All observation rows remain `DIAGNOSTIC_ONLY`.

## Write set

- this CAL-07F package;
- CAL-07E `artifacts/acquisition-needed.md` for the user-directed deferral;
- `docs/planning/canopy-phenology-assurance-roadmap.md`; and
- `docs/work-packages/README.md`.

Production code, contracts, ADRs, and predecessor evidence other than the
single CAL-07E deferral record are read-only.

## Prospective decision rule

A bounded calibration round may be recommended only if every condition passes:

1. **Operator independence:** the recommendation does not select
   `gcc_mean` or `gcc_90` from model agreement; either conclusions are robust
   across both or an external method rule selects the operator.
2. **Crossing sufficiency:** at least one frozen member has same-year,
   same-direction, season-window crossings for all 12 T10/T25/T50 event rows
   in both products. Each product/year is split halfway between its falling
   T10 and rising T10 dates; earlier crossings belong to leaf-off and later
   crossings to leaf-on.
3. **Uncertainty fit:** at least one frozen member has at least 8 of 12
   crossings inside the reported confidence intervals in both products and
   median absolute residual no greater than 21 days in both.
4. **Direction coherence:** that member's median signed residual magnitude is
   no greater than 21 days separately for rising and falling events in both
   products.
5. **Parameter plausibility:** CAL-07D counterfactual evidence demonstrates a
   parameter-only direction capable of improving rising and falling timing
   together without removing transition crossings.
6. **Empirical role:** a calibration design can keep at least one year
   independent of fitting and does not relabel diagnostic evidence as external
   validation.

If any condition fails, CAL-07F must not recommend another calibration round.
It will disposition the retained contradiction as an ecosystem-model
limitation for the assessed tropical dry-forest lane, defer further canopy
work, and state a concrete reactivation trigger.

The 21-day tolerance matches the implemented GSI averaging window; it is an
adjudication tolerance, not observational uncertainty or a physiological
bound.

## Execution phases

1. Freeze source identities, product semantics, decision rules, and empirical
   roles.
2. Extract the retained daily curves and independently verify transition
   dates, thresholds, and confidence intervals.
3. Re-select same-year/same-direction model crossings inside the frozen
   product/year seasonal windows without refitting.
4. Compute residual, coverage, completeness, ranking, and direction-coherence
   evidence.
5. Produce accessible figures and sidecars.
6. Apply the prospective decision rule and record the limitation or
   calibration recommendation.
7. Validate exact inputs, outputs, figures, documentation, and terminal diff.
8. Complete dual independent review and verification, disposition findings,
   update roadmap/catalog, and close truthfully.

## Exit criteria

- Both products retain exact source identity and provisional status.
- All 12 product/event/threshold rows and confidence intervals reproduce.
- Daily curves and transition markers are visibly auditable.
- Every frozen member receives product-specific completeness, interval
  coverage, residual, and direction-bias results.
- The calibration decision reduces mechanically from the frozen rule.
- No product is promoted by fit.
- Figures render and have complete Markdown sidecars.
- Documentation/schema validation and exact-diff reconciliation pass.
- Dual terminal reviews and verifications pass or the package remains held.
- Roadmap/catalog state the stop-loss disposition.

## Security and production impact

No secrets, protected data, production runtime, dependency, serialization, or
security boundary is changed. Rust and workspace gates are not applicable.

## Delegated review authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two terminal science-review and verification subagents for independent
source reconstruction, result reduction, decision-rule, figure, citation, and
closure review. Expected outputs are `artifacts/review-agent-a.md`,
`artifacts/verification-agent-a.md`, `artifacts/review-agent-b.md`, and
`artifacts/verification-agent-b.md`. Write access is limited to those four
package artifacts.
