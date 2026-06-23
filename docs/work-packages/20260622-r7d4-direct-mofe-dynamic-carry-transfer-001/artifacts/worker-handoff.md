# Worker Handoff

Status: executed-held.

## Handoff

- R7D4 hold:
  `HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT`.
- First actionable next item: close
  `HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT` by implementing
  producer-authoritative direct EROD14/EROD15 sediment publication operands
  for HBP/PASS, not by reading compatibility WB13 rows, compatibility public
  output builders, or stale runtime aliases.
- Follow-up package:
  `docs/work-packages/20260623-r7d5-direct-erod14-sediment-publication-001/package.md`.
- Current clean evidence:
  `/tmp/r7d4-h2637-5day/default/H2637.wat.parquet` and
  `/tmp/r7d4-h2637-5day/direct/H2637.wat.parquet` are byte-identical;
  `/tmp/r7d4-h2637-5day/default/H2637.pass.parquet` and
  `/tmp/r7d4-h2637-5day/direct/H2637.pass.parquet` are byte-identical.
- HBP residual evidence: both HBP files are `5254` bytes; `34` bytes differ.
  Payload offsets `928`, `936`, and `944` are nonzero in default and zero in
  direct, matching sediment concentration plus total detachment/deposition
  payload authority. Tail bytes differ as checksum consequence.
- Preserve the R7D4 hydrology fixes. Do not reintroduce R7D trace
  instrumentation or disable the no-carry frost fine-state purge; that purge is
  what closed the final WAT/PASS ULP drift.
