# Pre-Implementation Evidence

Evidence mode: Static + Ran.

Starting point:

- SNOWFROST-FIDELITY-E routed Sites 1, 2, and 4 to snow-depth fidelity issues,
  with dominant modeled-over-observed snow depth.
- Sites 3 and 5 lack paired observed snow-depth rows, so they cannot adjudicate
  snow-depth control against observations.
- Legacy WAT `Snow-Water` is SWE only. It cannot answer the physical snow-depth
  question.

Legacy output-lineage findings:

- Static: `outfil.for` exposes a `daily winter` output prompt and opens unit 42.
- Static: `winter.for` writes `tmpvr7=snodpt(iplane)*1000.0` and `densgt` with
  Julian day, hour, and calendar year.
- Static: `bigout.for` writes sparse large-graphics operands
  `treal(73)=snodpy(iplane)*1000.0` and `treal(75)=densg(iplane)`.
- Ran: a throwaway Site 1 replay showed large graphics is sparse for the
  hillslope fixtures (`9799` rows versus `13880` WAT days), so it is retained
  as operand provenance and not used as the date-aligned comparator feed.
- Ran: enabling daily-winter output produced dated hourly snow-depth rows; the
  package uses hour 24 as the daily physical snow-depth comparator value.
