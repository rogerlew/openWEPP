# HPHYS0212 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass
   - Logs:
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_fmt_check.stdout.log`
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_fmt_check.stderr.log`
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
   - Logs:
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_clippy_workspace.stdout.log`
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_clippy_workspace.stderr.log`
3. `cargo test --workspace` -> pass
   - Logs:
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_test_workspace.stdout.log`
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_test_workspace.stderr.log`
4. `cargo deny check` -> pass
   - Exit code: `0`, warnings only (`duplicate`, `license-not-encountered`)
   - Logs:
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_deny_check.stdout.log`
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_deny_check.stderr.log`

## Targeted test sweep
1. `cargo test -p openwepp-hillslope-orchestrator -p openwepp-runner` -> pass
   - Logs:
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_test_targeted_hphys0212.stdout.log`
     - `/tmp/hphys0212_20260530T222619Z/gates/cargo_test_targeted_hphys0212.stderr.log`
