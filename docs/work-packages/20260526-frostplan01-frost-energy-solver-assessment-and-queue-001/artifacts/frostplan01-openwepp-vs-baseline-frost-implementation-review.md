# FROSTPLAN01 openWEPP vs Baseline Frost Implementation Review

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Current openWEPP active frost coupling remains reductive and centered on
  `compute_active_frost_coupling` in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`.
- Implementation shape in that path is branch-reductive:
  - freeze/thaw is keyed to `tmin <= 0` with fixed cap depth
    `WB14_FROST_MAX_DEPTH_M`.
  - `infcap_frz` is derived from a bounded linear reduction formula using
    `freeze_fraction` and `kfactor_floor`.
- Baseline-authoritative frost/winter routine chain required for parity is
  broader and routine-coupled:
  - `/workdir/wepp-forest_260430_baseline/src/winter.for`
  - `/workdir/wepp-forest_260430_baseline/src/frostn.for`
  - `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
  - `/workdir/wepp-forest_260430_baseline/src/frwatc.for`
  - `/workdir/wepp-forest_260430_baseline/src/frzng.for`
  - `/workdir/wepp-forest_260430_baseline/src/frznw.for`
  - `/workdir/wepp-forest_260430_baseline/src/winthd.for`
  - `/workdir/wepp-forest_260430_baseline/src/getfreezecond.for`
- Canonical contract/disposition posture still reflects incomplete frost
  closure:
  - `SC-SNOWFREEZE-001` retains `GAP-SNOWFREEZE-002` and explicit
    `frost.hourly.*` follow-on ownership.
  - SIMIMPL30 disposition and hold-lift report retain `Decision: HOLD` with
    frost-process follow-on required.

## Parity-Relevant Gap Summary
1. Process-shape gap: baseline frost-energy solver chain is not represented by
   the current binary freeze/thaw reduction branch.
2. State-family gap: full `frost.hourly.*` migration closure remains open.
3. Validation gap: frost-focused admissible hourly parity evidence remains a
   hold-lift prerequisite.

## Ran
- `rg -n "compute_active_frost_coupling|WB14_FROST_MAX_DEPTH_M|infcap_frz" crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `rg -n "subroutine +(winter|frostN|frsoil|frwatc|frzng|frznw|winthd)|real function getFreezeCond" /workdir/wepp-forest_260430_baseline/src/{winter.for,frostn.for,frsoil.for,frwatc.for,frzng.for,frznw.for,winthd.for,getfreezecond.for}`
- `rg -n "GAP-SNOWFREEZE-002|frost\.hourly\.\*|SIMIMPL29 does not claim full baseline frost energy-balance migration closure" docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `rg -n "Decision: HOLD|frost\.hourly|non-zero common-key overlap" docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/{simimpl30_disposition.md,simimpl30-hold-lift-decision-report.md}`
