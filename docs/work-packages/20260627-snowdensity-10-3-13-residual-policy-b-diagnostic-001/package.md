# SNOWDENSITY-10.3.13 Residual Tail And Policy-B Diagnostic

Status: complete
Owner: Codex
Date: 2026-06-27

## Objective

Classify the residual tails left by the current best bundle
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`, and
define the missing Activation Policy B full-surface evidence needed before any
default activation package can proceed. Policy-B full-model-surface no-regression evidence is the binding activation gap.

## Context

SNOWDENSITY-10.3.12 closed `HOLD-OPT-IN-BUNDLE`: the bundle improves the current
default on gate-eligible paired-snow rows (`1147 -> 498`) and worsens no paired
surface relative to holding-capacity-only, but it did not produce Policy-B
full-model-surface no-regression evidence. The residuals are now nearly two-
sided (`264` modeled-over-observed and `234` modeled-under-observed), so the
next safe step is diagnostic attribution, not another compaction-rate or melt
lever.

## Required Reading

- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.3
- `docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001/artifacts/closeout.md`
- `docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001/artifacts/claude-review-activation-policy-b.md`

## Scope

In scope:

- Add `SC-SNOWFREEZE-001` v99 authority for this diagnostic.
- Add a diagnostic tool that consumes committed real direct-production WAT
  report lineage from 10.3.8, 10.3.11, and 10.3.12.
- Pair WAT rows to observations by date and classify state transitions across:
  default, holding-capacity-only, combined bundle, and spring densification.
- Attribute bundle under-persistence as persisted, induced from pass, induced
  from over-persistence, or unresolved.
- Attribute bundle over-persistence through March/April cap classes under the
  active `522 kg m^-3` cap.
- Publish a Policy-B full-surface no-regression evidence matrix.
- Update the snow/frost strategy and work-package catalog.

Out of scope:

- Default activation.
- Production physics changes.
- Density-cap changes, including `550 kg m^-3` SNOBAL cap re-anchoring.
- New compaction-rate, ablation, melt, phase, canopy, longwave, rain-heat,
  frost, Qwet/frzftp, fixture, parser/runfile/user, output-schema, or
  compatibility-runtime changes.
- Frost attribution.

## Phase Plan

1. Scaffold package and contract amendment.
2. Add diagnostic report generator and focused integration test.
3. Execute the diagnostic against the existing real WAT report lineage.
4. Update strategy/catalog and package artifacts.
5. Run focused gates plus workspace checks.

## Exit Criteria

- `SC-SNOWFREEZE-001` carries `INV-SNOWFREEZE-070` and
  `OBL-SNOWFREEZE-P-045`.
- Diagnostic report is generated in `artifacts/` with date-level residual
  transition evidence.
- Report names the activation blocker separately from the frost-attribution
  blocker.
- Report preserves the active `522 kg m^-3` cap and records any `550 kg m^-3`
  consideration only as follow-up.
- No production physics/default/schema/fixture/runtime-selector changes are
  made.
- Focused gates pass:
  - `.venv/bin/python tools/snowfreeze_observed/residual_policy_b_diagnostic.py`
  - `cargo test --test snowdensity10_3_13_residual_policy_b_diagnostic`
  - `cargo clippy --test snowdensity10_3_13_residual_policy_b_diagnostic -- -D warnings`

## Status Log

- 2026-06-27: Scaffolded diagnostic package and amended
  `SC-SNOWFREEZE-001` to v99.
- 2026-06-27: Executed residual transition and Policy-B diagnostic. Closed as
  diagnostic complete with `HOLD-ACTIVATION-EVIDENCE-MISSING` and frost
  attribution still blocked.
