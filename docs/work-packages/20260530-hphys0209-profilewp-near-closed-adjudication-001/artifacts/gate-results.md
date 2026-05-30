# HPHYS0209 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass
   - Logs:
     - `/tmp/hphys0209_20260530T171007Z/gates/cargo_fmt_check.stdout.log`
     - `/tmp/hphys0209_20260530T171007Z/gates/cargo_fmt_check.stderr.log`
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
   - Logs:
     - `/tmp/hphys0209_20260530T171007Z/gates/cargo_clippy_workspace.stdout.log`
     - `/tmp/hphys0209_20260530T171007Z/gates/cargo_clippy_workspace.stderr.log`
3. `cargo test --workspace` -> pass
   - Logs:
     - `/tmp/hphys0209_20260530T171007Z/gates/cargo_test_workspace.stdout.log`
     - `/tmp/hphys0209_20260530T171007Z/gates/cargo_test_workspace.stderr.log`
4. `cargo deny check` -> pass
   - Exit code: `0`
   - Warnings only: `duplicate`, `license-not-encountered`
   - Logs:
     - `/tmp/hphys0209_20260530T171007Z/gates/cargo_deny_check.stdout.log`
     - `/tmp/hphys0209_20260530T171007Z/gates/cargo_deny_check.stderr.log`

## Contract-derived targeted tests
1. `cargo test -p openwepp --test hphys0209_profilewp_adjudication_contract`
   -> pass
   - Logs:
     - `/tmp/hphys0209_20260530T171007Z/tests/hphys0209_integration.stdout.log`
     - `/tmp/hphys0209_20260530T171007Z/tests/hphys0209_integration.stderr.log`
2. `cargo test -p openwepp-runner hphys0209_` -> pass
   - Logs:
     - `/tmp/hphys0209_20260530T171007Z/tests/hphys0209_runner.stdout.log`
     - `/tmp/hphys0209_20260530T171007Z/tests/hphys0209_runner.stderr.log`

## Residual focus diagnostic gate
Generated focused lane summary from existing 39-hillslope semantic artifacts:

- Source semantic reports:
  `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic/H*.semantic.json`
- Derived summary outputs:
  - `/tmp/hphys0209_20260530T171007Z/parity/reports/hphys0209_profilewp_focus_summary.json`
  - `/tmp/hphys0209_20260530T171007Z/parity/reports/hphys0209_profilewp_focus_summary.tsv`
- Confirmed focus-lane counts:
  - `ProfileWPStore`: `1/39` fail hillslopes (`H7`)
  - `ProfileDepth`: `0/39`
  - `ProfilePorosityCap`: `0/39`
