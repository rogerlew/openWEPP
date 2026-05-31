# HPHYS0216 Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

## Implementation scope executed
- Implemented FC publication authority realignment in
  `crates/openwepp-runner/src/hillslope/mod.rs` by introducing
  `derive_profile_fc_store_from_authoritative_layers(...)` and publishing
  `ProfileFCStore` from `thetfc_####` + `dg_####`.
- Preserved WP publication authority from `wb13_profile_wp_store_mm`.
- Updated contract-derived integration coverage in
  `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`.
- Registered HPHYS0216 in `docs/work-packages/README.md`.

## Commands executed (Ran)
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
5. targeted reruns:
   - `cargo test -p openwepp-runner hphys0216_ -- --nocapture`
   - `cargo test -p openwepp --test hphys0202_profile_fc_wp_lineage_contract`

## 39-hillslope semantic rerun evidence (Ran)
- Rerun root: `/tmp/hphys0216_20260531T053959Z/`
- Batch status:
  `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_batch_status.tsv`
  (`39/39` rc=0)
- Semantic status:
  `/tmp/hphys0216_20260531T053959Z/parity/reports/semantic_status.tsv`
  (`39/39` rc=0)
- Summary:
  `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_semantic_summary.tsv`

## Closure result
- Contract/test/implementation scope completed.
- `MEASURE-HP216-004` failed (`ProfileFCStore` did not reduce vs HPHYS0214).
