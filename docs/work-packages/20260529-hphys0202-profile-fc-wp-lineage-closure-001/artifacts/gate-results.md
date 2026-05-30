# HPHYS0202 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
3. `cargo test --workspace` -> pass
4. `cargo deny check` -> pass (`duplicate` and `license-not-encountered`
   warnings only; command exit code `0`)

## Diagnostic rerun gate (MEASURE-HP202-004)
1. Hillslope execution batch (`openwepp-cli-hill`, 39 hillslopes) -> pass
   - Status:
     `/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_batch_status.tsv`
   - Result: `39/39` hillslopes `rc=0`.
2. Semantic comparator batch (`H1..H39`) -> executed
   - Status:
     `/tmp/hphys0202_20260530T003833Z/parity/reports/semantic_status.tsv`
   - Result: comparator executed for all `39/39` hillslopes (`rc=0`).
3. Diagnostic summary produced:
   `/tmp/hphys0202_20260530T003833Z/parity/reports/hillslope_semantic_summary.json`
