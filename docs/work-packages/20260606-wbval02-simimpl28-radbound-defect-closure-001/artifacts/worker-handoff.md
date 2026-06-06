# Worker Handoff

Status: complete

Evidence mode: `Static`

Static:

WBVAL02 itself has no remaining in-package work. The six observed
`CLIM-RUNTIME-E-017` blockers are closed by typed invalid-upstream evidence.

Follow-on defect target:

1. Close defect `WBVAL04-CLIMATE-RADLY-RAMAX-INPUT-BOUNDARY` end-to-end.

Required fields:

- Defect ID: `WBVAL04-CLIMATE-RADLY-RAMAX-INPUT-BOUNDARY`
- Observable failure: DRIGGS CLIGEN daily climate file used by WBVAL01/WBVAL02
  contains source `radly` rows above baseline `sunmap` horizontal daily
  potential. First observed row is `1990-02-18 radly=486 Ly d^-1`,
  `r3=453.068716 Ly d^-1`.
- Suspected mechanism: upstream climate-generation or climate-ingestion boundary
  accepts solar radiation above the runtime `SC-CLIMATE-001`/baseline
  horizontal potential bound.
- In-scope authority/write-set: climate input contract, climate parser or run
  ingestion boundary, and any wepppy/openWEPP orchestration contract that
  governs external CLIGEN payload validity.
- Failing fixture/evidence:
  `/wc1/runs/in/indispensable-presenter/wepp/runs/p2.cli` through
  `/wc1/runs/in/indispensable-presenter/wepp/runs/p17.cli`, all sharing the
  same DRIGGS climate source.
- Correction authority: `SC-CLIMATE-001#INV-CLIMATE-006`,
  `SC-CLIMATE-001#INV-CLIMATE-013`, and pinned baseline `sunmap.for`
  horizontal potential lineage.
- Acceptance target: invalid source radiation is rejected or quarantined before
  SIMIMPL28 hourly synthesis, or the upstream climate contract is amended with
  authoritative proof that a different `RAmax` boundary is correct.
- HOLD-legitimacy conditions: only legitimate if the responsible input
  boundary is outside the active repository authority, or if CLIGEN/upstream
  contract authority contradicts the current `SC-CLIMATE-001` daily source
  bound.

Forbidden relay satisfied:

- This handoff names a defect, evidence, suspected mechanism, authority,
  write-set, acceptance target, and boundary conditions. It is not a request to
  "trace the next radiation variable."
