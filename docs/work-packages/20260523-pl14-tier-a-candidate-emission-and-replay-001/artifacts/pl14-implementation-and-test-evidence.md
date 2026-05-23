# PL14 Implementation and Test Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Implementation Summary

Static:
- Canonical PL14 replay authority amendments implemented:
  - `SC-SYSTEM-001` (`v4`)
  - `SC-WATBAL-001` (`v8`)
  - `science-contracts/index.md` notes updated
- PL14 contract-derived integration test target implemented:
  - `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
  - `Cargo.toml` target registration
- Replay/harness production source edits: none required.

Ran:
- Pre-implementation PL14 contract gate passed (`4/4`).
- Strict Tier-A comparator replay commands executed and JSON artifacts persisted.
- Required repository gates executed and passing.

## Executed Commands

```bash
cargo test --test pl14_tier_a_candidate_replay_contract -- --nocapture
cargo fmt
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
(cd /tmp/pl14_tiera_cmp_20260523/baseline/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill < p5.run > /tmp/pl14_tiera_cmp_20260523/baseline_stdout.txt 2> /tmp/pl14_tiera_cmp_20260523/baseline_stderr.txt)
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14_tiera_cmp_20260523/baseline --candidate /tmp/pl14_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.wat.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14_tiera_cmp_20260523/h5_wat_comparator.json
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14_tiera_cmp_20260523/baseline --candidate /tmp/pl14_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.plot.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14_tiera_cmp_20260523/h5_plot_comparator.json
```
