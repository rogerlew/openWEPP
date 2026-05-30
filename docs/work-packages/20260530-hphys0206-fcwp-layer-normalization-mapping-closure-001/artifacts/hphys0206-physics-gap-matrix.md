# HPHYS0206 Physics Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP206-GAP-001` | Authoritative `thetfc_####`/`thetdr_####` publication needed deterministic mapping from normalized corrected layers into parser intervals. | Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs` now computes overlap-weighted mapping from normalized corrected layers and fails when mapping coverage is incomplete. | closed |
| `HP206-GAP-002` | Runtime surface allowed no explicit fail-closed contract surface for missing normalized-correction lineage inputs. | Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs` introduces `HS-RUNTIME-E-060..062`; tests enforce typed hard-fail behavior. | closed |
| `HP206-GAP-003` | FC/WP semantic residual remained open after HPHYS0205 and required rerun delta evidence against predecessor baselines. | Ran: `/tmp/hphys0206_20260530T032538Z/parity/reports/hillslope_semantic_summary.json` and semantic reports under `/tmp/hphys0206_20260530T032538Z/parity/reports/semantic/`. | closed (evidence produced) |

## Residual risk after closure
- Ran: FC/WP fail-hillslope counts remain saturated:
  - `ProfileFCStore`: `39/39`
  - `ProfileWPStore`: `39/39`
- Ran: residual magnitudes worsened vs HPHYS0205:
  - `ProfileFCStore` mean-abs-diff avg `6.4922 -> 7.2212` (`+0.7290`)
  - `ProfileWPStore` mean-abs-diff avg `1.8894 -> 2.2445` (`+0.3552`)
- Disposition remains `HOLD` pending follow-on physics-authority investigation.
