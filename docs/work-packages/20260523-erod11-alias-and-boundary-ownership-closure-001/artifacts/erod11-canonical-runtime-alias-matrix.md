# EROD11 Canonical Runtime Alias Matrix

Status: `completed`
Evidence mode: `Static + Ran`

## Ratified Wave-0 Matrix

| contract_id | canonical symbols | runtime alias surface | owner |
|---|---|---|---|
| `SC-RUNOFFPART-001` | `Q` | `HillslopeProductionFluxSymbol::Wb12RunoffQ -> Q` | WB12 runoff reconciliation kernel (`SC-RUNOFFPART-001`) |
| `SC-RUNOFFPART-001` | `peakro`, `watdur` | `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur}` | WB16 peak-runoff kernel (`SC-RUNOFFPART-001` + `SC-WATBAL-001`) |
| `SC-RUNOFFPART-001` | `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionStateSymbol::{Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | WB16 diagnostics producer (`SC-WATBAL-001`) |
| `SC-WATBAL-001` | `Q` | `HillslopeProductionFluxSymbol::Wb12RunoffQ -> Q` | WB12 storage/runoff closure producer (`SC-WATBAL-001`) |
| `SC-WATBAL-001` | `peakro`, `watdur` | `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur}` | WB16 hydrology diagnostics producer (`SC-WATBAL-001`) |
| `SC-ROUTE-001` | `hs{ID}_peakro`, `hs{ID}_watdur` | `WatershedProductionStateSymbol::{HillslopeContributorPeak,HillslopeContributorDuration}` | WS10 hillslope ingress (`SC-ROUTE-001`) |
| `SC-ROUTE-001` | `qpo`, `durrof`, `roff` | `WatershedProductionStateSymbol::ChannelNode`; `WatershedProductionFluxSymbol::ChannelNode` | WS10 channel node producer (`SC-ROUTE-001`) |
| `SC-HYDRAULICS-001` | `fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe` | canonical identity aliases (runtime projection owner deferred under erosion-physics `HOLD`) | deferred implementation ownership (`HOLD`) |
| `SC-SED-001` | `sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i` | canonical identity aliases (runtime projection owner deferred under erosion-physics `HOLD`) | deferred implementation ownership (`HOLD`) |

Static:
- Matrix is authority-derived from canonical `SC-*` contract amendments.

Ran:
- Matrix rows were cross-checked against typed symbol implementations in
  `openwepp-kernel-contract` and against the updated contract addenda.
