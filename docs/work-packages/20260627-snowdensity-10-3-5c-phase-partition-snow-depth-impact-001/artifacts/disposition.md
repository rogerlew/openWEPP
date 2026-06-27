# Disposition

Evidence mode: Static/Ran.

Status: complete.

Final disposition:
`COMPLETE-10-3-5C-PHASE-PARTITION-SNOW-DEPTH-ADJUDICATED`.

Candidate result:
`PHASE-PARTITION-NEUTRAL-OR-WORSE`.

## Summary

The package ran default `legacy_rst` versus opt-in `harder_pomeroy_hourly`
through the real direct-production WAT path for seven maritime diagnostic
surfaces. The opt-in path changed WAT snow depth on all seven surfaces, but it
worsened all four paired Sleepers/Harvard snow-depth surfaces.

Key result:

- Default paired snow-control failures: `1147 / 1415`.
- Opt-in paired snow-control failures: `1273 / 1415`.
- Delta default-minus-opt-in: `-126`.
- Paired surfaces improved: `0`.
- Paired surfaces worsened: `4`.

## Solver Robustness

The first opt-in WAT run exposed a valid-input hydrometeor solver
non-convergence. The package added a bracketing fallback in
`openwepp-meteorology` that preserves the same Harder-Pomeroy equation and keeps
the behavior opt-in-only. Saturated identity and warm unsaturated fallback cases
are covered by focused tests.

## Boundary Disposition

- Default activation changed: no.
- Parser/runfile/user CLI selector added: no.
- Fixture inputs changed: no.
- Public output schema changed: no.
- Site calibration performed: no.
- Observation-blocked defect verdicts assigned: no.

## Next Route

Target 10.3.4 rank-2 winter-thaw melt response before sub-canopy longwave or
rain-heat changes.
