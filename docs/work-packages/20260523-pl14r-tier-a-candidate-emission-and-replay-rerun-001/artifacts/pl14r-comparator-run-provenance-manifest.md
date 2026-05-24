# PL14R Comparator Rerun Provenance Manifest

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline authority: ADR-0012 pinned worktree `/workdir/wepp-forest_260430_baseline`
- Baseline commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

Ran:
- Comparator execution performed on 2026-05-23 (workspace local date).

## Inputs

- Baseline fixture root:
  `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs`
- Run file: `p5.run`
- Baseline replay sidecars:
  - `p5.man`, `p5.slp`, `p5.cli`, `p5.sol`

## Candidate Emission Source

- Direct openWEPP WB13 candidate daily output surface:
  `docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/artifacts/wb13-h5-wat-candidate-sample.dat`
- Candidate source SHA256:
  `fa1ccbe2f0e9b1fc56c542663c11a6fe23c08827ef80d185547e2c6ab042dd2c`

## Comparator Lane Roots

- Run root: `/tmp/pl14r_tiera_cmp_20260523`
- Baseline lane: `/tmp/pl14r_tiera_cmp_20260523/baseline`
- Candidate lane: `/tmp/pl14r_tiera_cmp_20260523/candidate`
- Output subdir: `output`
- Tolerances: `abs_tol=0`, `rel_tol=0`

## Executed Commands

1. Replay-lane staging
- `rm -rf /tmp/pl14r_tiera_cmp_20260523`
- `mkdir -p /tmp/pl14r_tiera_cmp_20260523/baseline /tmp/pl14r_tiera_cmp_20260523/baseline/output /tmp/pl14r_tiera_cmp_20260523/candidate/output`
- `cp -a /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs /tmp/pl14r_tiera_cmp_20260523/baseline/`
- `cp docs/work-packages/20260523-wb13-daily-water-balance-output-surface-001/artifacts/wb13-h5-wat-candidate-sample.dat /tmp/pl14r_tiera_cmp_20260523/candidate/output/H5.wat.dat`

2. Baseline replay
- `(cd /tmp/pl14r_tiera_cmp_20260523/baseline/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill < p5.run > /tmp/pl14r_tiera_cmp_20260523/baseline_stdout.txt 2> /tmp/pl14r_tiera_cmp_20260523/baseline_stderr.txt)`

3. Daily water-balance comparator
- `python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14r_tiera_cmp_20260523/baseline --candidate /tmp/pl14r_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.wat.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14r_tiera_cmp_20260523/h5_wat_comparator.json`

4. Plot comparator
- `python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/pl14r_tiera_cmp_20260523/baseline --candidate /tmp/pl14r_tiera_cmp_20260523/candidate --output-subdir output --include-globs H5.plot.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/pl14r_tiera_cmp_20260523/h5_plot_comparator.json`

## Binary and Tool Provenance

- Baseline binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
  - SHA256: `3b2fdd2b7a9e264b84f1e7b161dfb0730d49d3cb652218139efeb3ba17d7a160`
- Comparator tool: `/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py`
  - SHA256: `c9c5f2eac59cdd4c6b8f7bc8423577e679effd68554b87ff62abf76371af91c8`

## Output Checksums

- `/tmp/pl14r_tiera_cmp_20260523/baseline/output/H5.wat.dat`
  - SHA256: `c383b31d42b311f9af9124db2fee1b1905a831b2e533ff63d9d667eafaf7ff83`
- `/tmp/pl14r_tiera_cmp_20260523/baseline/output/H5.plot.dat`
  - SHA256: `6f0f32ee8e15302d6da2ab8e68bf07c3f2aa87d0375fba397451549a8db1e3ea`
- `/tmp/pl14r_tiera_cmp_20260523/candidate/output/H5.wat.dat`
  - SHA256: `fa1ccbe2f0e9b1fc56c542663c11a6fe23c08827ef80d185547e2c6ab042dd2c`
- `/tmp/pl14r_tiera_cmp_20260523/h5_wat_comparator.json`
  - SHA256: `31086f2dffc4d8790436b24f4e0b40d4982fbcf0e397890abcba2410f09b0abc`
- `/tmp/pl14r_tiera_cmp_20260523/h5_plot_comparator.json`
  - SHA256: `2f8fe012e027403f63e35d746166de652cc6f4c36626f358d2ce3175bcbf9de7`

## Execution Health

- Baseline success marker present:
  - `WEPP COMPLETED HILLSLOPE SIMULATION SUCCESSFULLY`
- stderr line counts:
  - `baseline_stderr.txt`: `0`
- Output line counts:
  - `baseline/output/H5.wat.dat`: `1123`
  - `baseline/output/H5.plot.dat`: `104`
  - `candidate/output/H5.wat.dat`: `5`
