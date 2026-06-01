# External-Authority Promotion Protocol

Status: Active  
Last updated: 2026-05-31  
Scope: agent-driven promotion/demotion guards for suite lane/failure posture

## Purpose

Prevent evidence-set manipulation during authority-suite posture changes
(especially promotions to blocking gates).

## Required Protocol

For any suite posture change (`gate_lane` and/or `failure_class`):

1. **Red-first capture**
   - Preserve anchor fixtures/cases that represent known problem regimes.
   - Record explicit threshold-status classification for anchor cases.
2. **Fix phase**
   - Implement the production/runtime correction package.
3. **Green confirmation**
   - Re-run the same anchor cases and record post-fix status.
4. **Posture change**
   - Only then promote to `required`/`hard-fail`.

## Non-Negotiable Guard Rules

1. Do not remove anchor fixtures/cases during posture changes.
2. Do not loosen thresholds (`max_relative_error_threshold` increases are
   forbidden) during posture changes.
3. Do not shrink cohort case cardinality during posture changes.
4. Any lane/failure posture change must update:
   - this protocol file, and
   - package disposition evidence for the change.
5. Any Level-4 suite temporarily held in `periodic`/`investigation` posture
   must declare and maintain an active closure follow-on package in
   `docs/work-packages/README.md` until promotion closure is completed.

## Machine-Checked Sources

Agent review/implementation must run:

- `tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`

The obligations driving those checks are declared in:

- `docs/specifications/external-authority/required-suite-obligations.json`

For non-blocking Level-4 suites, obligations must include closure package
linkage fields:

- `closure_follow_on_package_id`
- `closure_follow_on_package_path`
- `closure_follow_on_queue_path`

## Posture Change Log

- 2026-05-31 (`AUTH12`): `cas_l4_soil_fc_direct_theta_minus33_cohort_001`
  promoted from `periodic`/`investigation` to `required`/`hard-fail` after
  anchored rocky-soil red/fix/green closure evidence.
