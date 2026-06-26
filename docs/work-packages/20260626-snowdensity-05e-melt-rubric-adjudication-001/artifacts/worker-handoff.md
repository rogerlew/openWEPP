# Worker Handoff

Evidence mode: Static + Ran.

## Next Recommended Package

SNOWDENSITY-05F Melt Closure / Density Handoff.

## Required 05F Decisions

- Decide whether `coe_shortwave_albedo_v1` remains opt-in only or is ready for
  a bounded production selector surface.
- Ratify the opt-in albedo cold-start policy exposed by diagnostic replay:
  same-day future snowfall needs state continuity before active snow exists.
- Decide activation evidence baseline: diagnostic replay comparison,
  H as-built comparator context, or both.
- Preserve rollback to `legacy_coe`.
- Freeze the exact diagnostics density work can consume without retuning melt.
- Do not default-activate on 05E alone.

## 05E Evidence To Carry Forward

- Opt-in beats diagnostic legacy by rubric rule:
  `robust_fail_count 13 -> 10`, `robust_ordinal_score 61 -> 84`.
- H as-built context is not beaten:
  openWEPP/legacy as-built profiles are `robust_fail_count=9`,
  `robust_ordinal_score=84`.
- Non-SNOTEL frost attribution remains blocked by snow-depth control:
  `SNOW_CONTROL_FAILED=3`, no paired observed snow on two sites.
- `openwepp_defective_cells=0` remains the correct non-SNOTEL disposition.

## Guardrails

- No default activation without a dedicated activation package.
- No SNOTEL-fitted melt coefficients.
- No snow-only radiation rescaling.
- No density/pack physics changes inside melt closure.
