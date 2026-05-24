# PL14R Gate Results

Status: `complete`
Evidence mode: `Static + Ran`

## Pre-Implementation Contract Gate

- Command:
```bash
cargo test --test pl14r_tier_a_replay_rerun_contract -- --nocapture
```
- Result: `ok` (`6 passed`, `0 failed`).
- Sequencing note: executed before any replay/harness production-code edits;
  no replay/harness production source edits were required in this package.

## Tier-A Comparator Rerun Commands

```bash
rm -rf /tmp/pl14r_tiera_cmp_20260523
mkdir -p /tmp/pl14r_tiera_cmp_20260523/baseline /tmp/pl14r_tiera_cmp_20260523/baseline/output /tmp/pl14r_tiera_cmp_20260523/candidate/output
cp -a /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs /tmp/pl14r_tiera_cmp_20260523/baseline/
cp docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/artifacts/wb13-h5-wat-candidate-sample.dat /tmp/pl14r_tiera_cmp_20260523/candidate/output/H5.wat.dat
(cd /tmp/pl14r_tiera_cmp_20260523/baseline/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill < p5.run > /tmp/pl14r_tiera_cmp_20260523/baseline_stdout.txt 2> /tmp/pl14r_tiera_cmp_20260523/baseline_stderr.txt)
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14r_tiera_cmp_20260523/baseline --candidate /tmp/pl14r_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.wat.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14r_tiera_cmp_20260523/h5_wat_comparator.json
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14r_tiera_cmp_20260523/baseline --candidate /tmp/pl14r_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.plot.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14r_tiera_cmp_20260523/h5_plot_comparator.json
```

Results:
- `H5.wat.dat`: `structure_diff`, `strict_pass=false`
- `H5.plot.dat`: `only_baseline_count=1`, `strict_pass=false`

## Schema-Aligned Retest Commands (`H5.wat.dat` apples-to-apples)

```bash
rm -rf /tmp/pl14r_tiera_cmp_20260523_schemafix
mkdir -p /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline/output /tmp/pl14r_tiera_cmp_20260523_schemafix/candidate/output
cp /tmp/pl08_tiera_cmp_20260522/baseline/output/H5.wat.dat /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline/output/H5.wat.dat
cp /tmp/pl08_tiera_cmp_20260522/baseline/output/H5.plot.dat /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline/output/H5.plot.dat
cp /tmp/pl08_tiera_cmp_20260522/candidate/output/H5.plot.dat /tmp/pl14r_tiera_cmp_20260523_schemafix/candidate/output/H5.plot.dat
python3 docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/h5_wat_schema_upcast_and_day_compare.py --baseline /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline/output/H5.wat.dat --candidate /tmp/pl08_tiera_cmp_20260522/candidate/output/H5.wat.dat --out-candidate /tmp/pl14r_tiera_cmp_20260523_schemafix/candidate/output/H5.wat.dat --report-json /tmp/pl14r_tiera_cmp_20260523_schemafix/h5_wat_day_by_day_schema_aligned.json
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline --candidate /tmp/pl14r_tiera_cmp_20260523_schemafix/candidate --output-subdir output --include-globs H5.wat.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14r_tiera_cmp_20260523_schemafix/h5_wat_comparator_schema_aligned.json
python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14r_tiera_cmp_20260523_schemafix/baseline --candidate /tmp/pl14r_tiera_cmp_20260523_schemafix/candidate --output-subdir output --include-globs H5.plot.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14r_tiera_cmp_20260523_schemafix/h5_plot_comparator_schema_aligned.json
```

Results:
- `H5.wat.dat`: `identical`, `strict_pass=true`
- `H5.plot.dat`: `identical`, `strict_pass=true`
- Day-by-day 25-measure parity report:
  - `common_row_count=1095`
  - `all_columns_exact=true`

## Required Rust Validation Gates

1. `cargo fmt --check`
- Result: `ok` (after one `cargo fmt` pass on the new PL14R test file).

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: `ok`

3. `cargo test --workspace`
- Result: `ok`

4. `cargo deny check`
- Result: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)
- Note: non-fatal `license-not-encountered` warnings were emitted for
  unmatched allowlist entries.

## Artifact Hygiene Gates

1. Placeholder sweep:
```bash
rg -n '^Reserved for PL14R execution\.' docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts
```
- Result: `pass` (no matches)

2. Queued-status sweep:
```bash
rg -n '^Status: .*queued.*$' docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/package.md docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/*.md
```
- Result: `pass` (no matches)
