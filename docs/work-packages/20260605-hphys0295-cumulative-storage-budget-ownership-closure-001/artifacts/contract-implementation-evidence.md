# Contract Implementation Evidence

Status: executed
Evidence mode: Static

Static:
- Amended `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  with `INV-WATBAL-070`, guard-map coverage, and `OBL-WATBAL-P-019`.
- Amended `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
  with `INV-EVAP-027`, guard-map coverage, and `OBL-EVAP-P-009`.
- Recorded revision-history entries:
  - `SC-WATBAL-001` version `115`, dated `2026-06-05`.
  - `SC-EVAP-001` version `26`, dated `2026-06-05`.

Interpretation:
- The contracts now require cumulative row-to-row storage-budget evidence
  before assigning H1/H7/H39 residual ownership to WB17, WB18, WB19, or WB13.
- Same-day comparator deltas remain insufficient production-change authority
  when ET, percolation, lateral flow, and excluded snow/`RM` masks have not
  been separated.
