# Implementation Evidence

Evidence class: Static + Ran.

## Diagnostic Tool

- Added `tools/snowfreeze_observed/post_partition_residual_decomposition.py`.
- The tool consumes:
  - 10.3.20 current-default real-run artifact:
    `docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/artifacts/sublimation-stage-b-unlock.json`.
  - 10.3.18 pre-partition rubric artifact:
    `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/artifacts/cross-snotel-mechanism-rubric.json`.
- Output schema:
  `snowdensity10-3-21-post-partition-residual-decomposition-v1`.
- Evidence mode is `Static + Reused Ran`: the 10.3.20 source artifact is the
  real direct-production WAT/trace run for the current default.

## Diagnostic Result

- Current default: `15` robust fails / `179` robust score.
- Pre-partition activated bundle: `17` robust fails / `172` robust score.
- Legacy flag profile: `16` robust fails / `176` robust score.
- Dominant fail signature: `seasonal_densification_trajectory` (`9/15`).
- Depth-SWE geometry fails: `2/15`, both humid-New-England paired rows.
- Timing under-persistence fails: `4/15`, all mountain SNOTEL timing rows.
- Over-persistence timing fails: `0`.
- Frost-threshold input read: `MIXED-NO-SINGLE-GLOBAL-SNOW-LEVER`.

## Protected Boundaries

- No production/default change.
- No selector, parser/runfile/user CLI, or `.run` option added.
- No density-cap, output-schema, fixture, frost, Qwet/frzftp, or site-calibration
  change.
- Legacy and PySnobal remain ADR-0017 flags, not targets.
