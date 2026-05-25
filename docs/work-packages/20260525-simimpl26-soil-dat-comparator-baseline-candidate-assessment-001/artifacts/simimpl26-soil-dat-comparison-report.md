# SIMIMPL26 Soil.dat Comparison Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Comparison objective: classify baseline-vs-candidate soil-input deltas for
  SIMIMPL Tier-A evidence lanes using reproducible file provenance.
- Canonical authority posture:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- Lane policy references:
  - PL08 comparator provenance (`2026-05-22`)
  - PL14R rerun provenance (`2026-05-23`)

## Ran
### Lane coverage
- Comparable lane pair (baseline + candidate soil files available):
  - PL08: `/tmp/pl08_tiera_cmp_20260522/baseline/runs/p5.sol`
    vs `/tmp/pl08_tiera_cmp_20260522/candidate/runs/p5.sol`
- Supplemental baseline lineage confirmation:
  - PL14R baseline: `/tmp/pl14r_tiera_cmp_20260523/baseline/runs/p5.sol`
- Non-comparable candidate lane:
  - PL14R candidate is output-only (`/tmp/pl14r_tiera_cmp_20260523/candidate/output`);
    no `runs/p5.sol` exists for baseline-vs-candidate soil-file diffing.

### Delta classification
1. Format/version markers
- PL08 baseline/candidate `p5.sol` first token line is `9003` for both.
- PL14R baseline `p5.sol` first token line is also `9003`.

2. Structural deltas
- PL08 baseline vs candidate `p5.sol`:
  - line count: `115` vs `115`
  - byte count: `5136` vs `5136`
  - result: no structural delta.

3. Value deltas
- PL08 baseline vs candidate `p5.sol` sha256:
  - both `259c855e46d9a30176483c23d66b8dda7b7a6f074624587569768c7b2062d4a0`
- byte-identity check (`cmp -s`): identical.

4. Semantic-impact classification
- Comparable lane verdict: no soil-input delta detected.
- PL14R candidate lane verdict: not comparable for soil-input delta because the
  lane contains no candidate `runs/p5.sol` (output-only staging by design).

### Contract/gate alignment
- Required parser contract test gate executed and passed:
  - `cargo test -p openwepp --test infile_soil_parser_contract` (8/8 pass)
- No contract contradiction was observed from SIMIMPL26 soil-file evidence.

## Final assessment
- For available baseline-vs-candidate soil-input evidence (PL08), `p5.sol` is
  identical; no soil-file delta explains previously observed water-balance
  output differences.
- PL14R remains intentionally output-surface-focused for candidate evidence,
  so soil-file baseline-vs-candidate comparison is not applicable in that lane
  without additional candidate run-input staging.

## Follow-on recommendation
- Keep SIMIMPL26 closed as evidence-complete.
- If future queue scope requires PL14R input-identity closure, stage a
  candidate `runs/` input bundle and record sidecar hash parity before any
  soil-input parity claim for that lane.
