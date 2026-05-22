# Comparator Run Provenance Manifest (SR07)

Status: `complete`
Evidence mode: `Ran + Static`

Static:
- Baseline authority: ADR-0012 pinned baseline worktree `/workdir/wepp-forest_260430_baseline` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Canonical baseline hill binary hash from ADR-0012 matches local binary hash.

Ran:
- Comparator run executed on 2026-05-22 (local workspace time).

## Inputs

- Fixture root: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs`
- Run file: `p5.run`
- Sidecars copied with fixture: `p5.man`, `p5.slp`, `p5.cli`, `p5.sol`, `chan.inp`, `chntyp.txt`, `gwcoeff.txt`, `pmetpara.txt`, `snow.txt`, `wepp_ui.txt`

## Comparator Lane Roots

- Baseline lane root: `/tmp/sr07_tiera_cmp_20260522/baseline`
- Candidate lane root: `/tmp/sr07_tiera_cmp_20260522/candidate`
- Output subdir compared: `output`
- Include globs: `H5.wat.dat`
- Tolerances: `abs_tol=0`, `rel_tol=0`

## Executed Commands

1. Replay baseline lane:
- `(cd /tmp/sr07_tiera_cmp_20260522/baseline/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill < p5.run > /tmp/sr07_tiera_cmp_20260522/baseline_stdout.txt 2> /tmp/sr07_tiera_cmp_20260522/baseline_stderr.txt)`

2. Replay candidate lane:
- `(cd /tmp/sr07_tiera_cmp_20260522/candidate/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260319_hill < p5.run > /tmp/sr07_tiera_cmp_20260522/candidate_stdout.txt 2> /tmp/sr07_tiera_cmp_20260522/candidate_stderr.txt)`

3. Compare daily water-balance output:
- `python3 /workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py --baseline /tmp/sr07_tiera_cmp_20260522/baseline --candidate /tmp/sr07_tiera_cmp_20260522/candidate --output-subdir output --include-globs H5.wat.dat --abs-tol 0 --rel-tol 0 --json-out /tmp/sr07_tiera_cmp_20260522/h5_wat_comparator.json`

## Binary Provenance

- `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
  - sha256: `3b2fdd2b7a9e264b84f1e7b161dfb0730d49d3cb652218139efeb3ba17d7a160`
- `/workdir/wepp-forest_260430_baseline/release/wepp_260319_hill`
  - sha256: `a4c504c52c89ddd2111df544d403cd9df908661513a54519990b98b25f73d09e`

## Generated Output Checksums

- `/tmp/sr07_tiera_cmp_20260522/baseline/output/H5.wat.dat`
  - sha256: `c383b31d42b311f9af9124db2fee1b1905a831b2e533ff63d9d667eafaf7ff83`
- `/tmp/sr07_tiera_cmp_20260522/candidate/output/H5.wat.dat`
  - sha256: `2c654b1ee84d2318df6a6da4526cdc217f266c8bed4da7c03617e001b02afe51`

## Execution Health

- Baseline stdout contains success marker: `WEPP COMPLETED HILLSLOPE SIMULATION SUCCESSFULLY`
- Candidate stdout contains success marker: `WEPP COMPLETED HILLSLOPE SIMULATION SUCCESSFULLY`
- `baseline_stderr.txt`: 0 lines
- `candidate_stderr.txt`: 0 lines

## Persisted Comparator Evidence

- Copied JSON report: `/home/workdir/openWEPP/docs/work-packages/20260522-sr07-comparator-confidence-tier-delta-review-001/artifacts/h5_wat_comparator.json`
