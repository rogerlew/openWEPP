# HPHYS0210 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass
   - Logs:
     - `/tmp/hphys0210_20260530T194829Z/gates/cargo_fmt_check.stdout.log`
     - `/tmp/hphys0210_20260530T194829Z/gates/cargo_fmt_check.stderr.log`
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
   - Logs:
     - `/tmp/hphys0210_20260530T194829Z/gates/cargo_clippy_workspace.stdout.log`
     - `/tmp/hphys0210_20260530T194829Z/gates/cargo_clippy_workspace.stderr.log`
3. `cargo test --workspace` -> pass
   - Logs:
     - `/tmp/hphys0210_20260530T194829Z/gates/cargo_test_workspace.stdout.log`
     - `/tmp/hphys0210_20260530T194829Z/gates/cargo_test_workspace.stderr.log`
4. `cargo deny check` -> pass
   - Exit code: `0`
   - Warnings only: `duplicate`, `license-not-encountered`
   - Logs:
     - `/tmp/hphys0210_20260530T194829Z/gates/cargo_deny_check.stdout.log`
     - `/tmp/hphys0210_20260530T194829Z/gates/cargo_deny_check.stderr.log`

## Integrated diagnostics gate
No additional kernel rerun was required for HPHYS0210; integrated adjudication
consumed upstream completed evidence and recomputed family metrics from semantic
reports:

- Source reports:
  - `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic/H*.semantic.json`
  - `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/H*.semantic.json`
  - `/tmp/hphys0209_20260530T171007Z/parity/reports/hphys0209_profilewp_focus_summary.json`
- Derived integrated outputs:
  - `/tmp/hphys0210_20260530T194829Z/diagnostics/hphys0210_integrated_family_summary.json`
  - `/tmp/hphys0210_20260530T194829Z/diagnostics/hphys0210_integrated_family_summary.tsv`
