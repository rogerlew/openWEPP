# Single-OFE Daily Water-Balance Comparator Delta Report (SR07)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Tier-A scope for this package is single-OFE daily water-balance surfaces (ADR-0011, numerics policy).
- Pinned legacy baseline authority is ADR-0012 (`/workdir/wepp-forest_260430_baseline`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`).

Ran:
- Executed hillslope replay for a single-OFE fixture (`delicate_game_pw0` hillslope `p5`) with two legacy binaries, then compared `H5.wat.dat` using `compare_wepp_raw_outputs.py`.

## Comparator Target

- Surface: `H5.wat.dat` (daily water-balance output)
- Fixture run path: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs/p5.run`
- Single-OFE evidence: `/workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0/runs/p5.slp` line 2 = `1`

## Run Lane

- Baseline binary: `/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill`
  - sha256: `3b2fdd2b7a9e264b84f1e7b161dfb0730d49d3cb652218139efeb3ba17d7a160`
- Candidate binary (surrogate legacy lane for comparator execution): `/workdir/wepp-forest_260430_baseline/release/wepp_260319_hill`
  - sha256: `a4c504c52c89ddd2111df544d403cd9df908661513a54519990b98b25f73d09e`

## Delta Summary (`abs_tol=0`, `rel_tol=0`)

- Result: `structure_diff`
- Strict pass: `false`
- Common files: `1`
- Changed files: `1`
- `H5.wat.dat`:
  - baseline lines: `1123`
  - candidate lines: `1118`
  - line count mismatch lines: `5`
  - numeric arity mismatch lines: `1096`
  - numeric values compared: `21802`
  - numeric values exceeding tolerance: `5568`
  - max absolute diff: `360.0`
  - max relative diff: `1.0`

## First Divergence Anchor

- First data-row divergence:
  - row index: `1`
  - key: `OFE=1, J=1, Y=1`
  - baseline row has 25 numeric fields; candidate row has 20 numeric fields.
- Interpretation: daily water-balance row schema/content diverges early and materially; this is not a tolerance-only drift.

## Evidence Artifacts

- Comparator JSON: `artifacts/h5_wat_comparator.json`
- Run workspace: `/tmp/sr07_tiera_cmp_20260522`
