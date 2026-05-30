# HPHYS0208 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass
   - Logs:
     - `/tmp/hphys0208_20260530T155837Z/gates/cargo_fmt_check.stdout.log`
     - `/tmp/hphys0208_20260530T155837Z/gates/cargo_fmt_check.stderr.log`
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
   - Logs:
     - `/tmp/hphys0208_20260530T155837Z/gates/cargo_clippy_workspace.stdout.log`
     - `/tmp/hphys0208_20260530T155837Z/gates/cargo_clippy_workspace.stderr.log`
3. `cargo test --workspace` -> pass
   - Logs:
     - `/tmp/hphys0208_20260530T155837Z/gates/cargo_test_workspace.stdout.log`
     - `/tmp/hphys0208_20260530T155837Z/gates/cargo_test_workspace.stderr.log`
4. `cargo deny check` -> pass
   - Exit code: `0`
   - Warnings only: `duplicate`, `license-not-encountered`
   - Logs:
     - `/tmp/hphys0208_20260530T155837Z/gates/cargo_deny_check.stdout.log`
     - `/tmp/hphys0208_20260530T155837Z/gates/cargo_deny_check.stderr.log`

## Contract-derived targeted tests
1. `cargo test -p openwepp --test hphys0208_fc_threshold_coupled_residual_contract` -> pass
2. `cargo test -p openwepp-runner hphys0208_` -> pass

## Diagnostic rerun gate
Run root: `/tmp/hphys0208_20260530T155837Z/parity/`

1. Hillslope execution batch (`openwepp-cli-hill`, 39 hillslopes) -> pass
   - Status:
     `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_batch_status.tsv`
   - Result: `39/39` hillslopes `rc=0`.
2. Semantic comparator batch (`H1..H39`) -> pass
   - Status:
     `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic_status.tsv`
   - Result: `39/39` semantic jobs `rc=0`.
3. Summary produced:
   - `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.json`
   - `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.tsv`
