# Worker Handoff

Status: executed-held.

## Handoff

- Start with R7D8:
  `20260623-r7d8-direct-hbp-erod15-export-alias-parity-001`.
- Do not re-open WB16 peak publication unless new evidence contradicts R7D7.
  Fresh H2637 5-day evidence has PASS byte identity and HBP peak/duration
  parity.
- First action: trace compatibility HBP aliases
  `total_detachment_kg` and `sediment_concentration_kg_m3_0001` from the
  compatibility runtime surface through EROD15/MOFE Wave-2 publication, then
  map the equivalent direct producer source without compatibility runtime
  wrapping.
- Required initial reproduction:
  run the H2637 5-day fixture under `/tmp/r7d4-h2637-5day/run` into a fresh
  `/tmp/r7d8-h2637-5day` output directory, compare HBP, and parse the HBP
  payload fields. Expected starting residual is compatibility
  `total_detachment_kg = 0.6` and
  `sediment_concentration_kg_m3 = 6.816136920064195` versus direct zeros.
- Do not resolve the mismatch by:
  forcing direct HBP sediment aliases to zero, copying compatibility
  `HillslopeWritebackSurface` values into direct publication, or using PASS
  all-zero sediment rows as HBP authority.
- Closure requires HBP byte identity, WAT/PASS byte identity preservation, and
  direct `compatibility_edge_invocations = 0`.
