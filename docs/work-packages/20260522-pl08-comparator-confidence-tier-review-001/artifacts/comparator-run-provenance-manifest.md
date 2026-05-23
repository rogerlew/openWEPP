# PL08 Comparator Run Provenance Manifest

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline authority: ADR-0012 pinned worktree `/workdir/wepp-forest_260430_baseline`.
- Baseline commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` (`Fix wshpas leap-day truncation and re-release wepp_260430`).

Ran:
- Comparator execution performed on 2026-05-22 (workspace local date).

## Inputs

- Fixture root: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs`
- Run file: `p5.run`
- Sidecars used by run file:
  - `p5.man`, `p5.slp`, `p5.cli`, `p5.sol`
  - `chan.inp`, `chntyp.txt`, `gwcoeff.txt`, `pmetpara.txt`, `snow.txt`, `wepp_ui.txt`

## Comparator Lane Roots

- Run root: `/tmp/pl08_tiera_cmp_20260522`
- Baseline lane: `/tmp/pl08_tiera_cmp_20260522/baseline`
- Candidate lane: `/tmp/pl08_tiera_cmp_20260522/candidate`
- Output subdir: `output`
- Include globs: `H5.wat.dat`, `H5.plot.dat`
- Tolerances: `abs_tol=0`, `rel_tol=0`

## Executed Commands

1. Baseline replay
- `(cd /tmp/pl08_tiera_cmp_20260522/baseline/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill < p5.run > /tmp/pl08_tiera_cmp_20260522/baseline_stdout.txt 2> /tmp/pl08_tiera_cmp_20260522/baseline_stderr.txt)`

2. Candidate replay
- `(cd /tmp/pl08_tiera_cmp_20260522/candidate/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260319_hill < p5.run > /tmp/pl08_tiera_cmp_20260522/candidate_stdout.txt 2> /tmp/pl08_tiera_cmp_20260522/candidate_stderr.txt)`

3. Daily water-balance comparator
- `python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl08_tiera_cmp_20260522/baseline --candidate /tmp/pl08_tiera_cmp_20260522/candidate --output-subdir output --include-globs H5.wat.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl08_tiera_cmp_20260522/h5_wat_comparator.json`

4. Plot comparator
- `python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl08_tiera_cmp_20260522/baseline --candidate /tmp/pl08_tiera_cmp_20260522/candidate --output-subdir output --include-globs H5.plot.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl08_tiera_cmp_20260522/h5_plot_comparator.json`

## Binary Provenance

- `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
  - sha256: `3b2fdd2b7a9e264b84f1e7b161dfb0730d49d3cb652218139efeb3ba17d7a160`
- `/workdir/wepp-forest_260430_baseline/release/wepp_260319_hill`
  - sha256: `a4c504c52c89ddd2111df544d403cd9df908661513a54519990b98b25f73d09e`

## Output Checksums

- `/tmp/pl08_tiera_cmp_20260522/baseline/output/H5.wat.dat`
  - sha256: `c383b31d42b311f9af9124db2fee1b1905a831b2e533ff63d9d667eafaf7ff83`
- `/tmp/pl08_tiera_cmp_20260522/candidate/output/H5.wat.dat`
  - sha256: `2c654b1ee84d2318df6a6da4526cdc217f266c8bed4da7c03617e001b02afe51`
- `/tmp/pl08_tiera_cmp_20260522/baseline/output/H5.plot.dat`
  - sha256: `6f0f32ee8e15302d6da2ab8e68bf07c3f2aa87d0375fba397451549a8db1e3ea`
- `/tmp/pl08_tiera_cmp_20260522/candidate/output/H5.plot.dat`
  - sha256: `6f0f32ee8e15302d6da2ab8e68bf07c3f2aa87d0375fba397451549a8db1e3ea`

## Execution Health

- Baseline stdout success marker present:
  - `WEPP COMPLETED HILLSLOPE SIMULATION SUCCESSFULLY`
- Candidate stdout success marker present:
  - `WEPP COMPLETED HILLSLOPE SIMULATION SUCCESSFULLY`
- stderr line counts:
  - `baseline_stderr.txt`: `0`
  - `candidate_stderr.txt`: `0`

## Persisted Evidence

- `artifacts/h5_wat_comparator.json`
- `artifacts/h5_plot_comparator.json`
