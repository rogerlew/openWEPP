# Worker Handoff

Status: complete
Evidence mode: static + ran

## Current State

HPHYS0291 is executed-hold.

Closed:

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-024`
- `SC-RUNOFFPART-001#INV-RUNOFFPART-021`
- `SC-WATBAL-001#INV-WATBAL-066`
- runoff reconciliation same-day publication of `snow.routed_melt_m`
- runoff reconciliation same-day publication of `snow.post_winter_rain_m`
- WB13 flux-only consumption of both snow publication terms
- trace/localization flux-only reporting for both snow publication terms

Open:

- H1..H39 semantic parity remains `0/39`.
- `Snow-Water`, `RM`, `Q`, `Total-Soil`, `SoilWaterTotal`, `Ep`, `Dp`, and
  `latqcc` residuals remain continuation scope.

## Key Evidence

- Full-suite metrics:
  `docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/full-39-suite-metrics.md`
- Target traces:
  `docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/h1-h7-h39-trace-evidence.md`
- Localization:
  `docs/work-packages/20260605-hphys0291-snow-publication-lifecycle-partition-localization-closure-001/artifacts/snow-publication-lifecycle-localization.md`
- Final gates:
  `/tmp/hphys0291_final_gates_post_review_20260605T023206Z`

## Recommended Next Package

Scaffold an iterative contract-first package focused on baseline-authoritative
snow/liquid partitioning upstream of WB13:

1. Establish canonical contract authority for winter/contin routed melt,
   rain-on-snow retention/release, and runoff/infiltration partition ownership.
2. Add contract-derived tests that compare producer fluxes before WB13 against
   expected partition surfaces for dry/no-snow, warm-rain/no-snow, retained
   rain-on-snow, and active-melt cases.
3. Trace H1/H7/H39 through snowpack state, `wmelt`, retained/released rain,
   runoff reconciliation input, same-pass infiltration, `Q`, and WB13 `RM`.
4. Run the full H1..H39 suite and keep disposition in HOLD until semantic
   residual ownership is proven.

Guardrail:

- Do not reintroduce WB13 inference or flux-preferred fallback for
  `snow.post_winter_rain_m` or `snow.routed_melt_m`.
