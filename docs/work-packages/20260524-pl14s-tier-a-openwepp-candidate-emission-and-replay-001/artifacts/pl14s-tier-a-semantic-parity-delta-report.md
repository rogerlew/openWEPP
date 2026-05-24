# PL14S Tier-A Semantic Parity Delta Report

Status: `completed-with-hold`
Evidence mode: `Static + Ran`

## Static
- Comparator surface: Tier-A hillslope WB13 semantic parity (`H5.wat`).
- Baseline source: `wepp_260430_hill` replay from `delicate_game_pw0` lane (`p5.run`).
- Candidate source: openWEPP-emitted `H5.wat.parquet` from runner fixture lane.
- Erosion/sediment surfaces remain out of scope by PL14S design.

## Ran
- Semantic report artifact:
  - `artifacts/h5_wat_semantic_comparator.json`
- Core comparator outcomes:
  - `semantic_pass`: `false`
  - `common_row_count`: `0`
  - `only_baseline_count`: `1095`
  - `only_candidate_count`: `1`
  - `only_candidate_examples`: `[[1, 1, 2000]]`
  - `only_baseline_examples` starts at `[[1, 1, 1], [1, 1, 2], [1, 1, 3], ...]`
- Column coverage diagnostics:
  - `shared_column_count`: `22`
  - `baseline_only_columns`: `[]`
  - `candidate_only_columns`: `[]`
  - investigation columns used: `P, Q, Ep, Es, Er, Dp, Total-Soil, frozwt, Snow-Water, SoilWaterTotal`
- Divergence bundle behavior:
  - `top_divergent_rows`: `[]` because no common row keys exist.

## Hold Rationale
- PL14S execution succeeded as an investigation lane, but semantic parity did not pass because candidate/baseline row-key sets do not overlap.
- This package therefore provides valid divergence evidence for follow-on closeout, not a parity pass signal.
- Root-cause implementation context for closeout:
  current runner/CLI candidate emission path is still first-day synthesized
  WB13-style output and not full daily watbal scheduler/kernel execution, so
  this replay lane is not yet a watbal-equivalent parity signal.
