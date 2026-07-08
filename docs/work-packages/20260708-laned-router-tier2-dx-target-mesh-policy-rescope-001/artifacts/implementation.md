# Implementation

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY
Evidence mode: Static + Ran.

No production mesh-policy flip landed. The active production default remains
fixed `10 cells/OFE`.

Implemented diagnostic surfaces:
- `DirectLanedActiveMeshPolicy` remains production-default fixed cells, with a
  bounded diagnostic target-`dx` selector:
  `raw_cells = ceil(slplen_m / target_dx_m)`, fail closed above `4096`, and
  `cells_per_ofe = max(raw_cells, 10)`.
- Runner selector: `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M`.
- Diagnostic trace selector: `OPENWEPP_LANED_ACTIVE_TRACE=1`.
- Trace output: manifest-checksummed `laned_active_trace.jsonl`, one row per
  active lane-day, including source/outlet/storage/tail terms, terminal outlet
  on terminal-lane rows, and the 24 routed hourly weights consumed by D13.
- Manifest active provenance now includes mesh policy metadata and trace row
  count when the trace selector is active.
- Trace-only misuse (`OPENWEPP_LANED_ACTIVE_TRACE=1` without
  `OPENWEPP_LANED_ACTIVE=1`) now fails at startup before output setup.

Production-isolation properties:
- Selector absent: fixed `10 cells/OFE`.
- Trace selector absent: no trace file and no trace rows retained.
- Invalid target `dx` or impossible derived cell count fails closed.
- Missing trace rows under `OPENWEPP_LANED_ACTIVE_TRACE=1` fails closed.

Adjudication result:
- Target-`dx` production promotion is held/rejected by package criteria.
- `mn_corn_h4` and `n_idaho_forest_h1` passed the declared comparison
  tolerances, but `wa_cascades_forest_h1` failed the fine-reference rungs at
  active closure day 1122 and showed non-promotable behavior at `dx10/dx5`.
- H2637 remains synthetic stress evidence and failed shape/sediment adequacy.
