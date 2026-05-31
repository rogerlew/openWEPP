# HPHYS0213 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass
   - Logs:
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_fmt_check.stdout.log`
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_fmt_check.stderr.log`
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
   - Logs:
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_clippy_workspace.stdout.log`
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_clippy_workspace.stderr.log`
3. `cargo test --workspace` -> pass
   - Logs:
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_test_workspace.stdout.log`
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_test_workspace.stderr.log`
4. `cargo deny check` -> pass
   - Exit code: `0`, warnings only (`duplicate`, `license-not-encountered`)
   - Logs:
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_deny_check.stdout.log`
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_deny_check.stderr.log`

## Contract-derived targeted tests
1. `cargo test -p openwepp-runner hphys0213_ -- --nocapture` -> pass
   - Logs:
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_test_targeted_hphys0213_runner.stdout.log`
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_test_targeted_hphys0213_runner.stderr.log`
2. `cargo test --test wb19_lateral_drainage_physics_kernel_contract -- --nocapture` -> pass
   - Logs:
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_test_targeted_hphys0213_wb19.stdout.log`
     - `/tmp/hphys0213_20260530T233248Z/gates/cargo_test_targeted_hphys0213_wb19.stderr.log`

## Diagnostic rerun gate
Run root: `/tmp/hphys0213_20260530T233248Z/parity/`

1. Hillslope execution batch (`openwepp-cli-hill`, 39 hillslopes) -> pass
   - Status:
     `/tmp/hphys0213_20260530T233248Z/parity/reports/hillslope_batch_status.tsv`
   - Result: `39/39` hillslopes `rc=0`.
2. Semantic comparator batch (`H1..H39`) -> pass
   - Runtime:
     `.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py ...`
   - Status:
     `/tmp/hphys0213_20260530T233248Z/parity/reports/semantic_status.tsv`
   - Result: `39/39` semantic jobs `rc=0`.
3. Summary produced:
   - `/tmp/hphys0213_20260530T233248Z/parity/reports/hillslope_semantic_summary.json`
   - `/tmp/hphys0213_20260530T233248Z/parity/reports/hillslope_semantic_summary.tsv`
4. H5 runtime blocker closure check:
   - `HKERNEL-WB12-STORAGE-E-003` absent from
     `/tmp/hphys0213_20260530T233248Z/parity/logs/h5.stderr.log`.
