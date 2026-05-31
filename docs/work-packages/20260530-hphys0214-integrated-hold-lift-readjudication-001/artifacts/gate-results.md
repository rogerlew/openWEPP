# HPHYS0214 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass
   - Logs:
     - `/tmp/hphys0214_20260531T004200Z/gates/cargo_fmt_check.stdout.log`
     - `/tmp/hphys0214_20260531T004200Z/gates/cargo_fmt_check.stderr.log`
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
   - Logs:
     - `/tmp/hphys0214_20260531T004200Z/gates/cargo_clippy_workspace.stdout.log`
     - `/tmp/hphys0214_20260531T004200Z/gates/cargo_clippy_workspace.stderr.log`
3. `cargo test --workspace` -> pass
   - Logs:
     - `/tmp/hphys0214_20260531T004200Z/gates/cargo_test_workspace.stdout.log`
     - `/tmp/hphys0214_20260531T004200Z/gates/cargo_test_workspace.stderr.log`
4. `cargo deny check` -> pass
   - Exit code: `0`
   - Warnings only: `duplicate`, `license-not-encountered`
   - Logs:
     - `/tmp/hphys0214_20260531T004200Z/gates/cargo_deny_check.stdout.log`
     - `/tmp/hphys0214_20260531T004200Z/gates/cargo_deny_check.stderr.log`

## Targeted contract-derived checks
1. `cargo test -p openwepp --test hphys0208_fc_threshold_coupled_residual_contract` -> pass
   - Logs:
     - `/tmp/hphys0214_20260531T004200Z/tests/hphys0208_integration.stdout.log`
     - `/tmp/hphys0214_20260531T004200Z/tests/hphys0208_integration.stderr.log`
2. `cargo test -p openwepp --test hphys0209_profilewp_adjudication_contract` -> pass
   - Logs:
     - `/tmp/hphys0214_20260531T004200Z/tests/hphys0209_integration.stdout.log`
     - `/tmp/hphys0214_20260531T004200Z/tests/hphys0209_integration.stderr.log`
3. `cargo test -p openwepp-runner hphys0213_` -> pass
   - Logs:
     - `/tmp/hphys0214_20260531T004200Z/tests/hphys0213_runner.stdout.log`
     - `/tmp/hphys0214_20260531T004200Z/tests/hphys0213_runner.stderr.log`

## Integrated diagnostics gate
Integrated readjudication consumed canonical upstream summaries and produced:

- `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.json`
- `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.tsv`

Key integrated signals:
- H5 blocker continuity: `HKERNEL-WB12-STORAGE-E-003` is present in HPHYS0212
  logs and absent in HPHYS0213 logs.
- Remaining hold blockers in HPHYS0213 lane:
  `ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`.
