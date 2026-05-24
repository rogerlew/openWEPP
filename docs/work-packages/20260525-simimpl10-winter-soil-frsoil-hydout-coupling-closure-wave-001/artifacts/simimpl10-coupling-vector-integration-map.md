# simimpl10-coupling-vector-integration-map

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
| Vector family | Required symbols | Runner integration surface | Manifest surface | Guard |
|---|---|---|---|---|
| winter | `snow.options.{snow_file_present,rst,newsnw,ssd}`, `snow.runtime_swe` | `build_simimpl10_coupling_vector_provenance` in `openwepp-runner` | `/coupling_vectors/winter/*` | `HS-SIMCOUP-E-001` |
| soil | `ssc`, `frost.runtime_infcap_frz` | `build_simimpl10_coupling_vector_provenance` | `/coupling_vectors/soil/*` | `HS-SIMCOUP-E-001` |
| frsoil | `frost.options.{frost_file_present,wintRed}`, `frost.runtime_{dfrost,dthaw,nft,ws_frz,infcap_frz}` | `build_simimpl10_coupling_vector_provenance` | `/coupling_vectors/frsoil/*` | `HS-SIMCOUP-E-001` |
| hydout-equivalent | WB13 surfaces `Total-Soil`, `frozwt`, `Snow-Water`, `SoilWaterTotal` | WB13 row closure check in `build_simimpl10_coupling_vector_provenance` | `/coupling_vectors/hydout_equivalent/*` | `HS-SIMCOUP-E-001` |

## Ran
- Integration verified by `simimpl04_wepp_ui_mode_closure_contract` assertions against emitted manifest.
