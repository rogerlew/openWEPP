# HPHYS0253 Disposition

Status: HOLD

Evidence mode: static + ran

Static:

- HPHYS0253 is diagnostic-only; no production code or canonical contract files
  were edited.

Ran:

- H1 trace run passed and produced day-1 phase storage evidence.
- Full `H1..H39` runtime suite passed `39/39`.
- Full semantic comparator suite completed `39/39`; semantic pass remains
  `0/39`.
- H1 candidate day-1 accounting closes internally: inferred initial from WAT
  terms equals actual post-seed storage at `323.346740 mm`.
- H1 candidate starts `20.153260 mm` below the baseline WAT-derived t=0 proxy
  before day-1 scheduler phases run.
- H1 candidate day-1 loss surplus is `2.072666 mm`, mainly `latqcc
  +1.826096 mm`; `Dp` is near baseline at `+0.011276 mm`.

Disposition:

- `HOLD`.

Continuation:

- Do not start another WB18/WB19/WB17 loss-surface correction from current
  evidence.
- Scaffold HPHYS0254 as a contract-first WB11 initial/runtime storage
  projection authority package.
- The next package should map baseline initial layer `st(i)`/`theta`/`watcon`
  and aggregate `soil_water` semantics into openWEPP seed state, with direct
  H1 t=0 layer and aggregate assertions before production edits.
- Keep WB19 lateral withdrawal as a secondary day-1 contributor, not the
  dominant root cause for the current H1 day-1 storage gap.
