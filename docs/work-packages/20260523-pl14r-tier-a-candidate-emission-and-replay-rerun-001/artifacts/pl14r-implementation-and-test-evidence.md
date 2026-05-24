# PL14R Implementation and Test Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Implementation Summary

Static:
- Canonical PL14R replay authority amendments implemented:
  - `SC-SYSTEM-001` (`v8`)
  - `SC-WATBAL-001` (`v16`)
  - `science-contracts/index.md` notes updated
- PL14R contract-derived integration test target implemented:
  - `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
  - `Cargo.toml` target registration
- Replay/harness production source edits: none required.

Ran:
- Pre-implementation PL14R contract gate passed (`6/6`).
- Strict Tier-A comparator rerun commands executed and JSON artifacts persisted.
- Required repository gates executed and passing.

## Executed Commands

```bash
cargo test --test pl14r_tier_a_replay_rerun_contract -- --nocapture
cargo fmt
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
rm -rf /tmp/pl14r_tiera_cmp_20260523
mkdir -p /tmp/pl14r_tiera_cmp_20260523/baseline /tmp/pl14r_tiera_cmp_20260523/baseline/output /tmp/pl14r_tiera_cmp_20260523/candidate/output
cp -a /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs /tmp/pl14r_tiera_cmp_20260523/baseline/
cp docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/artifacts/wb13-h5-wat-candidate-sample.dat /tmp/pl14r_tiera_cmp_20260523/candidate/output/H5.wat.dat
(cd /tmp/pl14r_tiera_cmp_20260523/baseline/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill < p5.run > /tmp/pl14r_tiera_cmp_20260523/baseline_stdout.txt 2> /tmp/pl14r_tiera_cmp_20260523/baseline_stderr.txt)
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14r_tiera_cmp_20260523/baseline --candidate /tmp/pl14r_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.wat.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14r_tiera_cmp_20260523/h5_wat_comparator.json
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14r_tiera_cmp_20260523/baseline --candidate /tmp/pl14r_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.plot.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14r_tiera_cmp_20260523/h5_plot_comparator.json
```
