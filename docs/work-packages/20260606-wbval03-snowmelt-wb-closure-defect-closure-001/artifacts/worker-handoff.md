# Worker Handoff

Status: complete

Evidence mode: `Static`

Static:

First required follow-on:

1. Close defect `WBVAL04-CLIMATE-RADLY-RAMAX-INPUT-BOUNDARY` end-to-end.

Defect fields:

- Defect ID: `WBVAL04-CLIMATE-RADLY-RAMAX-INPUT-BOUNDARY`
- Observable failure: all current WBVAL03 target runs fail before J-95
  percolation or WAT publication with `CLIM-RUNTIME-E-017`, source symbol
  `radly=486`.
- Suspected mechanism: shared DRIGGS CLIGEN daily climate source contains
  daily radiation above baseline `sunmap` horizontal daily potential.
- In-scope authority/write-set: climate input contract, climate ingestion, and
  upstream CLIGEN/run-input boundary.
- Failing fixture/evidence:
  `/wc1/runs/in/indispensable-presenter/wepp/runs/p*.cli`; first violating row
  from WBVAL02 is `1990-02-18 radly=486 Ly d^-1`.
- Correction authority: `SC-CLIMATE-001#INV-CLIMATE-006`,
  `SC-CLIMATE-001#INV-CLIMATE-013`, and pinned baseline `sunmap.for`.
- Acceptance target: WBVAL03 target runs can reach the J-95/WAT surfaces, or
  the upstream climate source is conclusively reclassified and quarantined with
  typed evidence.
- HOLD-legitimacy conditions: only legitimate if climate source authority is
  missing/contradictory or the responsible input boundary is outside the active
  repository authority.

Second follow-on after WBVAL04:

2. Resume defect `WBVAL03-HKERNEL-WB11-PERC-E-003-J95` and
   `WBVAL03-WAT-LEDGER-CONSERVATION-RESIDUAL` end-to-end.

Resume acceptance:

- Re-run `p7`, `p11`, `p18`, and `p20` through J-95.
- Re-run the 12 WAT-emitting hillslopes through WAT publication.
- Use `complete-balance-identity-audit.md` as the starting identity; do not
  treat omitted `UpStrmQ`, `SubRIn`, `Tile`, `InterceptionStorage`, or `frozwt`
  terms as unexplored breadcrumbs.

Forbidden relay satisfied:

- This handoff names defect IDs, observable failures, suspected mechanisms,
  authority/write-sets, evidence, acceptance targets, and HOLD conditions. It
  does not ask the next worker to inspect the next helper function.
