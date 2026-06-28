# SNOWDENSITY-10.3.12 Bundle Activation Adjudication

Status: complete
Owner: Codex
Date: 2026-06-27

## Objective

Adjudicate the current best combined snow-depth bundle:
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`.

The package must decide whether the bundle is eligible for activation, hold, or
retirement. It must rerun or reconstruct the real coupled direct-production WAT
evidence, compare against the prior liquid-holding-only and spring-densification
evidence, classify the remaining snow-control failures, and preserve all opt-in
boundaries unless Activation Policy B is satisfied. Policy B requires strict
improvement over the current default on gate-eligible paired-snow surfaces plus
full-model-surface no-regression evidence; it does not require zero paired snow-
depth failures.

## Context

SNOWDENSITY-10.3.8 proved `coe_liquid_holding_capacity_v1` improves coupled WAT
snow-depth failures `1147 -> 761` but does not clear snow control. SNOWDENSITY-
10.3.11 proved that adding existing `physics_bulk_density_compaction_v1` composes
positively with that melt/liquid boundary (`761 -> 498`), while the new
`physics_bulk_spring_densification_v1` over-densifies into under-persistence and
is a non-promotion (`498 -> 502`).

The active question is therefore not another compaction-rate variant. The active
question is whether the `coe_liquid_holding_capacity_v1 +
physics_bulk_density_compaction_v1` bundle is activation-ready under Policy B,
remains opt-in only, or should be retired. Frost attribution is a separate gate:
remaining snow-depth residuals keep frost attribution blocked even if a later
full-surface activation package authorizes the bundle as the default.

## Required Reading

- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3
- `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/package.md`
- `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/artifacts/liquid-holding-capacity-coupled-wat.json`
- `docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/artifacts/spring-pack-depletion-compaction-adjudication.json`
- `docs/work-packages/20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001/package.md`
- `docs/work-packages/20260627-snowdensity-10-3-11-spring-compaction-densification-candidate-001/artifacts/spring-compaction-densification-candidate.json`
- `tools/snowfreeze_observed/spring_compaction_densification_candidate.py`

## Scope

In scope:

- Amend `SC-SNOWFREEZE-001` to record the bundle adjudication authority,
  activation boundary, and residual-classification requirements.
- Add a diagnostic tool that runs the real direct-production WAT path with
  `OPENWEPP_SNOWDENSITY1038_MELT_MODEL=coe_liquid_holding_capacity_v1` and
  `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_density_compaction_v1`.
- Prove both selected opt-in members reached the direct snow partition through
  trace evidence.
- Compare against prior default/holding-only/spring-densification evidence.
- Classify the remaining paired snow-depth failures by surface, cover, month,
  residual sign, March/April cap class, and observation-blocked status.
- Decide activation eligibility truthfully:
  - `ACTIVATION-READY` only if Policy B is satisfied: strict gate-eligible snow
    improvement versus current default plus full-model-surface no-regression.
  - `HOLD-OPT-IN-BUNDLE` if the bundle improves but lacks full-surface evidence.
  - `RETIRE-BUNDLE` if it worsens the accepted baseline.

Out of scope:

- Default activation unless the current package evidence satisfies Policy B.
- Parser, runfile, user CLI, compatibility-runtime, fixture, public output
  schema, coefficient, canopy, radiation, phase partition, rain heat,
  sub-canopy longwave, frost, Qwet/frzftp, or density-cap changes.
- New physics levers, including open-surface ablation, cap changes, or another
  compaction-rate candidate.
- Treating observation-blocked surfaces as verdict-bearing.
- Unblocking frost attribution while paired snow-control failures remain.

## Phase Plan

1. Scaffold package, evidence placeholders, and required-reading artifacts.
2. Amend `SC-SNOWFREEZE-001` with `INV-SNOWFREEZE-069`,
   `OBL-SNOWFREEZE-P-044`, boundary disposition, and a 10.3.12 addendum.
3. Add a focused integration test binding contract/package/tool/report evidence.
4. Add and run the bundle activation adjudication tool.
5. Update strategy and work-package catalog with the disposition.
6. Complete gate results, review/disposition, verification, line-count, and
   closeout artifacts.

## Exit Criteria

Closure may be `complete` only if:

- Contract amendment is present before any package closure.
- The real direct-production WAT path runs or a current report proves why rerun
  is unnecessary. If current direct evidence cannot be produced, close `HOLD`.
- Trace proof counts the selected melt and density models in the direct snow
  partition.
- The report compares default/holding-only/bundle/spring-densification counts.
- Remaining failures are classified, including under-persistence and
  over-persistence/residual mass indications.
- Activation is rejected unless Policy B is satisfied.
- Frost attribution remains blocked if paired snow-control residuals remain
  large enough to prevent isolating frost residuals.
- Protected boundaries are explicitly reported as unchanged.
- Focused gates pass:
  - `.venv/bin/python tools/snowfreeze_observed/bundle_activation_adjudication.py`
  - `cargo fmt --check`
  - `cargo test --test snowdensity10_3_12_bundle_activation_adjudication`
  - `cargo clippy --test snowdensity10_3_12_bundle_activation_adjudication -- -D warnings`

## Status Log

- 2026-06-27: Scaffolded package for the current best bundle activation
  adjudication.
- 2026-06-27: Added `SC-SNOWFREEZE-001` v97 with `INV-SNOWFREEZE-069`,
  `OBL-SNOWFREEZE-P-044`, and the 10.3.12 bundle activation addendum.
- 2026-06-27: Ran seven real direct-production WAT bundle executions with
  `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`.
  Closed `HOLD-OPT-IN-BUNDLE`: the bundle improves default and holding-only
  evidence (`1147 -> 761 -> 498`) and beats spring densification (`502`), but
  default activation remains blocked under Policy B because full-model-surface
  no-regression evidence was not produced. `498/1415` paired rows still fail
  snow control, so frost attribution remains separately blocked.
- 2026-06-27: Incorporated `artifacts/claude-review-activation-policy-b.md`.
  `SC-SNOWFREEZE-001` v98 now supersedes the zero-paired-failure activation
  criterion with Policy B and records the residual tails as diagnostic/frost-
  attribution blockers rather than the sole activation blocker.
