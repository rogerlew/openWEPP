# HPARITY02 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass  
   Logs: `/tmp/hparity02_20260529T204555Z/gates/cargo_fmt_check.stdout.log`,
   `/tmp/hparity02_20260529T204555Z/gates/cargo_fmt_check.stderr.log`
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass  
   Logs: `/tmp/hparity02_20260529T204555Z/gates/cargo_clippy_workspace.stdout.log`,
   `/tmp/hparity02_20260529T204555Z/gates/cargo_clippy_workspace.stderr.log`
3. `cargo test --workspace` -> pass  
   Logs: `/tmp/hparity02_20260529T204555Z/gates/cargo_test_workspace.stdout.log`,
   `/tmp/hparity02_20260529T204555Z/gates/cargo_test_workspace.stderr.log`
4. `cargo deny check` -> pass  
   Logs: `/tmp/hparity02_20260529T204555Z/gates/cargo_deny_check.stdout.log`,
   `/tmp/hparity02_20260529T204555Z/gates/cargo_deny_check.stderr.log`

## Parity execution gates
1. Hillslope execution batch (`openwepp-cli-hill`, 39 hillslopes) -> pass
   - Status file:
     `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_batch_status.tsv`
   - Result: `39/39` `rc=0`.
2. Semantic comparator batch (`H1..H39`) -> executed
   - Status file:
     `/tmp/hparity02_20260529T204555Z/parity/reports/semantic_status.tsv`
   - Result: comparator executed for all `39/39` hillslopes.
   - Closure verdict determined by summary counts in
     `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`.
