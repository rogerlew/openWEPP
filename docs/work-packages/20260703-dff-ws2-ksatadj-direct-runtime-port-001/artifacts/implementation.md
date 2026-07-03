# WS-2 Implementation Evidence

Evidence: Static + Ran. Executor: Codex. Date: 2026-07-03.

## Summary

Implemented the `SC-SUBHYD-001#INV-SUBHYD-032` `ksatadj = 1`
source-intent effective-conductivity model in the direct production runtime.
The port is a direct-runtime re-port, not a new derivation and not a magnitude
tuning package.

What changed:

- Added `Wb11HydrologyKernel::compute_direct_ksatadj_effective_conductivity`
  with typed inputs/outcome and 9001 / 9002+ / 9003 branch coverage.
- Wired active `ksatadj` soil policy into WB14 infiltration producer inputs so
  `Keff_ksatadj` supplies the pre-frost soil conductivity before the old base
  fallback path.
- Preserved frost-on behavior. `ksflag` remains non-consuming for frost
  activation, and the final WB14 effective conductivity is
  `min(Keff_ksatadj, frost_infcap_m_s)` when a positive frost cap is present.
- Added the p313 disturbed-burn forest fixture run as a real direct-runtime
  consumer test, including a manifest audit-counter assertion that the
  production `ksatadj` evaluator executes only when the soil-side flag is
  active.
- Lifted the `BR-SUBHYD-KSATADJ-GUARD` governance `HOLD` in
  `SC-SUBHYD-001` to runtime typed hard-fail and recorded v34 contract history.
- Split the oversized direct-publication builder during closure so no touched
  Rust file remains in the 2000+ warning band.

## Consumer Path

| Stage | Evidence |
|---|---|
| Parsed policy | Existing `DisturbedPolicy` `ksatadj`, `ksatfac`, `ksatrec`, `lkeff` parsing remains the input authority. |
| Static projection | `TypedSoilWb11RuntimeProjection` carries `ksatadj`, `solwpv`, policy operands, and layer `cpm`. |
| Dynamic operands | `DirectSubsurfaceLayerState` supplies daily top-two water storage, field capacity, upper limit, depth, porosity, `thetfc`, and `thetdr`. |
| Evaluator | `compute_direct_ksatadj_effective_conductivity` forms source-intent `avsat/(avpor*avcpm)`, applies source caps, and evaluates 9001 / 9002+ / 9003. |
| WB14 handoff | `DirectProductionInfiltrationAuthority::inputs` uses active `ksatadj` output as the pre-frost conductivity, then applies a positive frost cap as the final `DirectWb14InfiltrationProducerInputs.effective_conductivity_m_s` limiter. |
| Real consumer | `direct_runtime/runoff.rs` consumes the WB14 producer input through the production Green-Ampt path. |
| End-to-end marker | The hillslope manifest records `direct_runtime_counters.ksatadj_effective_conductivity_evaluations`; the p313 integration asserts it is positive for `ksatadj = 1` and zero for the same fixture with only that flag disabled. |

## Source-Intent Notes

- The conformance vector explicitly proves `sat_frac` is the source-intent
  rock-corrected denominator, not a storage-over-upper-limit surrogate.
- A first p313 run exposed an over-strict guard that rejected valid saturated
  storage (`theta > ul`). The implementation now follows the reference-intent
  caps on `avsat` instead of rejecting those states, with a regression test.
- The frost/`ksatadj` composition is explicit: `INV-SUBHYD-032` owns the
  source-intent `Keff_ksatadj` evaluator, and the WB14 authority composes that
  pre-frost surface with active frost by taking the lower positive conductivity
  cap.
- Missing 9001 recovery operands, missing top-two layers, non-finite values, and
  invalid branch operands fail closed through typed runtime errors.

## Scope Boundaries

- No provisional or surrogate physics was added.
- No WS-3 disturbed-burn magnitude adjudication was attempted.
- No runtime parser/schema changes beyond consuming already-projected soil
  authority were added.
- No `ksflag = 0` frost-off workaround was reintroduced.
