# HPHYS0211 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass
   - Logs:
     - `/tmp/hphys0211_20260530T203603Z/gates/cargo_fmt_check.stdout.log`
     - `/tmp/hphys0211_20260530T203603Z/gates/cargo_fmt_check.stderr.log`
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
   - Logs:
     - `/tmp/hphys0211_20260530T203603Z/gates/cargo_clippy_workspace.stdout.log`
     - `/tmp/hphys0211_20260530T203603Z/gates/cargo_clippy_workspace.stderr.log`
3. `cargo test --workspace` -> pass
   - Logs:
     - `/tmp/hphys0211_20260530T203603Z/gates/cargo_test_workspace.stdout.log`
     - `/tmp/hphys0211_20260530T203603Z/gates/cargo_test_workspace.stderr.log`
4. `cargo deny check` -> pass
   - Exit code: `0`, warnings only:
     - `duplicate` (`wasm-bindgen-shared`, `twox-hash`)
     - `license-not-encountered` (`ISC`, `Unicode-DFS-2016`)
   - Logs:
     - `/tmp/hphys0211_20260530T203603Z/gates/cargo_deny_check.stdout.log`
     - `/tmp/hphys0211_20260530T203603Z/gates/cargo_deny_check.stderr.log`

## Targeted contract-derived tests
1. `cargo test -p openwepp --test hphys0208_fc_threshold_coupled_residual_contract` -> pass
   - Logs:
     - `/tmp/hphys0211_20260530T203603Z/gates/hphys0208_contract_test.stdout.log`
     - `/tmp/hphys0211_20260530T203603Z/gates/hphys0208_contract_test.stderr.log`
2. `cargo test -p openwepp --test hphys0209_profilewp_adjudication_contract` -> pass
   - Logs:
     - `/tmp/hphys0211_20260530T203603Z/gates/hphys0209_contract_test.stdout.log`
     - `/tmp/hphys0211_20260530T203603Z/gates/hphys0209_contract_test.stderr.log`

## Root-cause analysis extracts
- Generated analysis bundle:
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_hillslope_column_stats.tsv`
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_dp_latqcc_toprow.tsv`
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_failcount_summary.txt`
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_profilefc_fail_pattern_summary.txt`
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_dp_toprow_candidate_clusters.tsv`
