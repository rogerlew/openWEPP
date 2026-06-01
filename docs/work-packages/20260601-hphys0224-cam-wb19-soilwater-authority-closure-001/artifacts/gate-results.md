# HPHYS0224 Gate Results

Status: completed  
Evidence mode: Ran

## Commands Run

1. Pre-implementation contract gate:
   - `cargo test --test hphys0224_wb19_withdrawal_soilwater_cap_contract`
   - expected red capture: fail (`HKERNEL-WB11-LAT-OK-001` observed vs
     expected `HKERNEL-WB11-LAT-E-003` for `lateral_overdraw_rejected`)
2. Post-implementation targeted tests:
   - `cargo test --test hphys0224_wb19_withdrawal_soilwater_cap_contract` (pass)
   - `cargo test --test auth06_fixture_provenance_hash_enforcement_contract --test hphys0224_wb19_withdrawal_soilwater_cap_contract` (pass)
   - `cargo test --test wb19_lateral_drainage_physics_kernel_contract --test hphys0219_wb19_coca_threshold_contract --test hphys0221_wb19_water_yield_fcdep_coupling_contract` (pass)
3. Required workspace gates:
   - `cargo fmt --check` (pass)
   - `cargo clippy --workspace --all-targets -- -D warnings` (pass)
   - `cargo test --workspace` (pass)
   - `cargo deny check` (pass; non-blocking warnings only)
4. Rerun/readjudication execution:
   - `target/debug/openwepp-cli-hill` over
     `/tmp/hphys0224_20260601T054337Z/parity/runs/p{1..39}_openwepp.run`
     (pass)
   - `.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
     for `H1..H39` with `--candidate-year-offset 2012` (pass)
   - semantic summary aggregation (pass)

## Execution Artifacts

- `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_batch_status.tsv`
- `/tmp/hphys0224_20260601T054337Z/parity/reports/semantic_status.tsv`
- `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_semantic_summary.tsv`

## Gate Decision

- Gate execution quality: pass.
- Package disposition remains `HOLD` based on unchanged monitored residual
  families.
