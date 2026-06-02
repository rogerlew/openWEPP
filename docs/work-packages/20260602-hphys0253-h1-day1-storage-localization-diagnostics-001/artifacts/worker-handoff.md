# Worker Handoff

Status: complete

Evidence mode: static + ran

Completed:

- HPHYS0253 scaffolded and executed as diagnostic-only.
- Reproducibility helper:
  `artifacts/hphys0253_diagnostics.py`.
- Run root: `/tmp/hphys0253_20260602T203448Z`.
- H1 day-1 localization:
  `artifacts/h1-day1-storage-localization.md`.
- H1 conservation audit:
  `artifacts/h1-day1-conservation-audit.md`.
- Full `H1..H39` metric snapshot:
  `artifacts/full-39-suite-metrics.md`.

Carry forward:

- H1 post-seed storage is the dominant localized gap:
  candidate `323.346740 mm` vs baseline inferred t=0 proxy `343.500000 mm`.
- Candidate layer theta sum at post-seed is only `293.945130 mm`, leaving a
  `29.401610 mm` aggregate-layer alias gap inside the candidate seed state.
- Candidate day-1 accounting closes; do not chase WB13 publication as the
  immediate root cause.
- Day-1 `latqcc` is high by `1.826096 mm`, but that is secondary to the
  pre-scheduler storage deficit.

Recommended next package:

- HPHYS0254: contract-first WB11 initial/runtime storage projection authority
  closure for H1, then H7/H39 and full `H1..H39` metrics.
- Required focus: baseline initial layer `st(i)`/`theta`/`watcon` mapping,
  aggregate `soil_water` seeding, and explicit alias invariants between layer
  storage and WB13 `Total-Soil`/`SoilWaterTotal`.
