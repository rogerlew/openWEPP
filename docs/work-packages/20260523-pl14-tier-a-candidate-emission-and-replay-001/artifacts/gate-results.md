# PL14 Gate Results

Status: `complete`
Evidence mode: `Static + Ran`

## Pre-Implementation Contract Gate

- Command:
```bash
cargo test --test pl14_tier_a_candidate_replay_contract -- --nocapture
```
- Result: `ok` (`4 passed`, `0 failed`).
- Sequencing note: executed before any replay/harness production-code edits;
  no replay/harness production source edits were required in this package.

## Tier-A Comparator Replay Commands

```bash
(cd /tmp/pl14_tiera_cmp_20260523/baseline/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill < p5.run > /tmp/pl14_tiera_cmp_20260523/baseline_stdout.txt 2> /tmp/pl14_tiera_cmp_20260523/baseline_stderr.txt)
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14_tiera_cmp_20260523/baseline --candidate /tmp/pl14_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.wat.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14_tiera_cmp_20260523/h5_wat_comparator.json
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14_tiera_cmp_20260523/baseline --candidate /tmp/pl14_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.plot.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14_tiera_cmp_20260523/h5_plot_comparator.json
```

Results:
- `H5.wat.dat`: `structure_diff`, `strict_pass=false`
- `H5.plot.dat`: `only_baseline_count=1`, `strict_pass=false`

## Required Rust Validation Gates

1. `cargo fmt --check`
- Result: `ok` (after formatting test file with `cargo fmt`).

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: `ok`

3. `cargo test --workspace`
- Result: `ok`

4. `cargo deny check`
- Result: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)
- Note: non-fatal `license-not-encountered` warnings were emitted for unmatched
  allowlist entries.
