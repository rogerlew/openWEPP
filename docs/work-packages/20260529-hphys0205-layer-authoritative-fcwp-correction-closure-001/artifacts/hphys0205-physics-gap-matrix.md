# HPHYS0205 Physics Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Scope summary
HPHYS0205 targets FC/WP publication lineage only:
authoritative WB13 layer symbols remain `thetfc_####`/`thetdr_####`, but those
symbols must carry baseline-corrected moisture lineage when available.

## Gap matrix
1. `GAP-HP205-001`  
   Scope: authoritative layer symbols were still raw-parser lineage in
   corrected-lane contexts.  
   Action: moved corrected moisture lineage into authoritative runtime layer
   symbol projection and reconciled WB13 FC/WP seed surfaces to those
   authoritative aggregates.  
   Evidence:
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
   - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
   - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
   - `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
   - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`  
   Status: **closed (authority + implementation surface)**.

2. `GAP-HP205-002`  
   Scope: tests did not guard corrected-layer authority and reconciliation
   behavior at seam boundaries.  
   Action: added contract-derived tests for corrected-layer projection and
   FC/WP aggregation reconciliation; updated seam probes to reject raw-theta
   authority assumptions.  
   Evidence:
   - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
   - `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
   - `tests/integration/parser_runtime_seam_integration.rs`  
   Status: **closed (test surface)**.

3. `GAP-HP205-003`  
   Scope: FC/WP semantic residual remained open in 39-hillslope parity
   diagnostics despite corrected-layer projection closure.  
   Evidence:
   - HPHYS0205 rerun:
     `/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_semantic_summary.json`
   - HPHYS0202 predecessor:
     `/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_semantic_summary.json`  
   Result:
   - `ProfileFCStore`: `39/39` fail hillslopes (unchanged vs HPHYS0202)
   - `ProfileWPStore`: `39/39` fail hillslopes (unchanged vs HPHYS0202)
   - Residual magnitudes are materially reduced in sampled evidence but remain
     above tolerance (see `artifacts/claude-code-review-findings.md`).
   - Still regressed vs HPARITY02 predecessor baseline (`27/39`, `1/39`).  
   Status: **open (follow-on required)**.
