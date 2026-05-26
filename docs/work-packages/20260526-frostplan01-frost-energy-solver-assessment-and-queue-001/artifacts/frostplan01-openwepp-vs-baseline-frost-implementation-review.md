# FROSTPLAN01 openWEPP vs Baseline Frost Implementation Review

Status: queued
Evidence mode: static
Date: 2026-05-26

## Static
- Current openWEPP active frost coupling path remains reductive and is centered
  in `compute_active_frost_coupling`:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs:1677`
  - freeze branch sets fixed cap depth (`WB14_FROST_MAX_DEPTH_M`) and thaw depth
    by binary `tmin <= 0` decision.
  - infiltration capacity reduction is linearized via
    `soil_conductivity * (1 - freeze_fraction + freeze_fraction * kfactor_floor)`.
- Baseline authority chain for frost process parity is broader and routine-based:
  - `/workdir/wepp-forest_260430_baseline/src/winter.for`
  - `/workdir/wepp-forest_260430_baseline/src/frostn.for`
  - `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
  - `/workdir/wepp-forest_260430_baseline/src/frwatc.for`
  - `/workdir/wepp-forest_260430_baseline/src/frzng.for`
  - `/workdir/wepp-forest_260430_baseline/src/frznw.for`
  - `/workdir/wepp-forest_260430_baseline/src/winthd.for`
  - `/workdir/wepp-forest_260430_baseline/src/getfreezecond.for`
- Existing audit and disposition posture confirms this mismatch:
  - audit row classifies frost chain as "replaced by reduction" and calls out
    missing fine-layer thermal solver behavior.
  - SIMIMPL30 hold-lift report and disposition retain `HOLD` with unresolved
    `frost.hourly.*` family/process closure ownership.

## Parity-Relevant Gap Summary
1. Process-shape gap: baseline 10-layer/fine-layer frost-energy bookkeeping is
   not represented by current binary threshold branch.
2. State-family gap: canonical `frost.hourly.*` outputs remain incomplete for
   full process-family closure.
3. Comparator-depth gap: executable hourly frost comparator vectors are not yet
   complete for hold-lift confidence.

## Ran
- `rg -n "compute_active_frost_coupling|FROST_MAX_DEPTH|infcap_frz" crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `rg -n "subroutine +(frostn|frsoil|frwatc|frzng|frznw|getfreezecond|winter|winthd)" /workdir/wepp-forest_260430_baseline/src/*.for`
- `rg -n "frostn\.for|frsoil\.for|frwatc\.for|frzng\.for|frznw\.for|getfreezecond\.for|compute_active_frost_coupling|frost\.hourly" docs/audits/20260525_water_erosion_kernel_audit.md docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `sed -n '1,220p' docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/simimpl30_disposition.md`
