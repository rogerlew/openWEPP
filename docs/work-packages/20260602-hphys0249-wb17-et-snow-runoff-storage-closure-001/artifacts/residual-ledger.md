# Residual Ledger

Status: complete

Evidence mode: static + ran

Static:

- HPHYS0249 was intentionally scoped to WB17 `Ep`/`Es`, snow/runoff timing,
  and aggregate storage diagnosis/correction.

Ran:

- Runtime/comparator completed `39/39`; semantic parity remains `0/39`.
- Corrected/near-closed focus:
  - `Es` improved from mean abs mean `3.340827` to `0.036841`.
- Remaining priority:
  1. `Ep`: unchanged all-hillslope failure; likely root-depth/growth activation
     lineage. Verify comparator ingestion of post-WB19 `PlantRootUptake` `Ep`
     output before the next WB17 correction.
  2. Snow/runoff timing: `Snow-Water`, `RM`, `Q` unchanged.
  3. Aggregate storage: worsened after correct `Es`; reassess only after `Ep`
     and snow/runoff timing closure.
  4. WB19 `latqcc`: still all-hillslope failing but not materially affected by
     HPHYS0249.
  5. `pltol`: crop-specific projection remains open; runtime currently uses
     baseline default `0.25`.

Disposition implication:

- Package remains `HOLD`; corrected WB17 `Es` can be retained as
  process-authoritative, but H39/full-suite water-balance closure is not
  complete.
