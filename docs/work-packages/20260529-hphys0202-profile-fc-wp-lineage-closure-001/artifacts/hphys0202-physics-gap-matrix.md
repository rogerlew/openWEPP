# HPHYS0202 Physics Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Scope summary
HPHYS0202 targets only `ProfileFCStore` and `ProfileWPStore` WB13 publication
lineage closure and typed-guard behavior.

## Gap matrix
1. `GAP-HP202-001`  
   Scope: WB13 FC/WP publication authority still coupled to adapter seed
   surfaces (`wb13_profile_fc_store_mm`, `wb13_profile_wp_store_mm`).  
   Action: switched publication authority to layer aggregation
   (`thetfc/thetdr * dg`), retained seed values as diagnostics only.  
   Evidence:
   - `SC-WATBAL-001`, `SC-SOIL-001`, `SC-PERC-001`, `SC-SYSTEM-001`
   - `crates/openwepp-runner/src/hillslope/mod.rs`  
   Status: **closed (authority and implementation surface)**.

2. `GAP-HP202-002`  
   Scope: WB13 FC/WP guard/type-state behavior insufficiently exercised by
   direct tests.  
   Action: added direct WB13 probe tests in runner unit test module and
   behavior-level integration tests.  
   Evidence:
   - `hphys0202_wb13_fc_seed_guard_is_exercised_by_direct_row_builder_probe`
   - `hphys0202_wb13_wp_seed_guard_is_exercised_by_direct_row_builder_probe`
   - `hphys0202_wb13_profile_fc_wp_publication_ignores_seed_values_when_valid`
   - `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`  
   Status: **closed (test closure for publication/guard surface)**.

3. `GAP-HP202-003`  
   Scope: diagnostic semantic residual persists for FC/WP after closure edits.  
   Evidence:
   - `ProfileFCStore` and `ProfileWPStore` fail in `39/39` hillslopes in
     `/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_semantic_summary.json`.
   - predecessor HPARITY02 disposition reported:
     - `ProfileFCStore`: `27/39` fail
     - `ProfileWPStore`: `1/39` fail
   - Claude review identifies source-lineage mismatch between corrected seed
     FC/WP symbols and currently published raw layer aggregates:
     `artifacts/claude-code-review-findings.md` (`F-5`, `F-6`, `F-11`).  
   Status: **open (follow-on required)**.

## Out-of-scope residuals (diagnostic context only)
Always-fail columns outside HPHYS0202 scope remain:
`RM`, `Ep`, `Es`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `Snow-Water`,
`Q`, `QOFE`.
