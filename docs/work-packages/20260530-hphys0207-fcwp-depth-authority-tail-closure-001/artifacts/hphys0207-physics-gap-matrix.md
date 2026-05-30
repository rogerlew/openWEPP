# HPHYS0207 Physics Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP207-GAP-001` | WB13 FC/WP publication authority was sourced from parser-layer-depth aggregates while profile depth/capacity used normalized-profile authority, creating a depth-domain mismatch. | Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs` now projects FC/WP storage from normalized corrected-layer lineage via `Wb13ProfileSymbols.fc_store/wp_store`; `crates/openwepp-runner/src/hillslope/mod.rs` consumes `wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm` as WB13 publication authority. | closed |
| `HP207-GAP-002` | Normalized-tail storage policy was implicit and allowed silent truncation behavior at parser-layer publication depth. | Static: HPHYS0207 addenda in `SC-SOIL-001`, `SC-WATBAL-001`, `SC-PERC-001`, and `SC-SYSTEM-001` now encode explicit normalized-tail consumption policy and forbid silent truncation/fallback repair. | closed |
| `HP207-GAP-003` | HPHYS0206 regressed FC/WP residual direction vs HPHYS0205 and required no-regression closure evidence. | Ran: `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_semantic_summary.json` plus semantic reports under `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/` show FC/WP fail-count and residual-magnitude improvement vs HPHYS0205 and HPHYS0206. | closed |

## Residual risk after closure
- Ran: comparator residual is not zero:
  - `ProfileFCStore`: `27/39` fail hillslopes.
  - `ProfileWPStore`: `1/39` fail hillslopes.
- Ran: residual direction is strongly improved and non-regressing vs
  HPHYS0205/HPHYS0206.
- Disposition remains `HOLD` pending downstream package hold-lift policy,
  despite HPHYS0207 scope closure.
