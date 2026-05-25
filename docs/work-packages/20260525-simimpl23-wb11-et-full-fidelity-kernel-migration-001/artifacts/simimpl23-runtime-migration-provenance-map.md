# SIMIMPL23 Runtime Migration Provenance Map

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
| baseline authority | migrated runtime behavior | openWEPP implementation surface |
|---|---|---|
| `/workdir/wepp-forest_260430_baseline/src/watbal.for:486-497,921,958-967` | Preserve WB11 hydrology sequence authority so percolation precedes ET and ET precedes lateral flow; preserve downstream `watcon` lineage expectations. | `crates/openwepp-hillslope-orchestrator/src/lib.rs` (`HillslopePhase::ORDERED`, `HillslopePhase::rank`, `HillslopePhaseGraph::canonical_dependencies`) |
| `/workdir/wepp-forest_260430_baseline/src/evap.for:458-555` | Preserve stage-memory transition family (`s1`, `s2`, `tu`, `tv`) with stage-1/stage-2 branch behavior and explicit `.0035` transition denominator. | `crates/openwepp-hillslope-orchestrator/src/lib.rs` (`run_evapotranspiration`) |
| `/workdir/wepp-forest_260430_baseline/src/swu.for:122-191` | Preserve transpiration uptake lineage semantics (`UPi`, `Ui`) and stress ratio lineage (`Ws = Ui/Etp` with explicit zero-demand branch). | `crates/openwepp-hillslope-orchestrator/src/lib.rs` (`run_evapotranspiration`) |
| `/workdir/wepp-forest_260430_baseline/src/outfil.for:623-644` and `/workdir/wepp-forest_260430_baseline/src/watbal.for:958-967` | Preserve WB13 publication-lineage identifiers (`Ep`, `Es`, `Er`, `Total-Soil`, `SoilWaterTotal`) rooted in WB11 aggregate soil-water lineage (`watcon`). | `crates/openwepp-hillslope-orchestrator/src/lib.rs` (`run_peak_runoff` alias publication updates) |

## Ran
- `sed -n '440,570p' /workdir/wepp-forest_260430_baseline/src/evap.for`
- `sed -n '120,200p' /workdir/wepp-forest_260430_baseline/src/swu.for`
- `rg -n "watcon|call purk|call evap|call evappm|call swu|soilw\\(i\\)|watcon\\(iplane\\)" /workdir/wepp-forest_260430_baseline/src/watbal.for`
- `rg -n "Ep|Es|Er|Total-Soil|SoilWaterTotal|watcon" /workdir/wepp-forest_260430_baseline/src/outfil.for`
- `rg -n "INV-EVAP-013|UPi|Ui|Ws|watcon|Total-Soil|SoilWaterTotal|purk" docs/specifications/science-contracts/contracts/SC-EVAP-001.md docs/specifications/science-contracts/contracts/SC-WATBAL-001.md docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
