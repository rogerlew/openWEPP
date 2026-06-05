# Disposition

Status: complete
Evidence mode: Static + Ran

## Final Status

Static:

- Package status: `executed-hold`.
- Scoped objective closed: WB13 `RM` now consumes explicit fail-closed
  `snow.post_winter_rain_m + snow.routed_melt_m + Irr`.
- Semantic parity remains open: full H1..H39 semantic pass is `0/39`.

## Closure Evidence

Ran:

- Full H1..H39 root:
  `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix`.
- Runtime pass: `39/39`.
- Semantic pass: `0/39`.
- Target traces:
  `/tmp/hphys0290_target_traces_current_20260605T011834Z_postfix`.
- Final gates:
  `/tmp/hphys0290_final_gates_20260605T013019Z_after_nan/status.tsv`, all
  return codes `0`.

## Scientific Disposition

Static:

- HPHYS0290 proves the H39 2014-146 `RM=2.62 mm` row is not a WB13 raw-precipitation inference defect after this package.
- Pinned baseline `contin.for` restores `warain` for warm rain/no snow after
  clearing `rain(iplane)`, and openWEPP now publishes that equivalent as
  `snow.post_winter_rain_m`.
- Remaining `RM`, `Q`, `Snow-Water`, `Total-Soil`, `SoilWaterTotal`, `Dp`,
  `latqcc`, and `Ep` residuals should be pursued upstream in snowpack
  timing/state and runoff/storage partition lineage.

## Review / Verification

Static + Ran:

- Dual review complete: `review_agent_a.md`, `review_agent_b.md`.
- Review disposition complete: `review-disposition.md`.
- Dual verification complete: `verification_agent_a.md`,
  `verification_agent_b.md`.
- No blocking findings remain undispositioned.

## Continuation Recommendation

Static:

- Scaffold the next package around upstream snowpack timing/state and liquid
  partition coupling, not WB13 publication math.
- Include a focused lifecycle regression that proves same-day producer fluxes
  are written before WB13 and absent producer flux fails closed.
- Preserve the HPHYS0290 `snow.post_winter_rain_m` contract and do not
  reintroduce state defaults or raw-precipitation reconstruction in WB13.
