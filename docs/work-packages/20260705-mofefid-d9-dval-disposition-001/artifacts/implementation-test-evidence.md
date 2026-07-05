# Implementation and Test Evidence

Status: executed
Evidence mode: Static + Ran

## Code / Harness Changes

Static:

- Added `tools/dval/zone_taxonomy.py`, a copyright-safe Figure 9 taxonomy
  harness. It verifies workbook sha256, parses the `Results` sheet, applies the
  published critical `I*` thresholds, fits log-log `Psi*=k I*^l` summaries by
  slope, asserts published `Psi*` support within 10% relative grid tolerance,
  asserts Zone 2 near-linearity, and emits scalar JSON only.
- No `.rs` files were edited.
- No production/default activation files were edited.

## Focused Evidence

Ran:

- Case 1 comparator: PASS, `NS_trace=0.868483`, peak ratio `1.066`.
- Case 2 comparator: PASS, default `NS_trace=0.453954`, peak ratio `0.747`.
- Case 2 `Ks=10` sensitivity: PASS, `NS_trace=0.961209`, peak ratio `0.922`.
- Case 3 comparator: PASS, `NS_trace=0.537727`, peak ratio `0.547`.
- Zone taxonomy: PASS, scalar output in
  `artifacts/zone-taxonomy-20260705-1545.json`.
- Focused Rust D-val test:
  `cargo nextest run -p openwepp-hillslope-orchestrator case2_underprediction_is_ks_operand_limited`
  PASS (`1 passed, 278 skipped`).
