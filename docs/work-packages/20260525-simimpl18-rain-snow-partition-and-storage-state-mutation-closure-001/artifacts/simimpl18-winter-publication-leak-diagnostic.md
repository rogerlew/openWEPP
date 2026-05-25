# simimpl18-winter-publication-leak-diagnostic

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Diagnostic target: verify whether published hydout-equivalent `Snow-Water`
  follows runtime SWE state or mirrors static snow sidecar controls.

## Ran
- Evidence input:
  - `artifacts/replay-run-20260525T132822Z/candidate/openwepp_hillslope_run_manifest.json`
- Candidate provenance values:
  - `coupling_vectors.winter.ssd = 250.0`
  - `coupling_vectors.winter.runtime_swe = 0.0`
  - `coupling_vectors.hydout_equivalent.snow_water = 250.0`
- Corresponding emitted WB13 row values in
  `artifacts/replay-run-20260525T132822Z/candidate/H5.hbp` are consistent with
  static publication (`Snow-Water=250.00` across span).

## Interpretation
- Publication leak signal remains open: dynamic runtime SWE is not reflected in
  emitted `Snow-Water` for this fixture.
