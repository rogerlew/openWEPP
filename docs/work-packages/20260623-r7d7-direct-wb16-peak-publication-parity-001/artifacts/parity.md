# Parity Evidence

Status: executed-held.

## H2637

- Ran: fresh H2637 5-day compatibility output:
  `/tmp/r7d7-h2637-5day/default/` exited `0`.
- Ran: fresh H2637 5-day direct output:
  `/tmp/r7d7-h2637-5day/direct/` exited `0`.
- WAT byte identity holds:
  `/tmp/r7d7-h2637-5day/default/H2637.wat.parquet` equals
  `/tmp/r7d7-h2637-5day/direct/H2637.wat.parquet`.
- PASS byte identity now holds:
  `/tmp/r7d7-h2637-5day/default/H2637.pass.parquet` equals
  `/tmp/r7d7-h2637-5day/direct/H2637.pass.parquet`.
- PASS SHA-256 on both paths:
  `46d5ec947fe351c927c15dafd34d3487c036ac1bb3c1fc2848e3d22747003e89`.
- Direct runtime counters remain direct-only:
  `compatibility_edge_invocations = 0`.
- HBP still differs bytewise:
  compatibility SHA-256
  `73f8c9c8f139c48599d5818f785b2e6368fdacb2147ffa0bba661e3f2c9f3239`,
  direct SHA-256
  `2f43a2aa9389a8fd8ed7d6242fa29d0a402fa52640666eb5652a47993b1ef2cd`.

## Residuals

- `HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP`: HBP
  peak/duration are now parity-clean, but HBP sediment export aliases are not.
- Parsed HBP payload residual:
  compatibility `total_detachment_kg = 0.6`, direct `0.0`;
  compatibility `total_deposition_kg = 0.0`, direct `0.0`;
  compatibility `sediment_concentration_kg_m3 = 6.816136920064195`,
  direct `0.0`; particle flow fraction `1.0` on both.
- PASS sediment remains byte-identical and all-zero on both paths. The HBP
  residual therefore belongs to EROD15 HBP export alias authority, not PASS
  row serialization.
- Existing compatibility warning still identifies related follow-on scope:
  `MOFE01-MG-W-001 EROD14 Wave-2 qin is seeded from water-transfer provenance
  only; true sediment-coupled qin/qout and particle-fraction handoff remains
  MOFE01 M-G follow-on scope.`
