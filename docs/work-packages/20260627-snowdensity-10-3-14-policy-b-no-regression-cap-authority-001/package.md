# SNOWDENSITY-10.3.14 Policy-B No-Regression And Cap Authority

Status: complete
Owner: Codex
Date: 2026-06-27

## Objective

Execute the activation-policy diagnostic left by SNOWDENSITY-10.3.13: prove or
block Policy-B full-surface no-regression evidence for the current best bundle
and settle whether the `550 kg m^-3` SNOBAL cap re-anchor belongs in the same
activation path.

## Context

SNOWDENSITY-10.3.13 found the bundle
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1` remains
strictly better than current default on paired observed snow-depth rows
(`1147 -> 498`) but activation was blocked by missing full-surface
no-regression evidence. It also left the active `522 kg m^-3` density cap in
place and marked `550 kg m^-3` as a separate authority question.

## Required Reading

- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.3
- `docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001/artifacts/claude-review-activation-policy-b.md`
- `docs/work-packages/20260627-snowdensity-10-3-13-residual-policy-b-diagnostic-001/artifacts/closeout.md`

## Scope

In scope:

- Add `SC-SNOWFREEZE-001` v100 authority for the 10.3.14 diagnostic.
- Quantify active-cap and projected-`550 kg m^-3` cap-pinned residual behavior
  from real direct-production bundle WAT and trace lineage.
- Classify the `550 kg m^-3` cap as activation prerequisite, follow-up, or
  blocked based on evidence.
- Record Policy-B full-surface no-regression evidence for the active-cap bundle.
- Run the package diagnostic and focused contract tests.
- Run the workspace no-regression gate under the existing package-bound opt-in
  selectors.
- Update strategy/catalog and closeout artifacts.

Out of scope:

- Default activation.
- Changing `INV-SNOWFREEZE-003` density cap or any runtime cap constant.
- Adding parser/runfile/user selectors or output schema fields.
- New snow physics, coefficient tuning, fixture edits, open-surface ablation,
  shallow-pack guards, phase/canopy/radiation/longwave/rain-heat/frost changes,
  Qwet/frzftp, or compatibility-runtime changes.
- Frost attribution.

## Phase Plan

1. Scaffold package and contract-first amendment.
2. Add a diagnostic report generator and focused integration test.
3. Execute cap-pinned residual and composite trace-state diagnostics.
4. Run Policy-B no-regression gates under the current bundle selectors.
5. Update strategy/catalog and package closeout artifacts.

## Exit Criteria

- `SC-SNOWFREEZE-001` carries `INV-SNOWFREEZE-071` and
  `OBL-SNOWFREEZE-P-046`.
- Diagnostic report is generated in `artifacts/`.
- Report distinguishes:
  - active `522 kg m^-3` bundle activation readiness;
  - projected `550 kg m^-3` cap sensitivity;
  - actual default/cap mutation status.
- `550 kg m^-3` is not promoted without a real dynamic cap implementation and
  full Policy-B no-regression evidence.
- No production cap/default/schema/fixture/runtime-selector changes are made.
- Focused gates pass:
  - `.venv/bin/python tools/snowfreeze_observed/policy_b_no_regression_cap_authority.py`
  - `cargo test --test snowdensity10_3_14_policy_b_no_regression_cap_authority`
  - `cargo clippy --test snowdensity10_3_14_policy_b_no_regression_cap_authority -- -D warnings`
- Full closure gates pass:
  - `cargo fmt --check`
  - `cargo test --workspace`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo deny check`

## Status Log

- 2026-06-27: Scaffolded package for Policy-B no-regression and cap-authority
  diagnostic.
- 2026-06-27: Executed diagnostic and selector-scoped workspace gate. Result:
  `READY-FOR-ACTIVATION-PACKAGE-UNDER-ACTIVE-CAP`; the active cap remains
  `522 kg m^-3`, and the projected `550 kg m^-3` cap remains follow-up only.
