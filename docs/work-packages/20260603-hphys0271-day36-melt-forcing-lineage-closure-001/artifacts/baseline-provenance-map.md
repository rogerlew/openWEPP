# Baseline Provenance Map

Status: completed
Evidence mode: static

Static:

- `/workdir/wepp-forest_260430_baseline/src/melt.for` is the pinned baseline authority for `amelt`, `bmelt`, `cmelt`, `dmelt`, and final `wmelt = 0.0254 * (amelt + bmelt + cmelt + dmelt)`.
- `melt.for` consumes hourly temperature, radiation, cloud fraction, dewpoint, wind speed, rainfall, canopy cover, snow depth, and density.
- Corrected negative-melt authority from `/workdir/wepp-forest@03fee4558456535138592630b5dedc4d81ce8d06` remains retained; HPHYS0271 did not reproduce the pinned baseline negative-melt bug.
- HPHYS0270 review localized the first material H1 seam to sim-day 36: baseline reports `RM=0` while openWEPP reports a same-day redistributed melt release.

Ran: not-run; this artifact records static authority mapping only.
