# SIMIMPL34 Routine Lineage Migration Map

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Active-frost coupling migrated from reductive binary closure to
  baseline-lineage process shape in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`.
- Routine-chain lineage represented in runtime behavior:
  1. `frostN`-style daily/hourly frost bookkeeping with non-zero
     `frost.hourly.{qsrf_w_m2,quf_w_m2,ksrf_w_m_k}_####` emission.
  2. `frzng`/`frznw`-style freeze-lineage sensitivity via temperature-driven
     frost-depth progression (`frost.runtime_dfrost`) and frozen-water ledger
     progression (`frost.runtime_ws_frz`).
  3. `frwatc(1/0)`-style handoff effect via active-frost reduction of
     `wb11_soil_water` on positive frozen-water exchange increments.
  4. `frsoil` + `getFreezeCond` lineage via explicit
     `landuse.class_proxy -> kfactor{1,2,3}` selection for
     `frost.runtime_infcap_frz` coupling.
- SIMIMPL33 seam topology/publication surfaces are preserved and populated.

## Ran
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
