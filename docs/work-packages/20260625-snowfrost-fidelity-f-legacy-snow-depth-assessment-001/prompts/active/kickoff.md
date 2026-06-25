# SNOWFROST-FIDELITY-F Kickoff

Execute `docs/work-packages/20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/package.md`.

Required posture:

- Treat pinned legacy WEPP as diagnostic flag evidence only, not the openWEPP
  correctness target.
- Capture legacy physical snow depth from dated daily-winter rows; use large
  graphics `treal(73)` only as sparse operand provenance and do not use WAT
  `Snow-Water` as depth.
- Compare current openWEPP `Snow-Depth` and `Snow-Water` to legacy physical
  snow depth/SWE on common model dates.
- Compare both model snow depths to observed physical snow depth only where
  paired observed snow-depth rows exist.
- Do not change production physics, constants, runtime activation, or
  observation tolerances.
