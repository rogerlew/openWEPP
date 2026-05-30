# HPHYS0211 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Implementation scope
- Static: no production kernel/runtime code edits were performed in HPHYS0211.
- Static: package scope is residual root-cause decomposition from existing
  contracts, code, and semantic evidence.

## Workspace validation gates
- Ran: `cargo fmt --check` -> pass.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- Ran: `cargo test --workspace` -> pass.
- Ran: `cargo deny check` -> pass (warnings only, exit `0`).
- Ran: logs under `/tmp/hphys0211_20260530T203603Z/gates/`.

## Targeted contract-derived checks
- Ran:
  `cargo test -p openwepp --test hphys0208_fc_threshold_coupled_residual_contract`
  -> pass.
- Ran:
  `cargo test -p openwepp --test hphys0209_profilewp_adjudication_contract`
  -> pass.
- Ran: logs under `/tmp/hphys0211_20260530T203603Z/gates/`.

## Residual-root-cause diagnostics outputs
- Ran: generated per-hillslope column-stat extracts from HPHYS0208 semantic
  reports:
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_hillslope_column_stats.tsv`
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_failcount_summary.txt`
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_profilefc_fail_pattern_summary.txt`
  - `/tmp/hphys0211_20260530T203603Z/analysis/hphys0211_dp_toprow_candidate_clusters.tsv`
- Ran: extracted family summary used by HPHYS0211 decomposition:
  - `ProfileFCStore`: `27/39` fail hillslopes.
  - `Dp`: `39/39` fail hillslopes.
  - `latqcc`: `39/39` fail hillslopes.
  - `Total-Soil`: `39/39` fail hillslopes.
  - `SoilWaterTotal`: `39/39` fail hillslopes.
