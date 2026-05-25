# SIMIMPL20 WB11 Soil-Water Baseline Authority Path Assessment

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Baseline authority anchor for this assessment is
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- `watbal.for` establishes the daily soil-water mutation spine:
  1. `purk` percolation call (`watbal.for:486`),
  2. ET partition call (`evap`/`evappm`) (`watbal.for:495-497`),
  3. drainage + lateral-flow mutations (`watbal.for:552-720`),
  4. root uptake via `swu` conditioned on transpiration/root depth
     (`watbal.for:921`),
  5. aggregate profile recomputation into `watcon` (`watbal.for:958-966`).
- Baseline ET runtime state is stage-aware and layer-aware (`evap.for`):
  `s1`, `s2`, `tu`, `tv`, and explicit `st(layer)` extraction transitions
  (`evap.for:458-555`, `evap.for:609-659`).
- Baseline transpiration extraction is distributed by root depth in `swu.for`
  (`swu.for:122-177`) with stress feedback (`watstr`) (`swu.for:185-191`).
- Baseline daily publication semantics define `Ep`, `Es`, `Er`, and unfrozen
  `Total Soil Water` in WB13 output headers (`outfil.for:623-643`).
- openWEPP WB11 execution currently applies:
  - canonical phase order `ET -> PERC -> LAT -> DRAIN -> STORAGE`
    (`crates/openwepp-hillslope-orchestrator/src/lib.rs:395-418`,
    `:10003-10028`),
  - simplified ET partition using
    `Esp = et_demand * exp(-0.4 * lai)` in `run_evapotranspiration`
    (`lib.rs:4355-4499`),
  - per-layer WB18 percolation routing and WB19 lateral/drain surfaces in
    dedicated phases (`lib.rs:4509+`, `:4788+`, `:4954+`).

## Baseline-to-openWEPP Mapping
| baseline authority surface | baseline routine | current openWEPP surface | assessment |
|---|---|---|---|
| `st(i)` layer water storage mutation | `purk.for`, `evap.for`, `swu.for`, `drain.for` | `wb18_perc_theta_####` plus scalar `wb11_soil_water` | partial: WB18 layer percolation exists, but ET/root extraction is not layer-authoritative |
| `watcon` unfrozen profile total | `watbal.for` recompute loop | `wb11_soil_water`, WB13 `Total-Soil`, `SoilWaterTotal` publication | partial: publication exists; aggregate lineage remains coupled to simplified ET mutation path |
| `ep/es/eres` daily ET components | `evap.for` + `swu.for` | `Ep`, `Es`, `Er` flux writeback | partial: symbols emitted, but source physics excludes baseline stage-memory and layer uptake logic |
| `watstr` stress feedback | `swu.for` | `Ws` | partial: ratio branch exists, but baseline `Ui` layer-distribution path is absent |
| seepage / drainage / lateral runoff terms | `purk.for`, `drain.for`, `watbal.for` | `Pe`, `D`, `q`, `Qdd`, `Qd` | largely aligned for current WB18/WB19 lanes, pending full ET-soil coupling alignment |

## Landmine Summary
- Order-of-operations mismatch: baseline executes `swu` after lateral/drain
  mutation, while openWEPP applies transpiration withdrawal inside WB17 ET
  before WB18/WB19 phases.
- Missing first-class ET state surfaces (`s1`, `s2`, `tu`, `tv`, `dx`, `ds`,
  `UPi`, `Ui`) blocks strict baseline state-trajectory closure.
- Scalar-only `wb11_soil_water` ET mutation cannot encode baseline layer-selective
  root compensation behavior.

## Conclusion
- SIMIMPL20 confirms that full baseline-authoritative closure for
  `wb11_soil_water` + `Ep`/`Es`/`Er` requires follow-on contract and
  implementation waves; planning-only disposition remains `HOLD` until that
  queue executes.

## Ran
- `rg -n "call +(purk|evap|evappm|swu|drain)|watcon" /workdir/wepp-forest_260430_baseline/src/watbal.for`
- `rg -n "s1|s2|tu|tv|st\(" /workdir/wepp-forest_260430_baseline/src/evap.for`
- `rg -n "subroutine swu|watstr|st\(" /workdir/wepp-forest_260430_baseline/src/swu.for`
- `rg -n "Ep|Es|Er|Total Soil Water" /workdir/wepp-forest_260430_baseline/src/outfil.for`
- `rg -n "Evapotranspiration|PercolationDeepSeepage|LateralTransfer|Drainage" crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `sed -n '4310,4715p' crates/openwepp-hillslope-orchestrator/src/lib.rs`
