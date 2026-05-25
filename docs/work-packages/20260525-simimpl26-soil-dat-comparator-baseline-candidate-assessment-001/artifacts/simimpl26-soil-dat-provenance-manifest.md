# SIMIMPL26 Soil.dat Provenance Manifest

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Baseline authority remains ADR-0012 pinned worktree:
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Upstream lane provenance references used:
  - `docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-run-provenance-manifest.md`
  - `docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/pl14r-comparator-run-provenance-manifest.md`
- Lane selection frozen for SIMIMPL26 comparison:
  - PL08 replay lane root: `/tmp/pl08_tiera_cmp_20260522`
  - PL14R rerun lane root: `/tmp/pl14r_tiera_cmp_20260523`

## Ran
Commands executed for reproducible soil-file evidence:
1. file existence + digest + size + header markers
   - `sha256sum`, `wc -lc`, `sed -n '1,2p'` on:
     - `/tmp/pl08_tiera_cmp_20260522/baseline/runs/p5.sol`
     - `/tmp/pl08_tiera_cmp_20260522/candidate/runs/p5.sol`
     - `/tmp/pl14r_tiera_cmp_20260523/baseline/runs/p5.sol`
     - `/tmp/pl14r_tiera_cmp_20260523/candidate/runs/p5.sol`
2. byte-identical checks
   - `cmp -s` for:
     - PL08 baseline vs PL08 candidate `p5.sol`
     - PL08 baseline vs PL14R baseline `p5.sol`
3. soil file discovery scope check
   - `find <lane-root> -type f \( -name 'soil.dat' -o -name '*.sol' \)`

Observed provenance snapshot:
- `/tmp/pl08_tiera_cmp_20260522/baseline/runs/p5.sol`
  - exists: yes
  - sha256: `259c855e46d9a30176483c23d66b8dda7b7a6f074624587569768c7b2062d4a0`
  - lines/bytes: `115` / `5136`
  - first token line (`datver` marker): `9003`
- `/tmp/pl08_tiera_cmp_20260522/candidate/runs/p5.sol`
  - exists: yes
  - sha256: `259c855e46d9a30176483c23d66b8dda7b7a6f074624587569768c7b2062d4a0`
  - lines/bytes: `115` / `5136`
  - first token line (`datver` marker): `9003`
- `/tmp/pl14r_tiera_cmp_20260523/baseline/runs/p5.sol`
  - exists: yes
  - sha256: `259c855e46d9a30176483c23d66b8dda7b7a6f074624587569768c7b2062d4a0`
  - lines/bytes: `115` / `5136`
  - first token line (`datver` marker): `9003`
- `/tmp/pl14r_tiera_cmp_20260523/candidate/runs/p5.sol`
  - exists: no (candidate lane is output-only in PL14R provenance)

Comparator identity checks:
- `cmp`: PL08 baseline `p5.sol` vs PL08 candidate `p5.sol` -> `IDENTICAL`
- `cmp`: PL08 baseline `p5.sol` vs PL14R baseline `p5.sol` -> `IDENTICAL`

Coverage note:
- No `soil.dat` file was present in either selected lane root; canonical soil
  input evidence in these lanes is `.sol` (`p5.sol`).
